use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mahjong :)", version, about = "riichi :D")]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCli,
}

#[derive(Subcommand)]
enum SubCli {
    Server {port: u16},
    Client {ip: String, port: u16},
}

fn main() {
    let cli = Cli::parse();

    match &cli.subcommand {
        SubCli::Server { port } => {
            println!("server time! port is {port}");
        }, 
        SubCli::Client {ip, port} => { 
            todo!();
        },
    }
}