use std::{env, error::Error, fs::File, io::Read, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = Input::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(input) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(input.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("{}", contents);

    Ok(())
}

struct Input {
    filename: String,
}

impl Input {
    fn new(args: &[String]) -> Result<Input, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Input { filename })
    }
}
