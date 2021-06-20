#![allow(dead_code)]

use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use getopts::Options;

use library::lexer::tokenizer::Tokenizer;
use library::parser::parser;

mod library;

struct Settings {
    strict: bool,
    project_name: Option<String>,
    files: Vec<String>,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("s", "strict", "Strict mode (immutable)");
    opts.optopt("p", "project-name", "Cargo project name", "NAME");
    opts.optflag("h", "help", "show this help message");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let settings = if matches.free.is_empty() {
        get_settings_interactively()
    } else {
        Settings {
            strict: matches.opt_present("s"),
            project_name: matches.opt_str("p"),
            files: matches.free,
        }
    };

    invoke(&settings);
}

fn get_settings_interactively() -> Settings {
    let mut input = String::new();

    print!("Enter the C/C++ file to be converted to Rust : ");
    io::stdout().flush().expect("FATAL : Buffer flush failed");
    io::stdin().read_line(&mut input).expect("Unable to read");

    let mut strict = String::new();

    print!("Enter the translation mode [(S/s)trict/(L/l)oose] : ");
    io::stdout().flush().expect("FATAL : Buffer flush failed");
    io::stdin().read_line(&mut strict).expect("Unable to read");
    let strict = strict.trim();
    let strict = matches!(strict, "S" | "Strict" | "s");

    let mut cargo = String::new();
    print!("Do you want to create a cargo project :[Y/N]");
    io::stdout().flush().expect("FATAL : Buffer flush failed.");
    io::stdin()
        .read_line(&mut cargo)
        .expect("Unable to read input");
    let cargo = cargo.trim();
    let cargo: bool = matches!(cargo, "Y" | "y");

    let mut project_name = None;
    if cargo {
        let mut project = String::new();
        print!("Enter cargo project name : ");
        io::stdout().flush().expect("FATAL : Buffer flush failed");
        io::stdin()
            .read_line(&mut project)
            .expect("Unable to read input");
        project_name = Some(String::from(project.trim()));
    }

    Settings {
        strict,
        project_name,
        files: vec![input.trim().to_owned()],
    }
}

fn invoke(settings: &Settings) {
    for input in settings.files.iter() {
        let file = match File::open(input) {
            Ok(f) => f,
            Err(err) => {
                println!("Unable to open input source file '{}': {}.", input, err);
                std::process::exit(1);
            }
        };
        // get the reader
        let mut reader = BufReader::new(&file);
        let mut text: String = String::new();
        let size = reader
            .read_to_string(&mut text)
            .expect("unable to read file.");

        println!("Input file size : {}bytes ", size);

        let tok = Tokenizer::new(&text);
        print!("Tokenizing");

        let tokens = tok.tokenize();
        //let mut out: Vec<String> = Vec::new();
        // tok.tokenize();
        // let mut ln = 0;
        // for token in &tokens {
        //     let mut token_value = token.get_token_value();
        //     print!("{:?}\n",token);
        //     if token.get_token_line_num() != ln {
        //         token_value = "\n".to_string() + &token_value[..];
        //     }
        //     ln = token.get_token_line_num();
        //     out.push(token_value);
        // }

        println!("Invoking Parser....");

        let mode = if settings.strict { "Strict" } else { "Loose" };
        let rust_lexeme = parser::init_parser(&tokens, settings.strict);
        //regenerate the code from lexemes
        let mut o: String = String::new();
        for i in rust_lexeme {
            o += " ";
            o += &i[..];
        }

        let mut fname = PathBuf::from(input);

        fname.set_extension("rs");

        if let Some(ref project_name) = settings.project_name {
            let child = Command::new("cargo")
                .args(&["new", "--bin"])
                .arg(&project_name[..])
                .status()
                .expect("Failed to create project");
            if child.code().unwrap() == 101 {
                println!(
                    "Project already exist with the name : {}, it will be overwritten by \
                          the `crust`.",
                    project_name
                );

                fname = PathBuf::from(project_name.clone() + "/src/main.rs");
            }
            if child.success() {
                fname = PathBuf::from(project_name.clone() + "/src/main.rs");
            }
            println!("child code {} ", child.code().unwrap());
        }

        let mut file = File::create(&fname).expect("Unable to open file to write");
        file.write_all(o.as_bytes())
            .expect("Unable to write to file");
        Command::new("rustfmt")
            .arg("--")
            .arg(&fname)
            .output()
            .expect("Failed to format the translated code");
        println!(
            "Rust equivalent of source of `{}` in [{} mode ], is generated successfully, \n\
		View the rust code in file : `{}`",
            input.trim(),
            mode,
            fname.display()
        );
    }
}
