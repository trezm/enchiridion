use thruster::{
    errors::ThrusterError, middleware_fn, BasicHyperContext as Ctx, MiddlewareNext,
    MiddlewareResult,
};

#[middleware_fn]
pub async fn github(context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let _state = match context.query_params.get("state") {
        Some(s) => s,
        None => {
            return Err(ThrusterError {
                context: Ctx::default(),
                message: "The OAuth request must contain a state in the query parameters."
                    .to_string(),
                status: 400,
                cause: None,
            })
        }
    };

    let _code = match context.query_params.get("code") {
        Some(s) => s,
        None => {
            return Err(ThrusterError {
                context: Ctx::default(),
                message: "The OAuth request must contain a code in the query parameters."
                    .to_string(),
                status: 400,
                cause: None,
            })
        }
    };

    Ok(context)
}
