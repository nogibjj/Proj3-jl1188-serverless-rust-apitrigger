use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    num: u32,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

pub fn convert(num: &u32) -> String {
    let thousands: Vec<&str> = vec!["", "M", "MM", "MMM"];
    let hundreds: Vec<&str> = vec!["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    let tens: Vec<&str> = vec!["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    let ones: Vec<&str> = vec!["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    let sizenum = *num as usize;
    let mut result: String = "".to_owned();
    result.push_str(thousands[sizenum / 1000]);
    result.push_str(hundreds[sizenum % 1000 / 100]);
    result.push_str(tens[sizenum % 100 / 10]);
    result.push_str(ones[sizenum % 10]);

    result
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let num = event.payload.num;

    let response = convert(&num);

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Convert {} to Roman: {}.", num, response),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
