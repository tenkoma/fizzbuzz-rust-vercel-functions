use serde_json::{json, Value};
use vercel_runtime::{run, service_fn, Error, Request};
use fizzbuzz_rust_vercel_functions::fizzbuzz;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

fn generate_fizzbuzz(number: u64) -> String {
    let mut res = String::from("");
    for i in 1..(number + 1) {
        res.push_str(fizzbuzz(i).as_str());
        res.push_str(" ");
    }
    res
}

pub async fn handler(_req: Request) -> Result<Value, Error> {
    Ok(json!({
        "message": generate_fizzbuzz(100),
    }))
}
