use thruster::{middleware_fn, BasicHyperContext as Ctx, MiddlewareNext, MiddlewareResult};

#[middleware_fn]
pub async fn github(context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    Ok(context)
}
