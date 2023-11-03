#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_cors;

use rand::Rng;
use rocket::http::Method;
use rocket::{response, State};
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use serde_derive::Serialize;
use simple_logger::SimpleLogger;
use std::cmp;
use std::sync::Mutex;

use log::LevelFilter;
use log::{error, info};

type Matrix = Vec<Vec<f64>>;

struct MatrixData {
    matrix: Vec<Vec<f64>>,
    b: Vec<f64>,
    solution: Option<Vec<f64>>,
}

#[derive(Serialize)]
struct SolutionResponse {
    solution: Option<Vec<f64>>,
    error: Option<String>,
}

#[derive(Serialize)]
struct MatrixResponse {
    matrix: Matrix,
}

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-type",
            "Cache-Control",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    let matrix_data = Mutex::new(MatrixData {
        matrix: Vec::new(),
        b: Vec::new(),
        solution: None,
    });

    rocket::ignite()
        .manage(matrix_data)
        .mount("/", routes![generate_matrix, solve_matrix, get_solution])
        .attach(cors)
        .launch();
}

fn solve_sparse_matrix_jacobi(
    matrix: Vec<Vec<f64>>,
    b: Vec<f64>,
    tolerance: f64,
) -> Result<Vec<f64>, String> {
    let n = matrix.len();
    let mut x = vec![0.0; n];
    let mut new_x = x.clone();
    let max_iterations = 10000; // Set a maximum for iterations
    let mut iterations = 0;

    while iterations < max_iterations {
        iterations += 1;
        for i in 0..n {
            let mut sigma = 0.0;
            for j in 0..n {
                if i != j {
                    sigma += matrix[i][j] * x[j];
                }
            }
            new_x[i] = (b[i] - sigma) / matrix[i][i];
        }

        // Check for convergence
        let error: f64 = new_x
            .iter()
            .zip(x.iter())
            .map(|(&new, &old)| (new - old).abs())
            .sum();
        if error < tolerance {
            return Ok(new_x);
        }
        x = new_x.clone();
    }
    Err("Maximum iterations reached without convergence".to_string())
}


fn generate_sparse_matrix(m: usize, n: usize, s: f64) -> Matrix {
    let mut rng = rand::thread_rng();
    let mut matrix = vec![vec![0.0; n]; m];

    // Generate the sparse elements
    let total_elements = m * n;
    let non_zero_elements = (s * total_elements as f64).ceil() as usize;

    for _ in 0..non_zero_elements {
        let mut row = rng.gen_range(0..m);
        let mut col = rng.gen_range(0..n);

        // Ensure we don't overwrite the diagonal elements
        while matrix[row][col] != 0.0 || row == col {
            row = rng.gen_range(0..m);
            col = rng.gen_range(0..n);
        }

        // Generate a random float and format it to 2 decimal places
        let value = rng.gen::<f64>();
        matrix[row][col] = format!("{:.2}", value).parse::<f64>().unwrap();
    }

    // Post-processing to ensure non-zero diagonals and diagonal dominance
    for i in 0..cmp::min(m, n) {
        // Generate and format the diagonal element
        let diagonal_value = rng.gen_range(1.0..2.0);
        matrix[i][i] = format!("{:.2}", diagonal_value).parse::<f64>().unwrap();

        // Ensure diagonal dominance
        let row_sum: f64 = matrix[i].iter().filter(|&&x| x != 0.0).sum();
        matrix[i][i] += row_sum;
        // Format the updated diagonal value to two decimal places
        matrix[i][i] = format!("{:.2}", matrix[i][i]).parse::<f64>().unwrap();
    }

    matrix
}

// Generate a sparse matrix and vector b
#[post("/generateMatrix")]
fn generate_matrix(state: State<Mutex<MatrixData>>) -> Json<MatrixResponse> {
    let mut data = state.lock().unwrap();
    data.matrix = generate_sparse_matrix(7, 7, 0.3); // Adjust the sparsity factor as needed
    data.b = vec![1.0; 7]; // Example values for b
    data.solution = None;

    info!("Generated matrix: {:?}", data.matrix);

    Json(MatrixResponse {
        matrix: data.matrix.clone(),
    })
}

#[post("/solveMatrix")]
fn solve_matrix(state: State<Mutex<MatrixData>>) -> Json<SolutionResponse> {
    let mut data = state.lock().unwrap();
    let mut response_error = None;

    if data.solution.is_none() {
        let tolerance = 1e-6;
        match solve_sparse_matrix_jacobi(data.matrix.clone(), data.b.clone(), tolerance) {
            Ok(solution) => {
                data.solution = Some(solution);
                info!("Solved matrix, solution: {:?}", data.solution);
            }
            Err(message) => {
                error!("Error solving matrix: {}", message);
                // Set the error field in the response
                response_error = Some(message);
                // Ensure the solution field is None
                data.solution = None;
            }
        }
    } else {
        info!("Matrix already solved");
    }

    // Construct your JSON response
    Json(SolutionResponse {
        solution: data.solution.clone(),
        error: response_error,
    })
}

#[get("/getSolution")]
fn get_solution(state: State<Mutex<MatrixData>>) -> Json<SolutionResponse> {
    let data = state.lock().unwrap();
    Json(SolutionResponse {
        solution: data.solution.clone(),
        error: None,
    })
}
