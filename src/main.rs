use clap::Parser;
use rand::Rng;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 16)]
    length: i32,
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
    println!("Generated password: {}", generate_password(args.length));
}
