use std::io::{self, Write};

fn READ(input: String) -> String {
    input
}

fn EVAL(input: String) -> String {
    input
}

fn PRINT(input: String) -> String {
    input
}

fn rep(input: String) -> String {
    let read_ret = READ(input);
    let eval_ret = EVAL(read_ret);
    let print_ret = PRINT(eval_ret);
    print_ret
}

fn main() -> io::Result<()> {
    loop {
        print!("user> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let rep_ret = rep(input.trim().to_string());
        println!("{rep_ret}");
    }
}
