use clap::Parser;
use color_eyre::Result;

pub fn parse() -> Result<Cli> {
    Ok(Cli::parse())
}

#[derive(Debug, Parser)]
pub struct Cli {
    /// Sets bind address
    #[arg(default_value = "0.0.0.0")]
    pub address: String,
    /// Listening port
    #[arg(default_value_t = 3000)]
    pub port: u16,
    /// Sets number of seconds to live (indefinite if omitted)
    pub seconds: Option<u16>,
}
