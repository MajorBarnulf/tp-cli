use std::{collections::HashMap, env, fs, io};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
pub struct Params {
    #[clap(short)]
    setup: bool,
    /// name of the location, create one by prefixing it by a '+'
    name: String,
}

fn main() {
    let arguments: Params = Params::parse();
    if !is_alias_setup(&arguments) {
        setup_alias();
    } else {
        if is_new_location_operation(&arguments) {
            new_location_operation(&arguments);
        } else {
            to_location_operation(&arguments);
        }
    }
}

fn local_path() -> String {
    env::var("HOME").unwrap() + "/.local/tp-cli"
}

fn is_alias_setup(arguments: &Params) -> bool {
    let path = local_path() + "/alias.sh";
    let result = fs::read(path);
    arguments.setup && result.is_ok()
}

fn setup_alias() {
    let result = fs::create_dir(local_path());
    if let Err(error) = &result {
        if error.kind() == io::ErrorKind::AlreadyExists {
            // ignore this error
        } else {
            result.unwrap();
        }
    };

    let path = local_path() + "/alias.sh";
    fs::write(&path, include_str!("alias.sh")).unwrap();

    println!(
        "
TP-CLI needs the environment to be setup in order to work correctly
please add the following lines to your .rc file:

# initialize TP-CLI
source \"{path}\"

then, you will be able to use it with the 'tp' command."
    );
}

fn is_new_location_operation(arguments: &Params) -> bool {
    arguments.name.chars().into_iter().nth(0).unwrap() == '+'
}

#[derive(Default, Serialize, Deserialize)]
pub struct Storage {
    entries: HashMap<String, String>,
}

fn read_storage() -> Storage {
    let path = local_path() + "/storage.ron";
    fs::read_to_string(path)
        .map(|content| ron::from_str(&content).unwrap())
        .unwrap_or(Storage::default())
}

fn write_storage(storage: Storage) {
    let serialized = ron::to_string(&storage).unwrap();
    let path = local_path() + "/storage.ron";
    fs::write(path, serialized).unwrap();
}

fn new_location_operation(arguments: &Params) {
    let name = &arguments.name[1..];
    let path = env::current_dir().unwrap();
    let mut storage = read_storage();
    storage
        .entries
        .insert(name.into(), path.to_str().unwrap().into());
    write_storage(storage);
}

fn to_location_operation(arguments: &Params) {
    let storage = read_storage();
    let path = storage.entries.get(&arguments.name).unwrap();
    println!("{path}");
}
