use std::{net::SocketAddr, path::PathBuf, process::ExitCode};

use clap::{Subcommand, Args, Parser};
use tracing::error;


const LOGO: &str = r#"
 _   _ _             __   __            _    
| \ | (_)            \ \ / /           | |    
|  \| |_ _ __   ___   \ V /__ _ _ __ __| |___ 
| . ` | | '_ \ / _ \   \ // _` | '__/ _` / __|
| |\  | | | | |  __/   | | (_| | | | (_| \__ \
\_| \_/_|_| |_|\___|   \_/\__,_|_|  \__,_|___/
"#;

const INFO: &str = "
To get started using Nine Yards have a look at the documentation
on GitHub https://github.com/Rabbitminers/Nine-Yards/wiki.

If you find a bug or an issue please report it on GitHub
https://github.com/Rabbitminers/Nine-Yards/issues

We would appreciate if you could start the repository
(https://github.com/Rabbitminers/Nine-Yards)

------------
";

#[derive(Parser, Debug)]
#[command(name = "Nine Yards command-line interface and server", bin_name = "nineyards")]
#[command(about = INFO, before_help = LOGO)]
#[command(disable_version_flag = true, arg_required_else_help = true)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Start the server")]
    Start(StartCommandArguments),
    
    #[command(about = "Generate an OpenApi schema")]
    Schema(OpenApiSchemaArguements)
}

#[derive(Args, Debug)]
pub struct StartCommandArguments {
    #[arg(help = "The hostname or ip address to listen for connections on")]
	#[arg(env = "BIND_ADDR", short = 'b', long = "bind")]
	#[arg(default_value = "0.0.0.0:3000")]
	pub listen_address: SocketAddr,

    #[arg(help = "The location of your sqlite database")]
    #[arg(env = "DATABASE_URL", short = 'd', long = "database_url")]
    #[cfg_attr(feature = "sqlite", arg(default_value = "sqlite://database.db"))]  
    #[cfg_attr(feature = "postgres", arg(default_value = "postgresql://localhost"))]
    pub database_url: String,
}

#[derive(Args, Debug)]
pub struct OpenApiSchemaArguements {
    #[arg(help = "Location to store generated openapi schema")]
    #[arg(short = 'o', long = "openapi_schema_location")]
    #[arg(default_value = "openapi.json")]
    pub output_location: PathBuf, 
}

pub async fn init() -> ExitCode {
    let args = Cli::parse();

    let output = match args.command {
        Commands::Start(args) => crate::api::init(args).await,
        Commands::Schema(args) => crate::openapi::write(args).await,
    };

	if let Err(e) = output {
		error!("{}", e);
		ExitCode::FAILURE
	} else {
		ExitCode::SUCCESS
	}

}