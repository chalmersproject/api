use super::prelude::*;

pub struct Meta {
    pub id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Meta {
    #[allow(clippy::clippy::new_without_default)]
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
        }
    }
}
