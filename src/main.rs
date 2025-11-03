use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Eth {
        #[clap(subcommand)]
        subcommand: EthSubcommands,
    },

    Solana {
        #[clap(subcommand)]
        subcommand: SolanaSubcommands,
    }
}

#[derive(Subcommand, Debug, Clone)]
enum EthSubcommands {
    Selector {
        #[clap(value_name = "FUNCTION_SIGNATURE")]
        function_signature: String,
    },

    Address {
        #[clap(value_name = "ADDRESS")]
        private_key: String,
    }
}

#[derive(Subcommand, Debug, Clone)]
enum SolanaSubcommands {
    Address {
        #[clap(value_name = "PRIVATE_KEY")]
        private_key: String,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Eth { subcommand} => {
            match subcommand {
                EthSubcommands::Selector { function_signature } => {
                    println!("fn: {}", function_signature);
                }

                EthSubcommands::Address { private_key } => {
                    println!("private_key: {}", private_key);
                }

            }

        }

        Commands::Solana {subcommand} => {
            match subcommand {
                SolanaSubcommands::Address { private_key } => {
                    println!("private_key: {}", private_key);
                }
            }
        }
    }
}
