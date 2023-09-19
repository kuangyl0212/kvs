use clap::{Parser, Subcommand};
use kvs::KvStore;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() {
    let cli = Cli::parse();
    let mut store = KvStore::new();
    match &cli.command {
        Commands::Get { key } => {
            // store.get(key.to_owned());
            panic!("unimplemented");
        }
        Commands::Set { key, value } => {
            // store.set(key.to_owned(), value.to_owned());
            panic!("unimplemented");
        }
        Commands::Rm { key } => {
            // store.remove(key.to_owned());
            panic!("unimplemented");
        }
    }
}
