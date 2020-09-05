use serde::Deserialize;
use serde_yaml;

use std::collections::BTreeMap;
use std::fs;

#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    version: String,
    description: Option<String>,
    author: Option<String>,
    goals: BTreeMap<String, Action>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Action {
    Cmd(Cmd),
    CmdGroup(Vec<Cmd>),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Cmd {
    Raw(String),
    Detailed(DetailedCmd),
}

#[derive(Deserialize, Debug)]
struct DetailedCmd {
    message: String,
    cmd: DetailedCmdAction,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum DetailedCmdAction {
    Raw(String),
    Multiple(Vec<String>),
}

fn main() {
    if let Ok(config_string) = fs::read_to_string("Rune.yaml") {
        let config: Config =
            serde_yaml::from_str(&config_string).expect("Failed to deserialze Rune.yaml!");
        println!("{:?}", config);
    } else {
        println!("Couldn't read Rune.yaml in the current directory!");
    }
}
