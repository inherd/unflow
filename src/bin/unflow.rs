use std::{env, fs};
use std::path::Path;
use unflow::parse;

pub mod language_server;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("cannot find cmd. You can try
1. lsp
2. convert
");
        return;
    }

    let query = &args[1];
    if query == "lsp" {
        language_server::start();
    } else if query == "convert" {
        if args.len() < 3 {
            println!("lost file name");
            return;
        }
        let filename = &args[2];
        match fs::read_to_string(Path::new(filename)) {
            Ok(content) => {
                let unflow = parse(content.as_str());
                let string = serde_json::to_string(&unflow).unwrap();
                let _ = fs::write("unflow.json", string);
            }
            Err(_) => {
                println!("cannot find file name: {:?}", filename);
            }
        };
    }
}