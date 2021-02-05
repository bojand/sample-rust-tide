use std::env;
use std::collections::HashMap;
use tide::prelude::*;
use tide::{http::mime, Body, Request, Response, StatusCode, log};

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

    let mut app = tide::new();
    
    app.with(tide::log::LogMiddleware::new());
    app.at("/hello").post(get_hello);
    app.at("/headers").get(get_headers);
    
    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_e) => "3000".to_string(),
    };

    

    app.listen(format!("0.0.0.0:{}", port)).await?;

    Ok(())
}

async fn get_hello(mut req: Request<()>) -> tide::Result {
    let HelloRequest { name } = req.body_json().await?;
    let msg = HelloResponse {
        message: format!("Hello {}!!", name),
    };

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(Body::from_json(&msg)?);
    Ok(res)
}

async fn get_headers(req: Request<()>) -> tide::Result {
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
