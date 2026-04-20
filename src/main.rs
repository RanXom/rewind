use clap::{ Parser, Subcommand };

#[derive(Parser)]
#[command(name = "rw")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Log {
        #[arg(short, long)]
        command: String,

        #[arg(short, long)]
        exit: i64,

        #[arg(short, long)]
        cwd: String,

        #[arg(short, long)]
        start: i64
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Log { command, exit, cwd, start } => {
            println!("rw debug:");
            println!("Captured: \"{}\"", command);
            println!("Status: {}", if *exit == 0 { "finish with 0" } else { "failure" });
            println!("Working dir: {}", cwd);
            println!("Started: {}", start);
        }
    }
}
