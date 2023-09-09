use clap::{Args, Parser, Subcommand};
use std::{
    env::{self, var_os},
    ffi::OsString,
    fs::{self, remove_dir_all},
    io::Write,
    path::Path,
    process::Command,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None,propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Init nb
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
            let working_dir_string = get_working_dir_path();
            let path = Path::new(&working_dir_string);
            init(path);
        }
        Commands::Add(add_args) => {
            println!("{:?}", add_args.id);
            let editor_path = get_editor_path();
            Command::new(editor_path)
                .arg("absolute/path/to/mytestfile.txt")
                .status()
                .expect("Something went wrong.");
            // Command::new("$EDITOR")
            //     .status()
            //     .expect("ls command failed to start");
            std::process::exit(0);
        }
        Commands::Edit(id) => {
            println!("'myapp add' was used, name is: {:?}", id.id)
        }
        Commands::Reset(_) => {
            let working_dir_string = get_working_dir_path();
            let path = Path::new(&working_dir_string);
            reset(path);
            init(path);
        }
    }
}

static DEFAULT_VALUE_DIR: &str = "~/.rnb";

fn get_working_dir_path() -> OsString {
    match env::var_os("RNB_DIR") {
        Some(rnb_dir) => rnb_dir,
        None => {
            let mut default_dir = OsString::new();
            default_dir.push(DEFAULT_VALUE_DIR);
            default_dir
        }
    }
}

fn reset(path: &Path) {
    let owned_path = path.to_owned();
    let copy_path = owned_path.to_str().unwrap();
    let path_string: String = if copy_path.starts_with('~') {
        tilde_expand(copy_path)
    } else {
        path.to_owned().to_str().unwrap().to_string()
    };
    let path = Path::new(&path_string);

    match remove_dir_all(path) {
        Ok(_) => println!("The rnb folder under {:?} has been deleted", path),
        Err(err) => println!("Error while reseting, {:?}", err),
    };
}

fn init(path: &Path) {
    match path.exists() {
        true => {
            println!("The path already exists {:?}", path.to_str().unwrap());
        }
        false => {
            println!("Initing rnb...{:?}", path);
            match fs::create_dir_all(path) {
                Ok(_) => println!("Done initailzing rnb"),
                Err(err) => println!("Error while inidtilzint, {:?}", err),
            };
            let _ = fs::create_dir_all(path.join("home"));
            let mut current_file = fs::File::create(path.join(".current")).unwrap();
            let _ = current_file.write_all("home\n".as_bytes());
            let home_path_string = get_home_path();
            let home_path = Path::new(&home_path_string);
            let mut rc_file = fs::File::create(home_path.join(".rnbrc")).unwrap();
            let _ = rc_file.write_all("".as_bytes());
        }
    };
}

fn get_editor_path() -> OsString {
    env::var_os("EDITOR").unwrap()
}

// fn get_current_notebook() {
//     let home_path = get_home_path();
// }
//
// fn get_rc_file() {
//     let home_path = get_home_path();
// }

fn get_path_string(path: &String) -> String {
    let path_string: String = if path.starts_with('~') {
        tilde_expand(path)
    } else {
        path.to_owned()
    };
    path_string
}

fn get_home_path() -> String {
    var_os("HOME").unwrap().into_string().unwrap()
}

fn tilde_expand(path: &str) -> String {
    let mut owned_path = path.to_owned();
    owned_path.remove(0);
    let home_path: String = var_os("HOME").unwrap().into_string().unwrap();
    home_path + &owned_path
}
