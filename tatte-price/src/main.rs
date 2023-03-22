use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response, aws_lambda_events::serde_json::json, IntoResponse};
use serde::{Deserialize, Serialize};


struct TatteList {
    menu: Vec<Dish>,
}

#[derive(Serialize)]
struct Dish {
    name: String,
    price: u32,
}

impl TatteList {
    fn new() -> TatteList {
        TatteList { menu: vec![
            Dish {name: String::from("Cheesecake"), price: 7},
            Dish {name: String::from("Breakfast Sandwich"), price: 10},
            Dish {name: String::from("French Toast"), price: 13},
            Dish {name: String::from("Avocado Tartines"), price: 12},
            Dish {name: String::from("Traditional Shakshuka"), price: 14},
            Dish {name: String::from("Chicken Pita"), price: 12},
            Dish {name: String::from("Chicken Salad"), price: 11},
        ]
        
        }
    }
}

fn get_dish_from_name<'a>(name: &'a str, tatte_list: &'a TatteList) -> Option<&'a Dish> {
    let mut iter = tatte_list.menu.iter();
    iter.find(|&dish| dish.name == name)
}

async fn build_success_response(dish: &Dish) -> Response<Body> {
    json!(dish).into_response().await
}

async fn build_failure_response(error_message: &str) -> Response<Body> {
    Response::builder()
        .status(400)
        .header("content-type", "application/json")
        .body(Body::from(json!({"error": error_message}).to_string()))
        .expect("could not build the error response")
    
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn new_tatte_list_price() {
        let all_tatte_price = TatteList::new();
        assert_eq!(all_tatte_price.menu.len(), 7);
        let cheesecake = get_dish_from_name("Cheesecake", &all_tatte_price);
        assert_eq!(cheesecake.unwrap().price, 7);
        let breaksand = get_dish_from_name("Breakfast Sandwich", &all_tatte_price);
        assert_eq!(breaksand.unwrap().price, 10);
        let french = get_dish_from_name("French Toast", &all_tatte_price);
        assert_eq!(french.unwrap().price, 13);
        let avocado = get_dish_from_name("Avocado Tartines", &all_tatte_price);
        assert_eq!(avocado.unwrap().price, 12);
        let shakshuka = get_dish_from_name("Traditional Shakshuka", &all_tatte_price);
        assert_eq!(shakshuka.unwrap().price, 14);
        let pita = get_dish_from_name("Chicken Pita", &all_tatte_price);
        assert_eq!(pita.unwrap().price, 12);
        let salad = get_dish_from_name("Chicken Salad", &all_tatte_price);
        assert_eq!(salad.unwrap().price, 11);
    }

    #[tokio::test]
    async fn build_success_response_test(){
        let test_dish = Dish {name: String::from("test dish"), price: 10};
        let result = build_success_response(&test_dish).await;
        let (parts, body) = result.into_parts();
        assert_eq!(parts.status, 200);
        assert_eq!(parts.headers.get("content-type").unwrap(), "application/json");
        assert_eq!(String::from_utf8(body.to_ascii_lowercase()).unwrap(), "{\"name\":\"test dish\",\"price\":10}");
    }

    #[tokio::test]
    async fn build_failure_response_test(){
        let result = build_failure_response("test error message").await;
        let (parts, body) = result.into_parts();
        assert_eq!(parts.status, 400);
        assert_eq!(parts.headers.get("content-type").unwrap(), "application/json");
        assert_eq!(String::from_utf8(body.to_ascii_lowercase()).unwrap(), "{\"error\":\"test error message\"}");
    }

    #[test]
    fn process_event_vaid_dish_test(){
        let tatte_list = TatteList::new();
        let res = process_event(Some("Cheesecake"), &tatte_list);
        assert!(res.is_ok());
    }
}





/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello AWS Lambda HTTP request".into())
        .map_err(Box::new)?;
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
