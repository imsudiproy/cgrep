use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern : String,
    path: std::path::PathBuf,
}

fn main() {
    //implements how to take command line arguments without using any library
    //like clap:
    //====================================================================
    //let pattern = std::env::args().nth(1).expect("No pattern provided");
    //let path = std::env::args().nth(2).expect("No Path provided");
    //====================================================================

    let args = Cli::parse();
    let  content = std::fs::read_to_string(&args.path).expect("Couldn't read the file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    //println!("Pattern: {:?}, Path: {:?}", args.pattern, args.path);
}
