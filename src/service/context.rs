use super::prelude::*;

/// A request-scoped context.
#[derive(Debug, Clone, Default)]
pub struct Context {
    /// The current authenticated user.
    pub viewer: Option<ContextViewer>,
}

#[derive(Debug, Clone)]
pub enum ContextViewer {
    User(User),
    Anonymous,
}

impl Context {
    pub fn internal(&self) -> Context {
        let mut context = self.clone();
        context.viewer = None;
        context
    }

    pub fn is_internal(&self) -> bool {
        match &self.viewer {
            Some(ContextViewer::User(user)) => user.is_admin,
            _ => true,
        }
    }
}

impl Context {
    pub fn viewing_user(&self) -> Option<&User> {
        self.viewer.as_ref().and_then(|viewer| match viewer {
            ContextViewer::User(user) => Some(user),
            ContextViewer::Anonymous => None,
        })
    }
}
