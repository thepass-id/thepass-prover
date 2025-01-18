# proof_generation

This project implements a web server that provides STARK proof generation for a given secret. The server uses Actix Web to handle HTTP requests and logs details of each request and the corresponding proof generation process.

## Features:
- **STARK Proof Generation**: Accepts a secret and generates a STARK proof for it.
- **Logging**: Logs incoming requests, successful proof generation, and errors.
- **Error Handling**: Returns an error response if proof generation fails.

## Prerequisites

Before running the server, make sure you have the following installed:

- **Rust** (version 1.60.0 or later): [Rust Installation Guide](https://www.rust-lang.org/tools/install)
- **Cargo**: This is automatically installed with Rust.

## Setup and Installation

### 1. Clone the Repository

Clone the repository to your local machine:

```sh
git clone https://your-repository-url.git
cd proof_generation
```
### 2. Install Dependencies
Install the necessary dependencies and build the project using Cargo:

```sh
cargo build
```
This command will compile the project and fetch all required dependencies specified in `Cargo.toml`.  

### 3. Run the Server
To start the web server, run the following command:

```sh
cargo run
```
The server will now be running at http://127.0.0.1:8090.
### 4. Log Level Configuration
To adjust the log level, you can set the `RUST_LOG` environment variable before running the server. For example, to enable debug-level logging, run the following:

```sh
RUST_LOG=debug cargo run
```

## Example Requests
###  1. Generate Proof for a Secret
To generate a STARK proof for a given secret, send a GET request to the `/stark-proof/{secret}` endpoint, replacing `{secret}` with your secret.

#### Request:

```sh
curl http://127.0.0.1:8090/stark-proof/mysecret123
```
#### Response:  

If proof generation is successful, you will receive a JSON response containing the generated proof:

```json
{
  "proof": "generated_proof_data_here"
}
```
If there is an error during the proof generation process, an error response will be returned:

```json
{
  "error": "Error generating proof: <error_message>"
}
```
#### Example:

```sh
curl http://127.0.0.1:8090/stark-proof/mysecret123
```
#### Response:

```json
{
  "proof": "example_generated_proof_data"
}
```
If an error occurs, the response will look like:

```json
{
  "error": "Error generating proof: <error_message>"
}
```

