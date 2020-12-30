use super::prelude::*;

use service::Email;

use service::Shelter as ShelterRepr;
use service::ShelterFood as ShelterFoodRepr;
use service::ShelterSpace as ShelterSpaceRepr;
use service::ShelterTag as ShelterTagRepr;

use service::CreateShelterRequest;
use service::DeleteShelterRequest;
use service::GetShelterRequest;
use service::ListSheltersRequest;
use service::UpdateShelterRequest;

#[derive(Debug, Clone)]
pub struct Shelter(ShelterRepr);

impl From<ShelterRepr> for Shelter {
    fn from(shelter: ShelterRepr) -> Self {
        Self(shelter)
    }
}

/// A `Shelter` is a temporary residence for people without a home.
#[Object]
impl Shelter {
    async fn id(&self) -> Id {
        Id::new::<Self>(self.0.id)
    }

    async fn slug(&self) -> &String {
        self.0.slug.as_string()
    }

    async fn name(&self) -> &String {
        &self.0.name
    }

    async fn about(&self) -> Option<&String> {
        self.0.about.as_ref()
    }

    async fn image_url(&self) -> Option<String> {
        let url = self.0.image_url.as_ref();
        url.map(ToString::to_string)
    }

    async fn email(&self) -> Option<&String> {
        let email = self.0.email.as_ref();
        email.map(Email::as_string)
    }

    async fn phone(&self) -> &String {
        self.0.phone.as_string()
    }

    async fn website_url(&self) -> Option<String> {
        let url = self.0.website_url.as_ref();
        url.map(ToString::to_string)
    }

    async fn address(&self) -> Address {
        let address = self.0.address.to_owned();
        address.into()
    }

    async fn location(&self) -> Coordinate {
        self.0.location.into()
    }

    async fn capacity(&self) -> ShelterSpace {
        let capacity = self.0.capacity.to_owned();
        capacity.into()
    }

    async fn occupancy(&self) -> ShelterSpace {
        let occupancy = self.0.capacity.to_owned();
        occupancy.into()
    }

    async fn food(&self) -> ShelterFood {
        self.0.food.into()
    }

    async fn tags(&self) -> Vec<ShelterTag> {
        let tags = self.0.tags.to_owned();
        tags.into_iter().map(Into::into).collect()
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Enum)]
pub enum ShelterFood {
    Meals,
    Snacks,
    None,
}

impl From<ShelterFood> for ShelterFoodRepr {
    fn from(role: ShelterFood) -> Self {
        use ShelterFood::*;
        use ShelterFoodRepr as Repr;
        match role {
            Meals => Repr::Meals,
            Snacks => Repr::Snacks,
            None => Repr::None,
        }
    }
}

impl From<ShelterFoodRepr> for ShelterFood {
    fn from(role: ShelterFoodRepr) -> Self {
        use ShelterFood::*;
        use ShelterFoodRepr as Repr;
        match role {
            Repr::Meals => Meals,
            Repr::Snacks => Snacks,
            Repr::None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Enum)]
pub enum ShelterTag {
    Adult,
    Youth,
    Family,
    Male,
    Female,
    LGBTQ,
    Pets,
}

impl From<ShelterTag> for ShelterTagRepr {
    fn from(role: ShelterTag) -> Self {
        use ShelterTag::*;
        use ShelterTagRepr as Repr;
        match role {
            Adult => Repr::Adult,
            Youth => Repr::Youth,
            Family => Repr::Family,
            Male => Repr::Male,
            Female => Repr::Female,
            LGBTQ => Repr::LGBTQ,
            Pets => Repr::Pets,
        }
    }
}

impl From<ShelterTagRepr> for ShelterTag {
    fn from(role: ShelterTagRepr) -> Self {
        use ShelterTag::*;
        use ShelterTagRepr as Repr;
        match role {
            Repr::Adult => Adult,
            Repr::Youth => Youth,
            Repr::Family => Family,
            Repr::Male => Male,
            Repr::Female => Female,
            Repr::LGBTQ => LGBTQ,
            Repr::Pets => Pets,
        }
    }
}

#[derive(Debug, Clone, Hash, SimpleObject)]
pub struct ShelterSpace {
    pub spots: u16,
    pub beds: u16,
}

impl From<ShelterSpaceRepr> for ShelterSpace {
    fn from(space: ShelterSpaceRepr) -> Self {
        let ShelterSpaceRepr { beds, spots } = space;
        Self { beds, spots }
    }
}

#[derive(Debug, Clone, Hash, InputObject)]
pub struct ShelterSpaceInput {
    pub spots: u16,
    pub beds: u16,
}

impl From<ShelterSpaceInput> for ShelterSpaceRepr {
    fn from(space: ShelterSpaceInput) -> Self {
        let ShelterSpaceInput { beds, spots } = space;
        Self { beds, spots }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct ShelterQueries;

#[Object]
impl ShelterQueries {
    /// Get a `Shelter` by its `ID`.
    async fn shelter(
        &self,
        ctx: &Context<'_>,

        #[rustfmt::skip]
        #[graphql(desc = "The `ID` of the `Shelter` to fetch.")]
        id: Id,
    ) -> FieldResult<Option<Shelter>> {
        let service = get_service(ctx);

        // Request shelter from service.
        let shelter = {
            let request = {
                let shelter_id =
                    id.get::<Shelter>().context("invalid shelter ID")?;
                GetShelterRequest { shelter_id }
            };
            let response =
                service.get_shelter(request).await.into_field_result()?;
            response.shelter
        };

        // Return shelter object.
        Ok(shelter.map(Into::into))
    }

    /// List all registered `Shelter`s.
    async fn shelters(
        &self,
        ctx: &Context<'_>,

        // TODO: Use `default` instead of `default_with` once
        // https://github.com/async-graphql/async-graphql/issues/361
        // is resolved.
        #[rustfmt::skip]
        #[graphql(
            desc = "The maximum number of `Shelter`s to fetch.",
            default_with = "25"
        )]
        limit: u32,

        #[rustfmt::skip]
        #[graphql(desc = "The number of initial `Shelter`s to skip.", default)]
        offset: u32,
    ) -> FieldResult<Vec<Shelter>> {
        let service = get_service(ctx);

        // Request shelters from service.
        let shelters = {
            let request = ListSheltersRequest { limit, offset };
            let response =
                service.list_shelters(request).await.into_field_result()?;
            response.shelters
        };

        // Return shelter object.
        let shelters = shelters.into_iter().map(Into::into).collect();
        Ok(shelters)
    }
}

#[derive(Debug, Clone, Hash)]
pub struct ShelterMutations;

#[derive(Debug, Clone, InputObject)]
pub struct CreateShelterInput {
    pub name: String,
    pub about: Option<String>,
    pub image_url: Option<String>,
    pub email: Option<String>,
    pub phone: String,
    pub website_url: Option<String>,
    pub address: AddressInput,
    pub location: Coordinate,
    pub capacity: ShelterSpaceInput,
    pub food: ShelterFood,
    pub tags: Set<ShelterTag>,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct CreateShelterPayload {
    pub shelter: Shelter,
}

#[derive(Debug, Clone, InputObject)]
pub struct UpdateShelterInput {
    pub shelter_id: Id,
    pub name: Option<String>,
    pub about: Option<String>,
    pub image_url: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website_url: Option<String>,
    pub address: Option<AddressInput>,
    pub location: Option<Coordinate>,
    pub capacity: Option<ShelterSpaceInput>,
    pub food: Option<ShelterFood>,
    pub tags: Option<Set<ShelterTag>>,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct UpdateShelterPayload {
    pub shelter: Shelter,
}

#[derive(Debug, Clone, InputObject)]
pub struct DeleteShelterInput {
    pub shelter_id: Id,
}

#[Object]
impl ShelterMutations {
    /// Register a new `Shelter`.
    async fn create_shelter(
        &self,
        ctx: &Context<'_>,
        input: CreateShelterInput,
    ) -> FieldResult<CreateShelterPayload> {
        let CreateShelterInput {
            name,
            about,
            image_url,
            email,
            phone,
            website_url,
            address,
            location,
            capacity,
            food,
            tags,
        } = input;

        // Get service.
        let service = get_service(ctx);

        // Get authenticated user.
        let viewer = get_viewer(ctx)
            .await
            .context("failed to get authenticated user")
            .into_field_result()?;

        // Only admins can register shelters.
        if !viewer.is_admin {
            let error = FieldError::new("not authorized");
            return Err(error);
        }

        // Create shelter in service.
        let shelter = {
            let request = {
                let name = name
                    .try_into()
                    .context("invalid name")
                    .into_field_result()?;

                let about = about
                    .map(TryInto::try_into)
                    .transpose()
                    .context("invalid about text")
                    .into_field_result()?;
                let image_url = image_url
                    .map(|url| url.parse())
                    .transpose()
                    .context("invalid image URL")
                    .into_field_result()?;
                let email = email
                    .map(TryInto::try_into)
                    .transpose()
                    .context("invalid email address")
                    .into_field_result()?;
                let phone = phone
                    .try_into()
                    .context("invalid phone number")
                    .into_field_result()?;

                let website_url = website_url
                    .map(|url| url.parse())
                    .transpose()
                    .context("invalid website URL")?;
                let address = address.into();
                let location = location.into();

                CreateShelterRequest {
                    name,
                    about,
                    image_url,
                    email,
                    phone,
                    website_url,
                    address,
                    location,
                    capacity: capacity.into(),
                    food: food.into(),
                    tags: tags.into_iter().map(Into::into).collect(),
                }
            };
            let response =
                service.create_shelter(request).await.into_field_result()?;
            response.shelter
        };

        // Respond with payload.
        let payload = CreateShelterPayload {
            shelter: shelter.into(),
        };
        Ok(payload)
    }

    /// Update a `Shelter`'s details.
    async fn update_shelter(
        &self,
        ctx: &Context<'_>,
        input: UpdateShelterInput,
    ) -> FieldResult<UpdateShelterPayload> {
        let UpdateShelterInput {
            shelter_id,
            name,
            about,
            image_url,
            email,
            phone,
            website_url,
            address,
            location,
            capacity,
            food,
            tags,
        } = input;

        // Validate shelter ID.
        let shelter_id = shelter_id
            .get::<Shelter>()
            .context("invalid shelter ID")
            .into_field_result()?;

        // Get service.
        let service = get_service(ctx);

        // Get authenticated user.
        let viewer = get_viewer(ctx)
            .await
            .context("failed to get authenticated user")
            .into_field_result()?;

        // Only admins can update shelters.
        if !viewer.is_admin {
            let error = FieldError::new("not authorized");
            return Err(error);
        }

        // Update shelter in service.
        let shelter = {
            let request = {
                let name = name
                    .map(TryInto::try_into)
                    .transpose()
                    .context("invalid name")
                    .into_field_result()?;

                let about = about
                    .map(TryInto::try_into)
                    .transpose()
                    .context("invalid about text")
                    .into_field_result()?;
                let image_url = image_url
                    .map(|url| url.parse())
                    .transpose()
                    .context("invalid image URL")
                    .into_field_result()?;
                let email = email
                    .map(TryInto::try_into)
                    .transpose()
                    .context("invalid email address")
                    .into_field_result()?;
                let phone = phone
                    .map(TryInto::try_into)
                    .transpose()
                    .context("invalid phone number")
                    .into_field_result()?;

                let website_url = website_url
                    .map(|url| url.parse())
                    .transpose()
                    .context("invalid website URL")
                    .into_field_result()?;
                let address = address.map(Into::into);
                let location = location.map(Into::into);

                let capacity = capacity.map(Into::into);
                let food = food.map(Into::into);
                let tags =
                    tags.map(|tags| tags.into_iter().map(Into::into).collect());

                UpdateShelterRequest {
                    shelter_id,
                    name,
                    about,
                    image_url,
                    email,
                    phone,
                    website_url,
                    address,
                    location,
                    capacity,
                    food,
                    tags,
                }
            };
            let response =
                service.update_shelter(request).await.into_field_result()?;
            response.shelter
        };

        // Return payload.
        let payload = UpdateShelterPayload {
            shelter: shelter.into(),
        };
        Ok(payload)
    }

    /// Delete a `Shelter`.
    async fn delete_shelter(
        &self,
        ctx: &Context<'_>,
        input: DeleteShelterInput,
    ) -> FieldResult<bool> {
        let DeleteShelterInput { shelter_id } = input;

        // Validate shelter ID.
        let shelter_id = shelter_id
            .get::<Shelter>()
            .context("invalid shelter ID")
            .into_field_result()?;

        // Get service.
        let service = get_service(ctx);

        // Get authenticated user.
        let viewer = get_viewer(ctx)
            .await
            .context("failed to get authenticated user")
            .into_field_result()?;

        // Only admins can delete shelters.
        if !viewer.is_admin {
            let error = FieldError::new("not authorized");
            return Err(error);
        }

        // Update shelter in service.
        {
            let request = DeleteShelterRequest { shelter_id };
            let _response =
                service.delete_shelter(request).await.into_field_result()?;
        };

        // Respond with payload.
        Ok(true)
    }
}
