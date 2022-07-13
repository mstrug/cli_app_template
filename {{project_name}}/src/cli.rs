use clap::{Args, Parser, Subcommand, AppSettings};

#[derive(Parser)]
#[clap(author = "Emrit.io", about = "About this app...", long_about = None)]
#[clap(setting(AppSettings::NoAutoVersion))]
pub struct CliDefinition{

    #[clap(subcommand)]
    pub command: Option<Commands>,

    /// Shows application version
    #[clap(short='V', long="version")]
    pub version: bool

}

#[derive(Subcommand)]
pub enum Commands {

    /// Example command
    Example(ExampleDef)

}

#[derive(Args)]
pub struct ExampleDef {

    /// Example delay parameter
    #[clap(short='d', long)]
    pub example_delay: bool

}
