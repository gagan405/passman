use clap::{Parser, Subcommand};
use rand::Rng;
use rusqlite::{Connection, Result};
use homedir::get_my_home;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug)]
struct Entry {
    id: i32,
    name: String,
    pw: String,
    url: String,
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

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Add { uname, site_url, pw } => {
            println!("Adding password details: name: {:?}, url: {}, pw: {}", uname, site_url, pw);
            persist(uname, site_url, pw);
        }
        Commands::Generate { length, file: _ } => {
            println!("Generated password: {}", generate_password(length));
        }
    }

    Ok(())
}

fn persist(uname: String, site_url: String, pword: String) -> Result<()> {
    let home = get_my_home().unwrap().unwrap();
    let home_path = home.as_path();
    let abs_path = home_path.display().to_string() + "/.passman/passman.db";
    let conn = Connection::open(&abs_path)?;

    conn.execute(
        "create table if not exists entries (
             id integer primary key,
             name text not null,
             pw text not null,
             url text not null unique,
             hints text
         )",
        (),
    )?;

    conn.execute(
        "INSERT INTO entries (name, pw, url) VALUES (?1, ?2, ?3)",
        (&uname, &pword, &site_url),
    )?;

    Ok(())
}
