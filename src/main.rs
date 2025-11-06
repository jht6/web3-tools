mod eth;
mod common;

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
    },

    Common {
        #[clap(subcommand)]
        subcommand: CommonSubcommands,
    },
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

#[derive(Subcommand, Debug, Clone)]
enum CommonSubcommands {
    HexToBs64 {
        #[clap(value_name = "HEX_STRING")]
        hex_str: String,
    },

    HexToBs58 {
        #[clap(value_name = "HEX_STRING")]
        hex_str: String,
    },

    Bs64ToHex {
        #[clap(value_name = "BASE64_STRING")]
        bs64_str: String,
    },

    Bs64ToBs58 {
        #[clap(value_name = "BASE64_STRING")]
        bs64_str: String,
    },

    Bs58ToHex {
        #[clap(value_name = "BASE58_STRING")]
        bs58_str: String,
    },

    Bs58ToBs64 {
        #[clap(value_name = "BASE58_STRING")]
        bs58_str: String,
    },
    
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Eth { subcommand} => {
            match subcommand {
                EthSubcommands::Selector { function_signature } => {
                    eth::selector::handle_selector(function_signature);
                }

                EthSubcommands::Address { private_key } => {
                    eth::address::handle_address(private_key);
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

        Commands::Common { subcommand } => {
            match subcommand {
                CommonSubcommands::HexToBs64 { hex_str } => {
                    common::encode::handle_hex_to_bs64(hex_str);
                }

                CommonSubcommands::HexToBs58 { hex_str } => {
                    common::encode::handle_hex_to_bs58(hex_str);
                }

                CommonSubcommands::Bs64ToHex { bs64_str } => {
                    common::encode::handle_bs64_to_hex(bs64_str);
                }

                CommonSubcommands::Bs64ToBs58 { bs64_str } => {
                    common::encode::handle_bs64_to_bs58(bs64_str);
                }

                CommonSubcommands::Bs58ToHex { bs58_str } => {
                    common::encode::handle_bs58_to_hex(bs58_str);
                }

                CommonSubcommands::Bs58ToBs64 { bs58_str } => {
                    common::encode::handle_bs58_to_bs64(bs58_str);
                }
            }
        }
    }
}
