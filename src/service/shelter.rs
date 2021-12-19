use super::prelude::*;

use models::Shelter as ShelterModel;
use models::Signal as SignalModel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shelter {
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,

    pub name: String,
    pub slug: Slug,

    pub about: Option<String>,
    pub image_url: Option<Url>,
    pub email: Option<Email>,
    pub phone: Phone,

    pub website_url: Option<Url>,
    pub address: Address,
    pub location: Coordinate,

    pub capacity: ShelterSpace,
    pub occupancy: Option<ShelterSpace>,
    pub food: ShelterFood,
    pub tags: Set<ShelterTag>,
}

#[derive(Debug, Clone, Hash, Default, Serialize, Deserialize)]
pub struct ShelterSpace {
    pub spots: u16,
    pub beds: u16,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShelterTag {
    Adult,
    Youth,
    Family,
    Male,
    Female,
    LGBTQ,
    Pets,
}

impl Display for ShelterTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = to_plain_string(self).map_err(|_| FmtError)?;
        s.fmt(f)
    }
}

impl FromStr for ShelterTag {
    type Err = SerdePlainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_plain_str(s)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShelterFood {
    Meals,
    Snacks,
    None,
}

impl Display for ShelterFood {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = to_plain_string(self).map_err(|_| FmtError)?;
        s.fmt(f)
    }
}

impl FromStr for ShelterFood {
    type Err = SerdePlainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_plain_str(s)
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetShelterRequest {
    pub shelter_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetShelterResponse {
    pub shelter: Option<Shelter>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetShelterBySlugRequest {
    pub slug: Slug,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetShelterBySlugResponse {
    pub shelter: Option<Shelter>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct GetShelterSignalsRequest {
    pub shelter_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetShelterSignalsResponse {
    pub signals: Vec<Signal>,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct ListSheltersRequest {
    pub limit: u32,
    pub offset: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSheltersResponse {
    pub shelters: Vec<Shelter>,
}

// #[derive(Debug, Clone, Hash, Serialize, Deserialize)]
// pub struct ListShelterMeasurementsRequest {
//     pub shelter_id: Uuid,
//     pub limit: u32,
//     pub offset: u32,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ListShelterMeasurementsResponse {
//     pub measurements: Vec<ShelterMeasurement>,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShelterRequest {
    pub name: InputString,
    pub about: Option<InputString>,
    pub image_url: Option<Url>,
    pub email: Option<Email>,
    pub phone: Phone,
    pub website_url: Option<Url>,
    pub address: Address,
    pub location: Coordinate,
    pub capacity: ShelterSpace,
    pub food: ShelterFood,
    pub tags: Set<ShelterTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShelterResponse {
    pub shelter: Shelter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateShelterRequest {
    pub shelter_id: Uuid,
    pub name: Option<InputString>,
    pub about: Option<InputString>,
    pub image_url: Option<Url>,
    pub email: Option<Email>,
    pub phone: Option<Phone>,
    pub website_url: Option<Url>,
    pub address: Option<Address>,
    pub location: Option<Coordinate>,
    pub capacity: Option<ShelterSpace>,
    pub food: Option<ShelterFood>,
    pub tags: Option<Set<ShelterTag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateShelterResponse {
    pub shelter: Shelter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteShelterRequest {
    pub shelter_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteShelterResponse {}

impl Service {
    pub(super) async fn can_list_shelters(
        &self,
        _context: &Context,
    ) -> Result<bool> {
        // Shelters are publicly viewable.
        Ok(true)
    }

    pub(super) async fn can_view_shelter(
        &self,
        _context: &Context,
        _shelter_id: Uuid,
    ) -> Result<bool> {
        // Shelters are publicly listable.
        Ok(true)
    }

    pub(super) async fn can_edit_shelter(
        &self,
        context: &Context,
        _shelter_id: Uuid,
    ) -> Result<bool> {
        if context.is_internal() {
            return Ok(true);
        }

        // Restrict shelter editing.
        Ok(false)
    }
}

impl Service {
    pub async fn get_shelter(
        &self,
        context: &Context,
        request: GetShelterRequest,
    ) -> Result<GetShelterResponse> {
        let GetShelterRequest { shelter_id } = request;

        let shelter = {
            let pool = self.db_pool.clone();
            let shelter =
                spawn_blocking(move || -> Result<Option<ShelterModel>> {
                    use schema::shelters;
                    let conn =
                        pool.get().context("database connection failure")?;
                    shelters::table
                        .find(shelter_id)
                        .first(&conn)
                        .optional()
                        .context("failed to load shelter model")
                })
                .await
                .unwrap()?;
            shelter
                .map(Shelter::try_from)
                .transpose()
                .context("failed to decode shelter model")?
        };

        // Assert shelter is viewable.
        if shelter.is_some()
            && !self.can_view_shelter(context, shelter_id).await?
        {
            bail!("not authorized");
        }

        let response = GetShelterResponse { shelter };
        Ok(response)
    }

    pub async fn get_shelter_by_slug(
        &self,
        context: &Context,
        request: GetShelterBySlugRequest,
    ) -> Result<GetShelterBySlugResponse> {
        let GetShelterBySlugRequest { slug } = request;

        let shelter = {
            let pool = self.db_pool.clone();
            let slug = slug.to_string();
            let shelter =
                spawn_blocking(move || -> Result<Option<ShelterModel>> {
                    use schema::shelters;
                    let conn =
                        pool.get().context("database connection failure")?;
                    shelters::table
                        .filter(shelters::slug.eq(slug))
                        .first(&conn)
                        .optional()
                        .context("failed to load shelter model")
                })
                .await
                .unwrap()?;
            shelter
                .map(Shelter::try_from)
                .transpose()
                .context("failed to decode shelter model")?
        };

        // Assert shelter is viewable.
        if let Some(shelter) = &shelter {
            if !self.can_view_shelter(context, shelter.id).await? {
                bail!("not authorized");
            };
        }

        let response = GetShelterBySlugResponse { shelter };
        Ok(response)
    }

    pub async fn get_shelter_signals(
        &self,
        context: &Context,
        request: GetShelterSignalsRequest,
    ) -> Result<GetShelterSignalsResponse> {
        let GetShelterSignalsRequest { shelter_id } = request;

        // Assert shelter is viewable.
        if !self.can_view_shelter(context, shelter_id).await? {
            bail!("not authorized");
        };

        let signals: Vec<Signal> = {
            let pool = self.db_pool.clone();
            let models = spawn_blocking(move || -> Result<Vec<SignalModel>> {
                use schema::signals;
                let conn = pool.get().context("database connection failure")?;
                signals::table
                    .filter(signals::shelter_id.eq(shelter_id))
                    .load(&conn)
                    .context("failed to load signal model")
            })
            .await
            .unwrap()?;
            models
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_>>()
                .context("failed to decode signal model")?
        };

        let response = GetShelterSignalsResponse { signals };
        Ok(response)
    }

    pub async fn list_shelters(
        &self,
        context: &Context,
        request: ListSheltersRequest,
    ) -> Result<ListSheltersResponse> {
        let ListSheltersRequest { limit, offset } = request;

        // Assert shelter is viewable.
        if !self.can_list_shelters(context).await? {
            bail!("not authorized");
        }

        let shelters = {
            let pool = self.db_pool.clone();
            let models =
                spawn_blocking(move || -> Result<Vec<ShelterModel>> {
                    use schema::shelters;
                    let conn =
                        pool.get().context("database connection failure")?;
                    shelters::table
                        .limit(limit.into())
                        .offset(offset.into())
                        .load(&conn)
                        .context("failed to load shelter models")
                })
                .await
                .unwrap()?;
            models
                .into_iter()
                .map(Shelter::try_from)
                .collect::<Result<Vec<_>>>()
                .context("failed to decode shelter models")?
        };

        let response = ListSheltersResponse { shelters };
        Ok(response)
    }

    // pub async fn list_shelter_measurements(
    //     &self,
    //     context: &Context,
    //     request: ListShelterMeasurementsRequest,
    // ) -> Result<ListShelterMeasurementsResponse> {
    //     let ListShelterMeasurementsRequest {
    //         shelter_id,
    //         limit,
    //         offset,
    //     } = request;

    //     // Assert shelter is viewable.
    //     if !self.can_view_shelter(context, shelter_id).await? {
    //         bail!("not authorized");
    //     }

    //     // List measurements.
    //     let measurements = {
    //         let pool = self.db_pool.clone();
    //         let models = spawn_blocking(
    //             move || -> Result<Vec<ShelterMeasurementModel>> {
    //                 use schema::shelter_measurements as measurements;
    //                 let conn =
    //                     pool.get().context("database connection failure")?;
    //                 measurements::table
    //                     .filter(measurements::shelter_id.eq(shelter_id))
    //                     .limit(limit.into())
    //                     .offset(offset.into())
    //                     .load(&conn)
    //                     .context("failed to load shelter measurement models")
    //             },
    //         )
    //         .await
    //         .unwrap()?;
    //         models
    //             .into_iter()
    //             .map(ShelterMeasurement::try_from)
    //             .collect::<Result<Vec<_>>>()
    //             .context("failed to decode shelter measurement models")?
    //     };

    //     let response = ListShelterMeasurementsResponse { measurements };
    //     Ok(response)
    // }

    pub async fn create_shelter(
        &self,
        context: &Context,
        request: CreateShelterRequest,
    ) -> Result<CreateShelterResponse> {
        let CreateShelterRequest {
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
        } = request;

        // Restrict shelter creation.
        if !context.is_internal() {
            bail!("not authorized");
        }

        // Create shelter.
        let shelter = {
            let Meta {
                id,
                created_at,
                updated_at,
            } = Meta::new();

            Shelter {
                id,
                created_at,
                updated_at,

                name: name.into(),
                slug: Default::default(),

                about: about.map(Into::into),
                image_url,
                email,
                phone,

                website_url,
                address,
                location,

                capacity,
                occupancy: None,
                food,
                tags,
            }
        };

        // Create shelter in database.
        {
            let pool = self.db_pool.clone();
            let shelter = ShelterModel::try_from(shelter.clone())
                .context("failed to encode shelter")?;
            spawn_blocking(move || -> Result<()> {
                use schema::shelters;
                let conn = pool.get().context("database connection failure")?;
                insert_into(shelters::table)
                    .values(shelter)
                    .execute(&conn)
                    .context("failed to insert shelter model")?;
                Ok(())
            })
            .await
            .unwrap()?
        };

        let response = CreateShelterResponse { shelter };
        Ok(response)
    }

    pub async fn update_shelter(
        &self,
        context: &Context,
        request: UpdateShelterRequest,
    ) -> Result<UpdateShelterResponse> {
        let UpdateShelterRequest {
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
        } = request;

        // Assert shelter is editable.
        if !self.can_edit_shelter(context, shelter_id).await? {
            bail!("not authorized");
        };

        // Fetch shelter.
        let mut shelter = {
            let context = context.internal();
            let request = GetShelterRequest { shelter_id };
            let response = self
                .get_shelter(&context, request)
                .await
                .context("failed to get shelter")?;
            response.shelter.context("shelter not found")?
        };

        // Mutate shelter.
        if let Some(name) = name {
            shelter.name = name.into();
        }
        if let Some(about) = about {
            shelter.about = about.discard_empty().map(Into::into);
        }
        if let Some(url) = image_url {
            shelter.image_url = Some(url);
        }
        if let Some(email) = email {
            shelter.email = Some(email);
        }
        if let Some(phone) = phone {
            shelter.phone = phone;
        }
        if let Some(url) = website_url {
            shelter.website_url = Some(url);
        }
        if let Some(address) = address {
            shelter.address = address;
        }
        if let Some(location) = location {
            shelter.location = location;
        }
        if let Some(space) = capacity {
            shelter.capacity = space
        }
        if let Some(food) = food {
            shelter.food = food;
        }
        if let Some(tags) = tags {
            shelter.tags = tags;
        }

        // Update shelter in database.
        {
            let pool = self.db_pool.clone();
            let shelter = ShelterModel::try_from(shelter.clone())
                .context("failed to encode shelter")?;
            spawn_blocking(move || -> Result<()> {
                use schema::shelters;
                let conn = pool.get().context("database connection failure")?;
                update(shelters::table.find(shelter_id))
                    .set(shelter)
                    .execute(&conn)
                    .context("failed to update shelter model")?;
                Ok(())
            })
            .await
            .unwrap()?
        };

        let response = UpdateShelterResponse { shelter };
        Ok(response)
    }

    pub async fn delete_shelter(
        &self,
        context: &Context,
        request: DeleteShelterRequest,
    ) -> Result<DeleteShelterResponse> {
        let DeleteShelterRequest { shelter_id } = request;

        // Assert shelter is editable.
        if !self.can_edit_shelter(context, shelter_id).await? {
            bail!("not authorized");
        };

        // Delete shelter in database.
        {
            let pool = self.db_pool.clone();
            spawn_blocking(move || -> Result<()> {
                use schema::shelters;
                let conn = pool.get().context("database connection failure")?;
                delete_from(shelters::table.find(shelter_id))
                    .execute(&conn)
                    .context("failed to delete shelter model")?;
                Ok(())
            })
            .await
            .unwrap()?
        };

        let response = DeleteShelterResponse {};
        Ok(response)
    }
}
