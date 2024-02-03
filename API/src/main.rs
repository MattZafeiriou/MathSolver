mod math_handler;

// use percent_encoding::{percent_decode_str, utf8_percent_encode, CONTROLS};
// use warp::Filter;
use math_handler::handle_math_equation;

// fn handle_test_api_request() -> &'static str {
//     "OK"
// }

// fn handle_test_request() -> &'static str {
//     "Test Endpoint"
// }

// #[tokio::main]
// async fn main() {
//     let test_api = warp::path!("test_API")
//         .map(|| handle_test_api_request());

//     let test = warp::path!("test")
//         .map(|| handle_test_request());

//     let math = warp::path!("math")
//         .and(warp::query::raw())
//         .map(|query: String| {
//             // Decode the URL-encoded query string
//             let decoded_query = percent_decode_str(&query).decode_utf8_lossy().to_string();

//             // Extract the value of the 'equation' parameter
//             let equation = decoded_query
//                 .split('&')
//                 .find(|part| part.starts_with("equation="))
//                 .map(|part| part.replace("equation=", ""))
//                 .unwrap_or_else(|| String::from(""));

//             handle_math_request(equation)
//         });

//     let routes = test_api.or(test).or(math).with(warp::cors().allow_any_origin());

//     warp::serve(routes).run(([127, 0, 0, 1], 9000)).await;
// }

fn main()
{
    println!("{}",handle_math_equation(String::from("")));
}