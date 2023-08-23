use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// what's your name? .... TONY
    name: Option<String>,

    /// Stop FILE time
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// DEBUG more like LE BUG, hon hon hon french buggete
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    //does test things
    /// Tests some stuff
    Test {
        #[arg(short, long)]
        list: bool,
    },
    
    /// Shall I say something?
    Speak {
        #[arg(short, long)]
        list: bool,
        /// can I put something here?
        something: String,
    },
}

fn main() {
    let cli = Cli::parse();

    //check the value provided
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    
    }

    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kinda on"),
        2 => println!("Debug mode is on"),
        _ => println!("LUDICRIOUS SPEED!"),
    }

    match &cli.command {
        Some(Commands::Test {list}) => {
            if *list {
                println!("Printin some testing lists....");
            } else {
                println!("Not printin testing lists....");
            }
        }
        Some(Commands::Speak {list, something}) => {
            if * list {
                println!("Shhhh! I am speaking! I want to say: {}!", something);
            } else {
                println!("Fine I won't say anything then, rat!");
            }
        }
        None => {}
    }
}
