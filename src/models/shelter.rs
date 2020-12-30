use super::prelude::*;

use service::Shelter as ShelterRepr;
use service::ShelterSpace;

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
    pub total_spots: i32,
    pub total_beds: i32,
    pub food: String,
    pub tags: Vec<String>,
    pub image_url: Option<String>,
    pub occupied_spots: Option<i32>,
    pub occupied_beds: Option<i32>,
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

            capacity,
            occupancy,
            food,
            tags,
        } = shelter;

        let address =
            to_json_value(address).context("failed to encode address")?;
        let location =
            to_json_value(location).context("failed to encode location")?;

        let ShelterSpace {
            spots: total_spots,
            beds: total_beds,
        } = capacity;

        let (occupancy_spots, occupancy_beds) = match occupancy {
            Some(ShelterSpace { beds, spots }) => (Some(beds), Some(spots)),
            None => (None, None),
        };
        let occupied_spots = occupancy_spots
            .map(TryInto::try_into)
            .transpose()
            .context("failed to convert occupied spots count")?;
        let occupied_beds = occupancy_beds
            .map(TryInto::try_into)
            .transpose()
            .context("failed to convert occupied beds count")?;

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
            total_spots: total_spots.into(),
            total_beds: total_beds.into(),
            food: food.to_string(),
            tags: tags.into_iter().map(|tag| tag.to_string()).collect(),
            occupied_spots,
            occupied_beds,
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
            total_spots,
            total_beds,
            food,
            tags,
            image_url,
            occupied_spots,
            occupied_beds,
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

        let total_spots = total_spots
            .try_into()
            .context("failed to convert total spots count")?;
        let total_beds = total_beds
            .try_into()
            .context("failed to convert total beds count")?;
        let capacity = ShelterSpace {
            spots: total_spots,
            beds: total_beds,
        };

        let occupied_spots = occupied_spots
            .map(TryInto::try_into)
            .transpose()
            .context("failed to convert occupied spots count")?;
        let occupied_beds = occupied_beds
            .map(TryInto::try_into)
            .transpose()
            .context("failed to convert occupied beds count")?;
        let occupancy = match (occupied_spots, occupied_beds) {
            (Some(spots), Some(beds)) => Some(ShelterSpace { spots, beds }),
            _ => None,
        };

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

            capacity,
            occupancy,
            food,
            tags,
        };

        Ok(shelter)
    }
}
