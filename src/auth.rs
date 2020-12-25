use crate::prelude::*;

use jwt::decode as decode_token;
use jwt::Algorithm as JwtAlgorithm;
use jwt::Validation as TokenValidation;
use jwt::{decode_header, DecodingKey, Header, TokenData};

use cache_control::CacheControl;
use http::header::CACHE_CONTROL;
use request::Client;
use tokio::sync::Mutex;

lazy_static! {
    static ref CLOCK_LEEWAY: Duration = Duration::seconds(30);
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Claims {
    pub exp: u64,
    pub iat: u64,
    pub aud: String,
    pub iss: String,
    pub sub: String,
    pub user_id: String,
}

#[derive(Debug)]
pub struct AuthInfo(TokenData<Claims>);

impl AuthInfo {
    pub fn new(data: TokenData<Claims>) -> Self {
        Self(data)
    }

    pub fn header(&self) -> &Header {
        &self.0.header
    }

    pub fn claims(&self) -> &Claims {
        &self.0.claims
    }
}

impl From<TokenData<Claims>> for AuthInfo {
    fn from(data: TokenData<Claims>) -> Self {
        Self::new(data)
    }
}

#[async_trait]
pub trait Verifier: Sync + Send {
    async fn decode_token(&self, token: &str) -> Result<AuthInfo>;
}

const FIREBASE_KEY_URL: &str = "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";
const FIREBASE_ISS_URL: &str = "https://securetoken.google.com";

pub struct FirebaseVerifier {
    client: Mutex<FirebaseClient>,
    project_id: String,
}

impl FirebaseVerifier {
    pub fn new(project_id: &str) -> Self {
        let client = FirebaseClient::new(project_id.to_owned());
        Self {
            client: Mutex::new(client),
            project_id: project_id.to_owned(),
        }
    }
}

impl FirebaseVerifier {
    fn expected_iss(&self) -> String {
        format!("{}/{}", FIREBASE_ISS_URL, &self.project_id)
    }

    fn expected_aud(&self) -> String {
        self.project_id.to_owned()
    }
}

#[async_trait]
impl Verifier for FirebaseVerifier {
    async fn decode_token(&self, token: &str) -> Result<AuthInfo> {
        let mut validation = TokenValidation::new(JwtAlgorithm::RS256);
        validation.leeway = CLOCK_LEEWAY.num_seconds().try_into().unwrap();
        validation.iss = Some(self.expected_iss());
        validation.set_audience(&[self.expected_aud()]);

        let header = decode_header(token).context("failed to decode header")?;
        let kid = header.kid.context("missing key ID")?;

        let keys = {
            let mut client = self.client.lock().await;
            client
                .keys()
                .await
                .context("failed to load decoding keys")?
        };
        let key = keys.get(&kid).context("no matching decoding keys")?;
        let data = decode_token::<Claims>(token, key, &validation)?;

        let iat = Utc.timestamp(
            data.claims
                .iat
                .try_into()
                .context("failed to convert issued-at time to u64")?,
            0,
        );
        if iat > (Utc::now() + *CLOCK_LEEWAY) {
            bail!("invalid issued-at time");
        }

        Ok(data.into())
    }
}

#[derive(Debug, Clone)]
struct FirebaseClient {
    client: Client,
    project_id: String,
    refresh_at: DateTime,
    keys: Map<String, DecodingKey<'static>>,
}

impl FirebaseClient {
    pub fn new(project_id: String) -> Self {
        Self {
            client: Client::new(),
            project_id,
            refresh_at: Utc::now(),
            keys: Default::default(),
        }
    }

    pub async fn keys(&mut self) -> Result<Map<String, DecodingKey<'static>>> {
        self.sync().await?;
        Ok(self.keys.clone())
    }
}

impl FirebaseClient {
    async fn sync(&mut self) -> Result<()> {
        let refresh_at = self.refresh_at - Duration::minutes(1);
        if Utc::now() <= refresh_at {
            return Ok(());
        }
        let response = self.client.get(FIREBASE_KEY_URL).send().await?;

        let cache_control = response
            .headers()
            .get(CACHE_CONTROL)
            .context("missing cache-control header")?;
        let cache_control = cache_control
            .to_str()
            .context("failed to parse cache-control header")?;
        let cache_control = CacheControl::from_value(cache_control)
            .context("missing cache-control directives")?;

        let max_age = cache_control.max_age.context("missing max-age")?;
        self.refresh_at = Utc::now() + max_age;

        let data: JwkData =
            response.json().await.context("failed to parse response")?;
        let mut keys = Map::<String, DecodingKey<'static>>::new();
        for jwk in data.keys {
            let JwkInfo { kid, n, e } = jwk;
            let key = DecodingKey::from_rsa_components(&n, &e).into_static();
            keys.insert(kid, key);
        }
        self.keys = keys;
        Ok(())
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
struct JwkData {
    keys: Vec<JwkInfo>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
struct JwkInfo {
    kid: String,
    n: String,
    e: String,
}
