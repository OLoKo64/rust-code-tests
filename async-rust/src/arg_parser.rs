use clap::Parser;

/// Make tons of simultaneous requests to a server.
#[derive(Parser, Clone)]
#[clap(version)]
pub struct Args {
    /// Total number of requests to be sent.
    #[clap(short, long, value_parser, default_value_t = 5000)]
    pub request_number: u32,

    /// Pool size to be used to make the requests.
    /// Maximum value recommended is 8000.
    #[clap(short, long, value_parser, default_value_t = 5000)]
    pub pool_size: u32,

    /// URL to be used to make the requests.
    /// Example: https://www.example.com
    #[clap(short, long, value_parser)]
    pub url: String,
}
