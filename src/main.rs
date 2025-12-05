#[cfg(feature = "server")]
mod server_runner;

// #[cfg(feature = "server")]
// mod server;

#[cfg(feature = "client")]
mod client_runner;

#[cfg(feature = "client")]
mod client;

fn main() {
    #[cfg(feature = "server")]
    server_runner::init();

    #[cfg(feature = "client")]
    client_runner::init();
}
