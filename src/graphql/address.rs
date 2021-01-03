use super::prelude::*;

use service::Address as AddressRepr;

#[derive(Debug, Clone, Hash, SimpleObject)]
pub struct Address {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub region: String,
    pub country: String,
    pub postcode: String,
}

impl From<AddressRepr> for Address {
    fn from(address: AddressRepr) -> Self {
        let AddressRepr {
            line1,
            line2,
            city,
            region,
            country,
            postcode,
        } = address;

        Self {
            line1,
            line2,
            city,
            region,
            country,
            postcode,
        }
    }
}

#[derive(Debug, Clone, Hash, InputObject)]
pub struct AddressInput {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub region: String,
    pub country: String,
    pub postcode: String,
}

impl From<AddressInput> for AddressRepr {
    fn from(address: AddressInput) -> Self {
        let AddressInput {
            line1,
            line2,
            city,
            region,
            country,
            postcode,
        } = address;

        Self {
            line1,
            line2,
            city,
            region,
            country,
            postcode,
        }
    }
}
