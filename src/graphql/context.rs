use super::prelude::*;

pub fn get_service<'a>(
    ctx: &'a Context<'_>,
) -> (&'a Service, &'a ServiceContext) {
    let service = ctx.data_unchecked::<Arc<Service>>();
    let context = ctx.data_unchecked::<ServiceContext>();
    (service.as_ref(), context)
}

pub fn get_auth<'a>(ctx: &'a Context<'_>) -> Option<&'a AuthInfo> {
    ctx.data_opt()
}
