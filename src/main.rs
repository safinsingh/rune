use colored::*;
use serde::Deserialize;
use serde_yaml;

use std::collections::BTreeMap;
use std::fs;
use std::process::Command;
use std::process::Stdio;

macro_rules! vprintln {
    ($v:ident, $($arg:tt)*) => {
        if $v == 1 {
            println!("{}", format!($($arg)*));
        }
    };
}

#[derive(Deserialize, Debug)]
struct Config {
    name: String,
    version: String,
    author: Option<String>,
    goals: BTreeMap<String, Action>,
}

fn info_print(v: u8, s: String) {
    vprintln!(v, "{} {}", "[INFO]".blue(), s);
}

fn error_print(v: u8, s: String) {
    vprintln!(v, "{} {}", "[ERROR]".red(), s)
}

fn success_print(v: u8, s: String) {
    vprintln!(v, "{} {}", "[PASS]".green(), s)
}

fn exec_cmd(v: u8, cmd: String) {
    let mut args = cmd.split(' ');
    let exec = args.next().unwrap();

    info_print(v, format!("Executing: {}...", format!("{}", cmd).green()));
    if let Ok(code) = Command::new(exec).args(args).stdout(Stdio::null()).status() {
        if let Some(e) = code.code() {
            if e == 0 {
                success_print(
                    v,
                    format!("Successfully executed {}!", format!("{}", cmd).green()),
                );
            } else {
                error_print(v, format!("Failed to execute {}!", cmd));
            }
        }
    }
}

impl Config {
    fn run(&self, v: u8, goal: Option<&str>) {
        info_print(
            v,
            format!(
                "{} by {} is v{}",
                self.name,
                self.author.as_ref().unwrap(),
                self.version
            ),
        );
        if let Some(g) = goal {
            info_print(v, format!("Goal {} specified. Running...", g));
            if self.goals.contains_key(g) {
                self.goals[g].act(v);
            } else {
                error_print(v, format!("Could not find goal {} in Rune.yaml!", g));
            }
        } else {
            info_print(v, "No goal specified, looking for default...".into());
            if self.goals.contains_key("default") {
                info_print(v, "Found default key!".into());
                self.goals["default"].act(v);
            } else {
                error_print(v, "Could not find default goal in Rune.yaml!".into());
            }
        }
        success_print(1, "Rune action is complete!".into());
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Action {
    Cmd(Cmd),
    CmdGroup(Vec<Cmd>),
}

impl Action {
    fn act(&self, v: u8) {
        match self {
            Self::Cmd(c) => c.exec(v),
            Self::CmdGroup(g) => {
                for c in g.into_iter() {
                    c.exec(v);
                }
            }
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Cmd {
    Raw(String),
    Detailed(DetailedCmd),
}

impl Cmd {
    fn exec(&self, v: u8) {
        match self {
            Self::Raw(r) => exec_cmd(v, r.into()),
            Self::Detailed(d) => {
                info_print(v, d.message.clone());
                d.cmd.exec(v);
            }
        }
    }
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

impl DetailedCmdAction {
    fn exec(&self, v: u8) {
        match self {
            Self::Raw(c) => exec_cmd(v, c.into()),
            Self::Multiple(i) => {
                for c in i.into_iter() {
                    exec_cmd(v, c.into());
                }
            }
        }
    }
}

fn main() {
    if let Ok(config_string) = fs::read_to_string("Rune.yaml") {
        let config: Config =
            serde_yaml::from_str(&config_string).expect("Failed to deserialze Rune.yaml!");
        let v = 0;
        let goal = Some("four");
        config.run(v, goal);
    } else {
        println!("Couldn't read Rune.yaml in the current directory!");
    }
}
