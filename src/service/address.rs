use super::prelude::*;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Address {
    pub line1: String,    // e.g. 51 Charles St W
    pub line2: String,    // e.g. Suite #199
    pub city: String,     // e.g. Waterloo
    pub region: String,   // e.g. Ontario
    pub country: String,  // e.g. Canada
    pub postcode: String, // e.g. N2G 1H6
}
