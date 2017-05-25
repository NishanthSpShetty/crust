#[allow(dead_code)]

mod library;

use std::process::Command;
use std::io;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::BufReader;


use library::lexer;
use library::parser;

fn main() {

    let mut input = String::new();

    print!("Enter the C/C++ file to be converted to Rust : ");
    io::stdout().flush().ok().expect("FATAL : Buffer flush failed");
    io::stdin().read_line(&mut input).expect("Unable to read");

    let file = match File::open(String::from("./") + input.trim()) {
        Ok(f) => f,
        Err(..) => {
            println!("Error: No such file or directory!");
            std::process::exit(1);
        }
    };

    let mut strict = String::new();

    print!("Enter the translation mode [(S/s)trict/(L/l)oose] : ");
    io::stdout().flush().ok().expect("FATAL : Buffer flush failed");
    io::stdin().read_line(&mut strict).expect("Unable to read");
    let strict = strict.trim();
    let strict: bool = match &strict[..] {
        "S" | "Strict" | "s" => true,
        _ => false,
    };

    let mut cargo = String::new();
    print!("Do you want to create a cargo project :[Y/N]");
    io::stdout().flush().ok().expect("FATAL : Buffer flush failed.");
    io::stdin().read_line(&mut cargo).expect("Unable to read input");
    let cargo = cargo.trim();
    let cargo: bool = match &cargo[..] {
        "Y" | "y" => true,
        _ => false,
    };

    let mut project_name = String::new();
    if cargo == true {
        let mut project = String::new();
        print!("Enter cargo project name : ");
        io::stdout().flush().ok().expect("FATAL : Buffer flush failed");
        io::stdin().read_line(&mut project).expect("Unable to read input");
        project_name = String::from(project.trim());
    }

    // get the reader
    let mut reader = BufReader::new(&file);
    let mut text: String = String::new();
    let size = reader.read_to_string(&mut text).expect("Unable to read file.");

    println!("Input file size : {}bytes ", size);

    let mut tok = lexer::Tokenizer::new(&text);
    print!("Tokenizing");

    let mut out: Vec<String> = Vec::new();
    let tokens = tok.tokenize();
    // tok.tokenize();
    let mut ln = 0;
    for i in &tokens {
        let mut temp = i.get_token_value();
        if i.get_token_ln() != ln {
            temp = "\n".to_string() + &temp[..];
        }
        ln = i.get_token_ln();
        out.push(temp);
    }

    for _ in 0..7 {
        print!(".");
        io::stdout().flush().ok().expect("Buffer cleaning error");
        //    std::thread::sleep(std::time::Duration::from_millis(500));

    }

    //    file.write_all(output.as_bytes()).expect("Unable to write to file");

    println!("\t:DONE");
    print!("Invoking Parser .");

    for _ in 0..7 {
        print!(".");
        io::stdout().flush().ok().expect("Buffer cleaning error");
        //    std::thread::sleep(std::time::Duration::from_millis(600));

    }
    let rust_lexeme = parser::init_parser(&tokens, strict);
    //regenerate the code from lexemes
    let mut o: String = String::new();
    for i in rust_lexeme {
        o = o + " ";
        o = o + &i[..];
    }

    println!("\t:DONE");
    let mut fname = PathBuf::from(input);
    fname.set_extension("rs");

    if cargo == true {
        let child = Command::new("cargo")
            .args(&["new", "--bin"])
            .arg(&project_name[..])
            .status()
            .expect("Failed to create project");
        if child.code().unwrap() == 101 {
            println!("Project already exist with the name : {}, it will be overwritten by the `crust`.",
                     project_name);

            fname = PathBuf::from(project_name.clone() + "/src/main.rs");
        }
        if child.success() {
            fname = PathBuf::from(project_name.clone() + "/src/main.rs");
        }
        println!("child code {} ", child.code().unwrap());
    }

    let mut file = File::create(&fname).expect("Unable to open file to write");
    file.write_all(o.as_bytes()).expect("Unable to write to file");
    Command::new("rustfmt")
        .arg("--")
        .arg(&fname)
        .output()
        .expect("Failed to format the translated code");
    println!("Rust equivalent of source of `{}` is generated successfully, View the rust code in \
              file : `{}`",
             input.trim(),
             fname.display());
}
