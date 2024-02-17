use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use fizzbuzz_rust_vercel_functions::fizzbuzz;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

fn generate_fizzbuzz(number: u64) -> String {
    let mut res = String::from("");
    for i in 1..(number + 1) {
        res.push_str(fizzbuzz(i).as_str());
        res.push_str(" ");
    }
    res
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
                "message": generate_fizzbuzz(100),
            })
                .to_string()
                .into(),
        )?)
}
