use std::env::set_current_dir;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::process::{Command, Stdio};

#[derive(Debug)]
struct Funcs {
    args: Vec<String>,
}

impl Funcs {
    fn new(args: Vec<String>) -> Self {
        Funcs { args: args }
    }

    fn cd(&self) -> i32 {
        if self.args.len() == 0 {
            panic!("No Arguments!")
        }
        let p = Path::new(&self.args[1]);
        set_current_dir(&p).unwrap();
        1
    }

    fn help(&self) -> i32 {
        println!("HELP!");
        1
    }

    fn exit(&self) -> i32 {
        process::exit(1);
    }

    fn other(&self) -> i32 {
        let mut cmd = Command::new(&self.args[0])
            .args(&self.args[1..])
            .spawn()
            .expect("Command not found--Did you mean something else?");
        cmd.wait().expect("Can not wait for child to complete.");
        1
    }
}

fn tokenize(input: String) -> Vec<String> {
    let args: Vec<_> = input.split_whitespace().map(|x| x.to_string()).collect();
    args
}

fn main() {
    loop {
        print!("minishell> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        // let mut funcs: Funcs;
        io::stdin().read_line(&mut input).unwrap();
        // println!("{:?}", input);
        let funcs = Funcs::new(tokenize(input));
        if input.contains("|") {
            funcs.piped();
        }
        match funcs.args[0].as_str() {
            "cd" => funcs.cd(),
            "help" => funcs.help(),
            "exit" => funcs.exit(),
            _ => funcs.other(),
        };
    }
}
