use clap::{Args, Parser, Subcommand};
use std::{
    env::{self},
    ffi::OsString,
    fs::{self, remove_dir_all},
    path::Path,
    process::{self},
};

use colored::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None,propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initalize rnb
    Init(InitArgs),
    /// Add note to your notebook
    Add(AddArgs),
    /// Edit note
    Edit(EditArgs),
    /// DANGER: don't do that!
    Reset(ResetArgs),
}

#[derive(Args)]
struct InitArgs {}

#[derive(Args)]
struct AddArgs {
    id: i32,
}

#[derive(Args)]
struct EditArgs {
    id: i32,
}

#[derive(Args)]
struct ResetArgs {}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init(_) => {
            let home_dir_path = get_home_path();
            init(&home_dir_path);
        }
        Commands::Add(add_args) => {
            println!("{:?}", add_args.id);
            let editor_path = get_editor_path();
            println!("{:?}", editor_path);
            process::exit(0);
        }
        Commands::Edit(id) => {
            println!("'myapp add' was used, name is: {:?}", id.id)
        }
        Commands::Reset(_) => {
            let home_dir_path = get_home_path();
            reset(&home_dir_path);
            init(&home_dir_path);
        }
    }
}

static DEFAULT_VALUE_DIR: &str = ".rnb";
static DEFAULT_RC_FILE: &str = ".rnbrc";

fn reset(home_dir_path: &OsString) {
    let working_dir_path = Path::new(&home_dir_path).join(DEFAULT_VALUE_DIR);
    let rc_path = Path::new(&home_dir_path).join(DEFAULT_RC_FILE);

    match remove_dir_all(&working_dir_path) {
        Ok(_) => {
            println!(
                "The rnb folder under {} has been deleted",
                &working_dir_path.to_str().unwrap().red()
            )
        }
        Err(err) => println!("Error while reseting, {}", err.to_string().red()),
    };
    match fs::remove_file(&rc_path) {
        Ok(_) => println!(
            "The rnb rc file under {} has been deleted",
            &rc_path.to_str().unwrap().red()
        ),
        Err(err) => println!("Error while reseting, {}", err.to_string().red()),
    };
}

fn init(home_dir_path: &OsString) {
    let working_dir_path = Path::new(&home_dir_path).join(DEFAULT_VALUE_DIR);
    let rc_path = Path::new(&home_dir_path).join(DEFAULT_RC_FILE);

    if working_dir_path.exists() {
        eprintln!("{}", "working dir already excists".red());
        process::exit(1)
    };

    if rc_path.exists() {
        eprintln!("{}", ".rnbrc file already excists".red());
        process::exit(1)
    };

    match std::fs::create_dir_all(&working_dir_path) {
        Ok(_) => println!(
            "Created working dir {}",
            &working_dir_path.to_str().unwrap().green()
        ),
        Err(err) => {
            eprintln!("{}", err.to_string().red());
            process::exit(1)
        }
    };

    match std::fs::File::create(&rc_path) {
        Ok(_) => println!("Created rc file {}", &rc_path.to_str().unwrap().green()),
        Err(err) => {
            eprintln!("{:?}", err);
            process::exit(1)
        }
    };
}

fn get_editor_path() -> Option<OsString> {
    env::var_os("EDITOR").and_then(|e| {
        if e.is_empty() {
            eprintln!("{}", "Can Not find Editor".red());
            process::exit(1)
        } else {
            Some(e)
        }
    })
}

// fn get_current_notebook() {
//     let home_path = get_home_path();
// }
//
// fn get_rc_file() {
//     let home_path = get_home_path();
// }

// fn get_path_string(path: &String) -> String {
//     let path_string: String = if path.starts_with('~') {
//         tilde_expand(path)
//     } else {
//         path.to_owned()
//     };
//     path_string
// }

fn get_home_path() -> OsString {
    env::var_os("HOME").unwrap_or_else(|| {
        eprintln!("Could not determine home dir");
        process::exit(1);
    })
}

// fn tilde_expand(path: &str) -> String {
//     let mut owned_path = path.to_owned();
//     owned_path.remove(0);
//     let home_path: String = var_os("HOME").unwrap().into_string().unwrap();
//     home_path + &owned_path
// }
