use store::Store;
use clap::{ Parser, Subcommand};
use colored::Colorize;
mod store;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    
    #[clap(subcommand)]
    subcmd: SubCommands,
 
}
#[derive(Debug, Subcommand)]
enum SubCommands {
    Check, 
    Add{
        #[clap(short, long)]
        note: String,
    },
    Remove
}

fn main() {
    let args = Args::parse();
    let storage = store::Storage::new(None);
 
    let cwd = std::env::current_dir().unwrap();

    match args.subcmd {
        SubCommands::Check => {
            let value = storage.get(cwd.to_str().unwrap());
            match value {
                Some(v) => println!("{}: {}", cwd.to_str().unwrap(), v),
                None => {
                    // Do nothing
                }
            }
        },
        SubCommands::Add {note} => {
            let set = storage.set(cwd.to_str().unwrap(), note.as_str());
            match set {
                Ok(_) => {
                    println!("{}:{}","Note added for:".blue(),cwd.to_str().unwrap())
                },
                Err(e) => println!("{}: {}", cwd.to_str().unwrap(), e)
            }
        },
        SubCommands::Remove => {
            let remove = storage.remove(cwd.to_str().unwrap());
            match remove {
                Ok(_) => println!("{}:{}","Note removed for:".blue(), cwd.to_str().unwrap(),),
                Err(_e) => {}
            }
        }
    }
}
