#![feature(decl_macro)]

use rand::Rng;
use rocket::{get, routes};
use rocket::serde::json::Json;
use serde::Serialize;
use rocket_cors::{AllowedOrigins, Cors, CorsOptions};


type Matrix = Vec< Vec< f64 > > ;

#[derive(Serialize)]
struct Response { 
    matrix: Matrix,
}


fn main() {
    let rocket_instance = rocket::build().mount("/", routes![api_generate_sparse_matrix]);
    rocket_instance.launch();

    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);

    let cors_1 = CorsOptions {
        allowed_origins,
        ..Default::default()
    }
    .to_cors()
    .expect("To configure CORS");

    rocket::build()
        .attach( cors_1 )
        .mount( "/", routes![ api_generate_sparse_matrix ])
        .launch();
}


fn generate_sparse_matrix( m: usize, n: usize, s: f64 ) -> Matrix {
    let mut rng = rand::thread_rng(); 
    let mut matrix = vec![ vec![ 0.0; n ]; m ]; 

    let total_elements = m * n ;
    let non_zero_elements = ( s * total_elements as f64 ).ceil() as usize;

    for _ in 0..non_zero_elements { 
        let mut row = rng.gen_range( 0..m ); 
        let mut col = rng.gen_range( 0..n );

        while matrix[ row ][ col ] != 0.0 {
            row = rng.gen_range( 0..m ); 
            col = rng.gen_range( 0..n );
        }

        matrix[ row ][ col ] = rng.gen::< f64 >();
    }
    matrix
}

#[get("/generateMatrix")]
fn api_generate_sparse_matrix() -> Json<Response> {
    let matrix = generate_sparse_matrix(10, 10, 0.2); // example values
    Json(Response { matrix })
}
