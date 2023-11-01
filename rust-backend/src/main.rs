#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_cors;

use rand::Rng;
use rocket::http::Method;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use serde_derive::Serialize;

type Matrix = Vec<Vec<f64>>;

#[derive(Serialize)]
struct Response {
    matrix: Matrix,
}

fn main() {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::ignite()
        .mount("/", routes![api_generate_sparse_matrix])
        .attach(cors)
        .launch();
}

fn generate_sparse_matrix(m: usize, n: usize, s: f64) -> Matrix {
    let mut rng = rand::thread_rng();
    let mut matrix = vec![vec![0.0; n]; m];

    let total_elements = m * n;
    let non_zero_elements = (s * total_elements as f64).ceil() as usize;

    for _ in 0..non_zero_elements {
        let mut row = rng.gen_range(0..m);
        let mut col = rng.gen_range(0..n);

        while matrix[row][col] != 0.0 {
            row = rng.gen_range(0..m);
            col = rng.gen_range(0..n);
        }

        // Generate a random float and format it to 2 decimal places
        let value = rng.gen::<f64>();
        let formatted_value = format!("{:.2}", value).parse::<f64>().unwrap();
        matrix[row][col] = formatted_value 
    }
    matrix
}

#[get("/generateMatrix")]
fn api_generate_sparse_matrix() -> Json<Response> {
    let matrix = generate_sparse_matrix(10, 10, 0.2); // example values
    Json(Response { matrix })
}
