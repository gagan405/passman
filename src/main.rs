use clap::{Parser, Subcommand};
use rand::Rng;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        #[arg(short, long, default_value_t = 16)]
        length: i32,

        #[arg(short, long, default_value_t = String::new())]
        file: String,
    },
    Add {
        #[arg(short, long)]
        uname: String,

        #[arg(short, long)]
        site_url: String,

        #[arg(short, long)]
        pw: String,
    },
}

static CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789$&!@#";

fn generate_password(length: i32) -> String {
    let mut rng = rand::thread_rng();
    let charset: Vec<char> = CHARACTERS.chars().collect();
    let password: String = (0..length)
        .map(|_| rng.gen_range(0..charset.len()))
        .map(|idx| charset[idx])
        .collect();
    password
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Add { uname, site_url, pw } => {
            println!("Adding password details: name: {:?}, url: {}, pw: {}", uname, site_url, pw);
        }
        Commands::Generate { length, file: _ } => {
            println!("Generated password: {}", generate_password(length));
        }
    }
}
