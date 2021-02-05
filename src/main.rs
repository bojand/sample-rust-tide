use std::env;
use std::collections::HashMap;
use tide::prelude::*;
use tide::{http::mime, Body, Request, Response, StatusCode, log};
use tera::Tera;
use tide_tera::prelude::*;

/// The shared application state.
#[derive(Clone)]
struct State {
    tera: Tera,
    public_url: String,
    port: String,
}

#[derive(Debug, Serialize)]
struct HelloResponse {
    message: String,
}

#[derive(Debug, Deserialize)]
struct HelloRequest {
    name: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();

    let mut tera = Tera::new("templates/**/*")?;
    tera.autoescape_on(vec!["html"]);

    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_e) => "3000".to_string(),
    };

    let public_url = match env::var("PUBLIC_URL") {
        Ok(val) => val,
        Err(_e) => "http://0.0.0.0:3000".to_string(),
    };

    let state = State {
        tera: tera,
        port: port.clone(),
        public_url: public_url,
    };

    let mut app = tide::with_state(state);
    
    app.with(tide::log::LogMiddleware::new());
    app.at("/").get(get_index);
    app.at("/hello").post(get_hello);
    app.at("/headers").get(get_headers);

    app.listen(format!("0.0.0.0:{}", port)).await?;

    Ok(())
}

async fn get_index(req: tide::Request<State>) -> tide::Result {
    let tera = &req.state().tera;
    tera.render_response("index.html", &context! { "public_url" => req.state().public_url })
}

async fn get_hello(mut req: Request<State>) -> tide::Result {
    let HelloRequest { name } = req.body_json().await?;
    let msg = HelloResponse {
        message: format!("Hello {}!", name),
    };

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&msg)?);
    Ok(res)
}

async fn get_headers(req: Request<State>) -> tide::Result {
    let mut headers: HashMap<&str, &str> = HashMap::new();
    for (name, value) in &req {
        headers.insert(name.as_str(), value.as_str());
    }

    let data = serde_json::to_string(&headers).unwrap();

    let res = Response::builder(StatusCode::Ok)
        .body(data)
        .content_type(mime::JSON)
        .build();

    Ok(res)
}
