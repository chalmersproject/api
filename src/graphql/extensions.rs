use crate::prelude::{debug as __debug, *};

use graphql::extensions::{Extension, ExtensionContext, ExtensionFactory};
use graphql::{PathSegment, Request, ServerError, ServerResult};

macro_rules! debug{
    ($($arg:tt)+) => (
        __debug!(target: "api::graphql", $($arg)+);
    )
}

pub struct Logging;

impl ExtensionFactory for Logging {
    fn create(&self) -> Box<dyn Extension> {
        Box::new(LoggingExtension {
            operation_name: None,
        })
    }
}

struct LoggingExtension {
    operation_name: Option<String>,
}

#[async_trait]
impl Extension for LoggingExtension {
    async fn prepare_request(
        &mut self,
        _: &ExtensionContext<'_>,
        request: Request,
    ) -> ServerResult<Request> {
        self.operation_name = request.operation_name.to_owned();
        Ok(request)
    }

    fn execution_start(&mut self, _: &ExtensionContext<'_>) {
        if let Some(name) = &self.operation_name {
            debug!("executing {}", name);
        } else {
            debug!("executing anonymous operation")
        };
    }

    fn error(&mut self, _ctx: &ExtensionContext<'_>, error: &ServerError) {
        let error = DisplayError {
            operation_name: &self.operation_name,
            error,
        };
        error!("{}", error);
    }
}

struct DisplayError<'a> {
    operation_name: &'a Option<String>,
    error: &'a ServerError,
}

impl<'a> Display for DisplayError<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "rejecting ")?;
        if let Some(name) = &self.operation_name {
            write!(f, "{}: ", name)?;
        } else {
            write!(f, "anonymous operation: ")?;
        }

        let ServerError {
            message,
            path,
            locations,
            ..
        } = self.error;
        write!(f, "{} (", message)?;

        if !path.is_empty() {
            write!(f, "path: ")?;
            for (i, segment) in path.iter().enumerate() {
                if i != 0 {
                    write!(f, ".")?;
                }

                use PathSegment::*;
                match segment {
                    Field(field) => f.write_str(field),
                    Index(i) => write!(f, "{}", i),
                }?;
            }
            write!(f, ", ")?;
        }

        if !locations.is_empty() {
            write!(f, "position: [")?;
            for (i, location) in locations.iter().enumerate() {
                if i != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}:{}", location.line, location.column)?;
            }
            write!(f, "]")?;
        }

        write!(f, ")")
    }
}
