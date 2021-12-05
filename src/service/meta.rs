use super::prelude::*;

pub struct Meta {
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Meta {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl Default for Meta {
    fn default() -> Self {
        Self::new()
    }
}
