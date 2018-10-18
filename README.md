# Building a Mini Shell in Rust

> Disclaimer: I'm sure there is a much more elegant way to do this.

There are 4 parts to a shell:

- Read
- Evaluate
- Print
- Loop

(REPL) for short. A recent assignment for one of my classes was to build a shell in C. So, I figured it would be a good learning experience to do the same in Rust.

_This article will assume you have a basic knowledge of Rust._

## Getting Started.

Move to a location where you'd like to keep this little project in a Terminal.

```
cargo new minishell
cd minishell/src
```

Now open `main.rs` in your preferred IDE.

It should look something like this.

```rust
fn main() {
    println!("Hello, world!");
}
```

## Reading In

Above `fn main()` add `use std::io::{self, Write};` and replace `println!("Hello, world!");` with

```rust
print!("minishell>");
io::stdout().flush().unwrap();
```

If you run `cargo run`, you should see `minishell>` appear.

To read in input, we'll add

```rust
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
```

So far, `main.rs` should look like this

```rust
use std::io::{self, Write};

fn main() {
    print!("minishell>");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
```

and calling `cargo run` should take in input. We can check by adding `println!("{}", input)` to the end of `main`.

### Try it

```
minishell>cd ../minishell
cd ../minishell
```

## Parsing Input

We need to be able to access each part of the input as separate elements. To do this, we'll split the string into an `Vec` of Strings.

Below, `main`, add the following function

```rust
fn tokenize(input: String) -> Vec<String> {
    let args: Vec<_> = input.split_whitespace().map(|x| x.to_string()).collect();
    args
}
```

This function takes in a String `input` and calls the _awesome_ `split_whitespace()` method which returns an `Iterator` of `SplitWhiteSpace` structs. This means we can convert it into Strings using `map` and then `collect` all of the outputs into a `Vec<String>`.

### Try It

Add the following to `main()` and run `cargo run`

```rust
let tokens = tokenize(input);
println!("{:?}", input);
```

which will give the output

```
minishell>cd ../minishell
cd ../minishell
["cd", "../minishell"]
```

### Evaluating

In this minishell, we'll be implementing `cd`, `help`, `exit` and any other command passed will be send to `bin` commands (i.e. `ls`, `grep`, etc.).

To do this, we'll use a struct `Func`

```rust
struct Func {
    args: Vec<String>
}
```

This just means our struct will have a property args which is a `Vec` of Strings. The juicy comes next

```rust
impl Func {
    fn new(args: Vec<String>) -> Self {
        Func { args: args }
    }

    fn cd(&self) -> i32 {}

    fn help(&self) -> i32 {}

    fn exit(&self) -> i32 {}

    fn other(&self) -> i32 {}
}
```

Above contains the boilerplate for our future functions and the constructor for `Func`s.

So, we should have in `main.rs`

```rust
use std::io::{self, Write};

struct Func {
    args: Vec<String>
}

impl Func {
    fn new(args: Vec<String>) -> Self {
        Func { args: args }
    }

    fn cd(&self) -> i32 {}

    fn help(&self) -> i32 {}

    fn exit(&self) -> i32 {}

    fn other(&self) -> i32 {}
}

fn main() {
    print!("minishell>");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn tokenize(input: String) -> Vec<String> {
    let args: Vec<_> = input.split_whitespace().map(|x| x.to_string()).collect();
    args
}
```

_I've not included any of the **Try It** section code._

### Help

The help function is the most simple, we'll just add a `println!` with out information.

```rust
    fn help(&self) -> i32 {
        println!("Functions:");
        println!("\tcd: changes the current directory");
        println!("\help: prints built in commands descriptions");
        println!("\texit: closes the minishell and all of it's processes.");
        1
    }
```

You're probably wondering why there is a `1` hanging out all alone at the end of the function. This will come up later.

### Exit

Adding `use std::process;` at the top of `main.rs` means we can simply write `exit()` as

```rust
fn exit(&self) -> i32 {
    process::exit(1);
}
```

This tells Rust to completely close the program.

### cd

`cd` is slightly more complicated. Add `use std::path::Path;` and `use std::env::set_current_dir;` to the top, and add to `cd()`

```rust
fn cd(&self) -> i32 {
    if self.args.len() == 0 {
        panic!("No Arguments!")
    }
    let p = Path::new(&self.args[1]);
    set_current_dir(&p).unwrap();
    1
}
```

Our if statement will check to make sure our input has some path to change to. The following lines creates a new `Path` with the second element in our tokenized input as it's intended location. Then, the `set_current_dir()` function takes this in and makes it happen.

### Everything else.

Now, we can take care of any commands found in `/bin/`. We'll use `Command` for this. It will receive a reference to `self.args[0]` as the command and it's arguments will be the rest of `self.args`. Add `use std::process::{Command, Stdio};` to `main.rs` and

```rust
fn other(&self) -> i32 {
    let mut cmd = Command::new(&self.args[0])
        .args(&self.args[1..])
        .spawn()
        .expect("Command not found--Did you mean something else?");
    cmd.wait().expect("Can not wait for child to complete.");
    1
}
```

The third line `.args(&self.args[1..])` uses a slice. This means get from `self.args` every element from index `1` to the end of the `Vec`. This way we won't be giving the command again and don't have to worry about mutating the `Vec`.

`spawn()` involves a process called forking. For our purposes, it means that a new exact copy of our program is made where the command we've made actually runs. This is called a child. It will run, finish and exit, passing back it's output to our original program.

The `expect()` allows us to inform the user when they've input an invalid argument or command.

Calling `wait()` on our command means we pause the program until the command has completed (and/or the spawned process has been killed).

`main.rs` should now look like

```rust
use std::env::set_current_dir;
use std::io::{self, Write};
use std::path::Path;
use std::process;
use std::process::{Command, Stdio};

struct Func {
    args: Vec<String>
}

impl Func {
    fn new(args: Vec<String>) -> Self {
        Func { args: args }
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
        println!("Functions:");
        println!("\tcd: changes the current directory");
        println!("\help: prints built in commands descriptions");
        println!("\texit: closes the minishell and all of it's processes.");
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

fn main() {
    print!("minishell>");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn tokenize(input: String) -> Vec<String> {
    let args: Vec<_> = input.split_whitespace().map(|x| x.to_string()).collect();
    args
}
```

## Almost There!

Now we need to actually have our commands run! We'll do this by matching the zeroeth argument to "cd", "help", "exit" or something else.

```rust
let funcs = Func::new(tokenize(input));
match funcs.args[0].as_str() {
    "cd" => funcs.cd(),
    "help" => funcs.help(),
    "exit" => funcs.exit(),
    _ => funcs.other(),
};
```

It is very important to call the `as_str()` method on `funcs.args[0]`. Otherwise, our branches will be recognized as `&str` and `funcs.args[0]` is of type `String`. This will cause a type mismatch and won't compile.

This is also where we see why we had all those weird `1`s. Essentially, every branch of a `match` must have the same return type. This was done through lots of trial and error and there is definitely a better way to acheive this.

### Try It

`cargo run`

```
minishell>ls
src target Cargo.lock Cargo.toml
```

## Nearly Done!

Our shell is done once one command is run! That's no good. If we wrap all of `main()` in a loop, it will continue until the user presses `Ctrl+C`.

```rust
fn main() {
    loop {
        print!("minishell> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let funcs = Func::new(tokenize(input));
        match funcs.args[0].as_str() {
            "cd" => funcs.cd(),
            "help" => funcs.help(),
            "exit" => funcs.exit(),
            _ => funcs.other(),
        };
    }
}
```

## Wrapping Up

Building a minishell in Rust was much simpler than I was expecting. Splitting by whitespace is a builtin function, processes are essentially taken care of and `match`s are incredibly easy to read and understand. I'd like to figure out how to add support for piping but overall, I would consider this a success!
