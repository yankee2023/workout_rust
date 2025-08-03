use clap::{arg, App};

fn main() {
    let matches = App::new("Sample CLI")
        .version("1.0.0")
        .author("Your Name")
        .about("Super awesome sample RPN calculator")
        .arg(arg!([FILE] "The input file to use").required(false))
        .arg(arg!(-v --verbose ... "Sets the level of verbosity").required(false))
        .get_matches();

    match matches.value_of("FILE") {
        Some(file) => println!("Using file: {}", file),
        None => println!("No file provided"),
    }

    let verbose = matches.is_present("verbose");
    println!("Verbose mode is {}", if verbose { "on" } else { "off" });
}
