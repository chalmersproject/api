use super::prelude::*;

use std::fmt::Debug;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum Verifiable<T> {
    Verified(T),
    Unverified(T),
}

impl<T> Verifiable<T> {
    pub fn new(inner: T, verified: bool) -> Verifiable<T> {
        use Verifiable::*;
        if verified {
            Verified(inner)
        } else {
            Unverified(inner)
        }
    }

    pub fn get(&self) -> &T {
        use Verifiable::*;
        match self {
            Verified(inner) => inner,
            Unverified(inner) => inner,
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        use Verifiable::*;
        match self {
            Verified(inner) => inner,
            Unverified(inner) => inner,
        }
    }

    pub fn into_inner(self) -> T {
        use Verifiable::*;
        match self {
            Verified(inner) => inner,
            Unverified(inner) => inner,
        }
    }

    pub fn is_verified(&self) -> bool {
        use Verifiable::*;
        matches!(self, Verified(_))
    }
}
