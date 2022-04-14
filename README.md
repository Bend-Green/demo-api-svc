# demo-api-svc
The finished source code for our tutorial: [Pedal to the Metal with PlanetScale and Rust](https://bend.green/blog/pedal-to-the-metal-with-planetscale-and-rust)

## Dependencies
To run this demo locally, you'll need the following:
* [Rust](https://www.rust-lang.org/)
* [Diesel](https://diesel.rs/)
* [MySQL](https://dev.mysql.com/doc/refman/8.0/en/installing.html)

## Development
1. Clone the repository
2. Copy the example.env file to .env and replace it with your local or remote MySQL DATABASE_URL
3. In the directory, run `diesel setup` to initialize the database
4. in the directory, enter `cargo run` to start the server
