use std::env;
use clap::Parser;
use tokio::time::Duration;

mod config;
use config::*;

mod cli;
use cli::*;


const VERSION: &str = env!("CARGO_PKG_VERSION");


#[tokio::main]
async fn main()
{
    // parse command line arguments and handle appropriate action
    let cli = CliDefinition::parse();
    handle_cli_commands( &cli ).await;
}

async fn handle_cli_commands(cli: &CliDefinition)
{
    if cli.version 
    {
        // show application version
        println!("{{project_name}} v{}", VERSION);
        println!("  Build timestamp: {}", env!("VERGEN_BUILD_TIMESTAMP"));
        println!("  Git info: {}  {}", env!("VERGEN_GIT_SHA_SHORT"), env!("VERGEN_GIT_COMMIT_TIMESTAMP"));
    }
    else 
    {
        // handle actions for specified CLI commands
        match &cli.command {
            Some(Commands::Example(cmd)) => {
                println!("Example command {}, {:?}", CONFIG.example_value_1, CONFIG.example_value_2);

                if cmd.example_delay {
                    println!(" delay enabled");
                    tokio::time::sleep( Duration::from_millis(1000) ).await 
                }
            }
            None => println!("No command specified")
        }
    }
}

