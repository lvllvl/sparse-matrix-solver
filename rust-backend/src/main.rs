#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_cors;

use rand::Rng;
use rocket::http::Method;
use rocket_contrib::json::{ Json, JsonValue };
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use serde_derive::Serialize;
use std::sync::Mutex;
use rocket::State;

type Matrix = Vec<Vec<f64>>;

struct MatrixData {
    matrix: Vec<Vec<f64>>,
    b: Vec<f64>,
    solution: Option<Vec<f64>>,
}

#[derive(Serialize)]
struct SolutionResponse {
    solution: Option<Vec<f64>>,
}

#[derive(Serialize)]
struct MatrixResponse {
    matrix: Matrix,
}


fn main() {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post ].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    let matrix_data = Mutex::new( MatrixData {
        matrix: Vec::new(),
        b: Vec::new(),
        solution: None,
    });

    rocket::ignite()
        .manage( matrix_data )
        .mount("/", routes![ generate_matrix, solve_matrix, get_solution ])
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

fn solve_sparse_matrix_jacobi(matrix: Vec<Vec<f64>>, b: Vec<f64>, tolerance: f64) -> Vec<f64> {
    let n = matrix.len();
    let mut x = vec![0.0; n]; // Initial guess
    let mut new_x = x.clone();
    let mut error;

    loop {
        for i in 0..n {
            let mut sigma = 0.0;
            for j in 0..n {
                if i != j {
                    sigma += matrix[i][j] * x[j];
                }
            }
            new_x[i] = (b[i] - sigma) / matrix[i][i];
        }
        error = new_x.iter().zip(x.iter()).fold(0.0, |acc, (&new, &old)| acc + (new - old).abs());
        if error < tolerance {
            break;
        }
        x = new_x.clone();
    }
    x
}

// Generate a sparse matrix and vector b
#[post("/generateMatrix")]
fn generate_matrix(state: State<Mutex<MatrixData>>) -> Json<MatrixResponse> {
    let mut data = state.lock().unwrap();
    data.matrix = generate_sparse_matrix(7, 7, 0.3); // Adjust the sparsity factor as needed
    data.b = vec![1.0; 7]; // Example values for b
    data.solution = None;

    Json(MatrixResponse {
        matrix: data.matrix.clone()
    })
}

// Solve the matrix
#[post("/solveMatrix")]
fn solve_matrix(state: State<Mutex<MatrixData>>) {
    let mut data = state.lock().unwrap();
    if data.solution.is_none() {
        let tolerance = 1e-6;
        data.solution = Some(solve_sparse_matrix_jacobi(data.matrix.clone(), data.b.clone(), tolerance));
    }
}

#[get("/getSolution")]
fn get_solution(state: State<Mutex<MatrixData>>) -> Json<SolutionResponse> {
    let data = state.lock().unwrap();
    Json(SolutionResponse {
        solution: data.solution.clone()
    })
}
