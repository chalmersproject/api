use super::prelude::*;

use service::Shelter as ShelterRepr;

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset,
)]
#[table_name = "shelters"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Shelter {
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub slug: String,
    pub name: String,
    pub about: Option<String>,
    pub email: Option<String>,
    pub phone: String,
    pub website_url: Option<String>,
    pub address: JsonValue,
    pub location: JsonValue,
    pub spots: i32,
    pub beds: i32,
    pub food: String,
    pub tags: Vec<String>,
    pub image_url: Option<String>,
}

impl TryFrom<ShelterRepr> for Shelter {
    type Error = Error;
    fn try_from(shelter: ShelterRepr) -> Result<Self, Self::Error> {
        let ShelterRepr {
            id,
            created_at,
            updated_at,
            slug,
            name,
            about,
            image_url,
            email,
            phone,
            website_url,
            address,
            location,
            spots,
            beds,
            food,
            tags,
        } = shelter;

        let address =
            to_json_value(address).context("failed to encode address")?;
        let location =
            to_json_value(location).context("failed to encode location")?;

        let shelter = Self {
            id,
            created_at,
            updated_at,
            slug: slug.into(),
            name,
            about,
            image_url: image_url.map(|url| url.to_string()),
            email: email.map(Into::into),
            phone: phone.into(),
            website_url: website_url.map(|url| url.to_string()),
            address,
            location,
            spots: spots.into(),
            beds: beds.into(),
            food: food.to_string(),
            tags: tags.into_iter().map(|tag| tag.to_string()).collect(),
        };

        Ok(shelter)
    }
}

impl TryFrom<Shelter> for ShelterRepr {
    type Error = Error;

    fn try_from(shelter: Shelter) -> Result<Self, Self::Error> {
        let Shelter {
            id,
            created_at,
            updated_at,
            slug,
            name,
            about,
            email,
            phone,
            website_url,
            address,
            location,
            spots,
            beds,
            food,
            tags,
            image_url,
        } = shelter;

        let slug = slug.try_into().context("failed to parse slug")?;
        let image_url = image_url
            .map(|url| url.parse())
            .transpose()
            .context("failed to parse image URL")?;

        let email = email
            .map(TryInto::try_into)
            .transpose()
            .context("failed to parse email address")?;
        let phone = phone.try_into().context("failed to parse phone number")?;

        let website_url = website_url
            .map(|url| url.parse())
            .transpose()
            .context("failed to parse website URL")?;
        let address =
            from_json_value(address).context("failed to decode address")?;
        let location =
            from_json_value(location).context("failed to decode location")?;

        let spots = spots.try_into().context("failed to convert spot count")?;
        let beds = beds.try_into().context("failed to convert bed count")?;
        let food = food.parse().context("failed to parse food options")?;
        let tags = tags
            .into_iter()
            .map(|tag| tag.parse())
            .collect::<Result<_, SerdePlainError>>()
            .context("failed to parse tags")?;

        let shelter = ShelterRepr {
            id,
            created_at,
            updated_at,
            slug,
            name,
            about,
            image_url,
            email,
            phone,
            website_url,
            address,
            location,
            spots,
            beds,
            food,
            tags,
        };

        Ok(shelter)
    }
}
