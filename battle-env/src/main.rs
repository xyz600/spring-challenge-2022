// Note: this requires the `derive` feature

use clap::Parser;
use lib::GameBoard;
use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

// bot implementation

struct BotPlayer {
    exec_name: String,
    process: std::process::Child,
}

impl BotPlayer {
    fn new(exec_name: &String) -> BotPlayer {
        BotPlayer {
            exec_name: exec_name.clone(),
            process: match Command::new(exec_name)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
            {
                Err(why) => panic!("can't spawn exec {}", exec_name),
                Ok(process) => process,
            },
        }
    }

    /// 問題情報を info として送信する
    fn send(&self, info: Vec<String>) {
        let content = info.into_iter().map(|s| s + "\n").collect::<Vec<String>>().join("");
        self.process
            .stdin
            .as_ref()
            .unwrap()
            .write_all(content.as_bytes())
            .expect("cannot send input through stdin");
    }

    /// Bot が出力した情報を受け取る
    fn recieve(&mut self) -> Vec<String> {
        let mut content = String::new();
        self.process
            .stdout
            .as_mut()
            .unwrap()
            .read_to_string(&mut content)
            .expect("cannot recieve output through stdout");

        content
            .split("\n")
            .take(3) // print for each hero
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(long)]
    exec1: String,

    /// Number of times to greet
    #[clap(long)]
    exec2: String,

    #[clap(long)]
    seed: u64,
}

fn main() {
    let args = Args::parse();
    eprintln!("battle '{}' vs '{}'", args.exec1, args.exec2);

    let bot1 = BotPlayer::new(&args.exec1);
    let bot2 = BotPlayer::new(&args.exec2);

    let mut game = GameBoard::new(args.seed);
}
