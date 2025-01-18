use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;
use log::{error, info};
use proof_generation::stark_proof;

/// Handles `/stark-proof/{secret}` requests.
///
/// Accepts a secret as a URL parameter and attempts to generate a proof for it.
/// Logs request details and proof generation results. Returns the proof or an error response.
///
/// # Parameters
/// - `path`: URL path parameter containing the secret.
///
/// # Returns
/// - `HttpResponse` with the generated proof as JSON if successful.
/// - `HttpResponse` with an error message if the proof generation fails.
#[get("/stark-proof/{secret}")]
async fn stark_proof_handler(path: web::Path<String>) -> impl Responder {
    let secret = path.into_inner();

    // Get the current timestamp for logging.
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    info!("[{}] Received request for secret: {}", timestamp, secret);

    // Generate the proof for the given secret.
    match stark_proof::generate_proof_of_secret(&secret) {
        Ok(proof) => {
            // Log success if the proof is available.
            info!(
                "[{}] Successfully retrieved proof for secret: {}",
                timestamp, secret
            );
            HttpResponse::Ok().json(proof)
        }
        Err(err) => {
            // Log failure and return an error response.
            error!(
                "[{}] Proof generation failed for secret: {}. Error: {}",
                timestamp, secret, err
            );
            HttpResponse::InternalServerError().body(format!("Error generating proof: {}", err))
        }
    }
}

/// The entry point of the application.
///
/// This function initializes the proof generation server and starts it at `http://127.0.0.1:8090`.
/// It sets up a single service for handling `/stark-proof/{secret}` requests.
///
/// Logs:
/// - Logs the server start-up information.
///
/// # Returns
/// - A `Result` indicating whether the server started successfully or encountered an error.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger for capturing logs.
    env_logger::init();

    // Log server start-up.
    info!("Starting server at http://127.0.0.1:8090");

    // Configure and run the Actix Web server.
    HttpServer::new(move || {
        App::new()
            // Register the handler for `/stark-proof/{secret}`.
            .service(stark_proof_handler)
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
