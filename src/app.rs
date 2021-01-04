use hyper::Body;
use log::error;
use std::env;
use std::sync::Arc;
use thruster::context::basic_hyper_context::{
    generate_context, BasicHyperContext as Ctx, HyperRequest,
};
use thruster::App;
use thruster::{async_middleware, middleware_fn};
use thruster::{Context, MiddlewareNext, MiddlewareResult};
use tokio;
use tokio_postgres::{Client, NoTls};

use crate::controllers::oauth;

#[middleware_fn]
async fn ping(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let val = "pong";
    context.body = Body::from(val);
    Ok(context)
}

#[middleware_fn]
async fn four_oh_four(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let val = "Oops! That route doesn't exist.";
    context.body = Body::from(val);
    context.status(404);
    Ok(context)
}

pub async fn init() -> App<HyperRequest, Ctx, Arc<Client>> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:postgres@localhost/enchiridion".to_string());

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {}", e);
        }
    });

    let mut app = App::<HyperRequest, Ctx, Arc<Client>>::create(generate_context, Arc::new(client));
    app.get("/ping", async_middleware!(Ctx, [ping]));
    app.get(
        "/users/github/oauth",
        async_middleware!(
            Ctx,
            [
                thruster::middleware::query_params::query_params,
                oauth::github
            ]
        ),
    );
    app.set404(async_middleware!(Ctx, [four_oh_four]));

    app
}
