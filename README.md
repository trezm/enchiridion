# Building Enchiridion

The story about building this here website. So meta. Enjoy.

## Introduction

A while back, I found an article that was amazing. Actually, the content was so-so, but the style was fantastic. It featured a description along the right hand side, and then code along the left side. The big difference here, was that when you scrolled to the correct position, the left side animated in and out what was being added and at what line. I found it so intuitive, I had to try myself. Then as I investigated what they used, I found that it was a framework based on some sort of slides based framework, and I wanted none of it.

So I built one myself. I wanted to write more, and I figured that this was a great opportunity to force myself into writing as well as making some useful products.

After I made a working version, I thought; "Hey, this is pretty useful! It would be great if you could login with github and magically publish Markdown files in this format."

And thus, Enchiridion was born.

## The stack

The stack is simple. We'll have

- Postgres (for tokens)
- GitHub (for OAuth and static markdown serving)
- Thruster (an http rust framework)
- Fancy-Code (the nice little library for sliding code in and out of the side left bar)

## Let's start

Like any great Rust project, we start with

```setup.sh
mkdir enchiridion
cd enchiridion
cargo init --bin
```

This creates us a new rust project with all the basic goodies (`Cargo.toml`, etc.). With this in hand, we can begin adding a few imports that we'll need for the backend. Namely we'll need (or want, or user, or whatever...):

- An http framework
  - [thruster](https://github.com/thruster-rs/thruster)
- A database client
  - [tokio-postgres](https://github.com/sfackler/rust-postgres)
- A templating framework
  - [askama](https://github.com/djc/askama)

I've included a few more packages in there that we'll need -- most of which are not worth mentioning individually. We'll go over them once we use them!

```Cargo.toml
[package]
name = "enchiridion"
version = "0.1.0"
authors = ["Pete Mertz <peter.s.mertz@gmail.com>"]
edition = "2018"

[dependencies]
askama = "0.10.2"
chrono = "0.4.11"
dotenv = "0.13.0"
env_logger = "0.7.1"
thruster = { version = "1.0.0", features = ["hyper_server"] }
tokio = { version = "0.2.19", features = ["full"] }
tokio-postgres = { version = "0.5.4", features = ["with-chrono-0_4"] }

```

Fantastic. We'll also need a `package.json` for our frontend packages. to do this, simply run `npm init` and fill in the appropraite fields.

Now we'll install a few packages. Namely, we'll install the parcel bundler for easy dev mode, and the `inline-code` package to make everything look nice-nice. Note that `inline-code` is installed via its github url.

```node-setup.sh
npm install --save-dev parcel-bundler https://github.com/trezm/inline-code
```

Now, because I'm writing this as I'm making the project, I'm going to make a folder specifically for looking at this README and making sure it looks good as I write it. Go ahead and make a new folder in the root for this project and call it `client`. Then make another two files in it called `index.js` and `index.html`.
\
\
\
\
.

```index.js
const fancyCode = require("inline-code");
const markdown = require("../README.md");
const el = document.getElementById("content");

const page = new fancyCode.FC(el);
page.parse(markdown);

```

\
\
\
\
\
\
\
\
\
\
.

```index.html
<html>
  <head>
    <link
      rel="stylesheet"
      href="//cdnjs.cloudflare.com/ajax/libs/highlight.js/10.1.2/styles/vs2015.min.css"
    />
  </head>
  <body
    style="background-color: #1d1d1d; color: #ffffff; font-family: sans-serif;"
  >
    <div id="content"></div>
    <script src="./index.js"></script>
  </body>
</html>

```

Now you can see the README rendered nicely as we write it using `./node_modules/.bin/parcel client/index.html`.

Next we're going to make the basic scaffold for a thruster app. Thruster, for those who don't know (probably most of you,) is a rust http framework that's loosely based off of an express or koa style syntax. It aims to be both fast and easy to write (or use? I don't know... [ask a linguist](https://www.robertpasternak.com/)). I tend to like to start my servers with two files: `main.rs`, and `app.rs`. `main.rs` serves as the entry point for a Rust binary, and `app.rs` will serve as the primary way to access our server as an object. Splitting the code like this will _also_ enable us to test endpoints much more easily. So, our `app.rs` will look at its most basic level like this:

\
\
\
\
\
\
\
\
\
.

```src/app.rs
use hyper::Body;
use thruster::context::basic_hyper_context::{
    generate_context, BasicHyperContext as Ctx, HyperRequest,
};
use thruster::{async_middleware, middleware_fn};
use thruster::App;
use thruster::{MiddlewareNext, MiddlewareResult};

#[middleware_fn]
async fn ping(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let val = "pong";
    context.body = Body::from(val);
    Ok(context)
}

pub fn init() -> App<HyperRequest, Ctx, ()>  {
    let mut app = App::<HyperRequest, Ctx, ()>::create(generate_context, ());
    app.get("/ping", async_middleware!(Ctx, [ping]));

    app
}

```

Breaking this down line by line -- the top portion (`use` statements) are for bringing in dependencies for the file. This is all the pieces from `thruster` we'll need, as well as a piece from `hyper`. `hyper` is a lower level http library that `thruster` can use as an engine. `thruster` actually allows you to use different backends behind it, making it extremely versatile in terms of both speed and functionality.

Starting at line 10, we see a middleware function. You don't necessarily need to know how, but line 9 is what tells thruster that this particular function can be used as a piece of middleware. This middleware function simply sets the body on the response to `pong` and returns it, thereby sending the response.

Line 16 is a function that creates a new `App` object that you can make requests against. Line 18 sets the `GET /ping` route. This means that making a request to `/ping` should return `pong`.

\
\
\
\
\
\
\
\
.

```src/main.rs
use log::info;
use std::env;
use thruster::hyper_server::HyperServer;
use thruster::ThrusterServer;
use tokio;

mod app;

#[tokio::main]
async fn main() {
    env_logger::init();
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "4321".to_string());
    info!("Starting server at {}:{}", host, port);

    let app = app::init().await;
    let server = HyperServer::new(app);
    server.build(&host, port.parse::<u16>().unwrap()).await;
}

```

In the `main.rs` file, we replace the contents with this. We're bringing in some logging mechanisms (lines 1, and 2) and then grabbing `HOST` and `PORT` environment variables from lines 10 and 11 in order to bind the server to. We then actually initialize the app using the function we just made in `app.rs` (14) and create a new `HyperServer`, which is the backend that thruster will run on. Then we start the server, line 16.

We can now test this out by simply running `cargo run`. Visiting `http://localhost:4321/ping` in a browser should now give you a nice `pong` message. Congratulations, you made a rust surver 👏!

## Building Auth

I don't particularly feel like building out a full account system with password reset, etc. So we'll just be taking advantage of GitHub's OAuth system for logging in. The basic flow of this will be:

1. A user clicks a link to "Sign in with GitHub."
1. The user is then directed to GitHub, along with our client id and a one-time-use string known as a "state."
1. The user clicks "authorize" on the GitHub page, they are then redirected back to us with a "code" and the "state" from earlier.
1. The server will then take the code, and check the state, and make a `POST` to github to exchange the code for an access token.
1. The server will store the access token and corresponding id.
1. The server will redirect the user along with generating a session cookie that the browser will be able to use to authenticate. We'll store this token in a separate "session" table.
1. We're done!

Let's start by adding some shared config into the app. Thruster is able to propagate references through its middleware stack. This is very helpful for sharing configurations or, in this case, postgres client pools.

```src/app.rs
use hyper::Body;
use log::error;
use std::env;
use std::sync::Arc;
use thruster::context::basic_hyper_context::{
    generate_context, BasicHyperContext as Ctx, HyperRequest,
};
use thruster::{async_middleware, middleware_fn};
use thruster::App;
use thruster::{MiddlewareNext, MiddlewareResult};
use tokio;
use tokio_postgres::{Client, NoTls};

#[middleware_fn]
async fn ping(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    let val = "pong";
    context.body = Body::from(val);
    Ok(context)
}

pub async fn init() -> App<HyperRequest, Ctx, Arc<Client>>  {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:postgres@localhost/droppod".to_string());

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {}", e);
        }
    });

    let mut app = App::<HyperRequest, Ctx, Arc<Client>>::create(generate_context, Arc::new(client));
    app.get("/ping", async_middleware!(Ctx, [ping]));

    app
}

```

Here we grab the database from an env variable, and then create a connection to that database. Then we pass that reference via Arc (automatic reference counting,) to the app creator so that it will know to pass that as the "state" when it generates a new context.

\
\
\
\
\
.

### Testing

Now let's make our first test case! Hurray TDD! Just kidding, we'll make a test case to make sure this route exists, but we won't be tedious about TDD. That's a rant for a totally different article, so don't tune out just yet.

So we make a new folder and file, `tests/mod.rs`.

```src/tests/mod.rs
mod oauth;

```
\
\
\
.

And we'll make our first actual test file, `tess/oauth.rs`. We want to...

1. Make sure that you can call the endpoint and receive a 200
1. Make sure that given a code parameter, we make a request with that code parameter to get an access token
1. Assuming the access token request is valid, we return a valid session token

Let's start by just testing 1.

```src/tests/oauth.rs
use hyper::{Body, Request};
use thruster::testing;
use tokio::runtime::Runtime;

use crate::app;

#[test]
fn it_should_have_a_ping_route() {
    let _ = Runtime::new().unwrap().block_on(async {
        let app = app::init().await;

        let response = testing::request(
            &app,
            Request::builder()
                .method("GET")
                .uri("/users/github/oauth")
                .body(Body::from(""))
                .unwrap(),
        )
        .await;

        assert!(response.status == 200);
    });
}

```

Now we test by running `cargo test`, expecting a failure, and we get... a success? What's going on here?

In thruster, the default behavior of the framework is to log a warning, but return a 200 even if no route is found. In order to actually cause a 404, you have to explicitly tell thruster what to do. We'll add some code to `app.rs` in order to catch any unfound route, and then appropriately set the status and return a message.

\
\
\
\
\
\
\
\
\
\
\
\
\
\
\
\
.

```src/app.rs
use hyper::Body;
use log::error;
use std::env;
use std::sync::Arc;
use thruster::context::basic_hyper_context::{
    generate_context, BasicHyperContext as Ctx, HyperRequest,
};
use thruster::App;
use thruster::{async_middleware, middleware_fn};
use thruster::{MiddlewareNext, MiddlewareResult};
use tokio;
use tokio_postgres::{Client, NoTls};

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
        .unwrap_or("postgres://postgres:postgres@localhost/droppod".to_string());

    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {}", e);
        }
    });

    let mut app = App::<HyperRequest, Ctx, Arc<Client>>::create(generate_context, Arc::new(client));
    app.get("/ping", async_middleware!(Ctx, [ping]));
    app.set404(async_middleware!(Ctx, [four_oh_four]));

    app
}
```

Now we run our test again, `cargo test`, and we see a failing test! Yay?


\
\
\
\
\
\
\
\
\
\
\
\
\
\
\
\
.

Let's start actually making a route controller now. First we make two new files, `src/controllers/mod.rs`, and `src/controllers/oauth.rs` (you'll have to make the `src/controllers` folder.)

`mod.rs` is very simple, it just allows the controller folder to be used as a module. So we'll include the oauth controller like so:

```src/controllers/mod.rs
pub mod oauth;

```

\
\
\
\
.

We'll also have to update our `main.rs` to include _that_ module as well.

```src/main.rs
use log::info;
use std::env;
use thruster::hyper_server::HyperServer;
use thruster::ThrusterServer;
use tokio;

mod app;
pub(crate) mod controllers;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    env_logger::init();
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "4321".to_string());
    info!("Starting server at {}:{}", host, port);

    let app = app::init().await;
    let server = HyperServer::new(app);
    server.build(&host, port.parse::<u16>().unwrap()).await;
}

```

\
\
\
\
\
\
\
\
.

Our oauth controller should look very familiar -- it's an even simpler version of the ping middleware we made earlier! It will start life looking something like this:

```src/controllers/oauth.rs
use thruster::{middleware_fn, BasicHyperContext as Ctx, MiddlewareNext, MiddlewareResult};

#[middleware_fn]
pub async fn github(context: Ctx, _next: MiddlewareNext<Ctx>) -> MiddlewareResult<Ctx> {
    Ok(context)
}

```

\
\
\
\
\
\
\
\
.

And, finally, we'll have to actually add our newly created oauth middleware as a route to the app. All we need to do is add the line to our app folder like this:

```src/app.rs
use hyper::Body;
use log::error;
use std::env;
use std::sync::Arc;
use thruster::context::basic_hyper_context::{
    generate_context, BasicHyperContext as Ctx, HyperRequest,
};
use thruster::App;
use thruster::{async_middleware, middleware_fn};
use thruster::{MiddlewareNext, MiddlewareResult};
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
        .unwrap_or("postgres://postgres:postgres@localhost/droppod".to_string());

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
        async_middleware!(Ctx, [oauth::github]),
    );
    app.set404(async_middleware!(Ctx, [four_oh_four]));

    app
}

```

And just like that, running `cargo test`, everything passes!
