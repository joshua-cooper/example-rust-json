use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use warp::Filter;

#[tokio::main]
async fn main() {
    let endpoint = warp::post()
        .and(warp::body::json())
        .map(|body| warp::reply::json(&calculation_handler(body)));
    warp::serve(endpoint).run(([127, 0, 0, 1], 5000)).await;
}

fn calculation_handler(request: Request) -> Response {
    let result = match (request.calculation, request.shape) {
        (Calculation::Perimeter, Shape::Circle { radius }) => PI * 2.0 * radius,
        (Calculation::Perimeter, Shape::Rectangle { length, width }) => 2.0 * length + 2.0 * width,
        (Calculation::Area, Shape::Circle { radius }) => PI * radius * radius,
        (Calculation::Area, Shape::Rectangle { length, width }) => length * width,
    };

    Response { result }
}

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    calculation: Calculation,
    #[serde(flatten)]
    shape: Shape,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "shape", rename_all = "lowercase")]
enum Shape {
    Circle { radius: f64 },
    Rectangle { length: f64, width: f64 },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Calculation {
    Perimeter,
    Area,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    result: f64,
}
