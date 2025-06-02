use std::fs::File; 
use std::io::{self, BufWriter, BufRead, BufReader, Write};
use std::env;
use std::process;

// config struct to hold the configuration options
struct Config{
    show_line_numbers: bool,
    show_nonprinting: bool, 
    squeeze_blank: bool
}

// Implementation of the Config struct
impl Config{
    fn new() -> Self {

        Config {
            show_line_numbers: false, 
            show_nonprinting: false,
            squeeze_blank: false,

        }
    }
}

/// Concatenate files and print to stdout
/// If no files are provided, read from stdin and write to stdout
/// If the -n option is provided, show line numbers
/// If the -s option is provided, remove repeated blank lines from output
/// If the -v option is provided, show non-printing characters in the output
/// If the -h option is provided, show help message
/// If an invalid option is provided, print an error message and exit
/// If a file does not exist, print an error message and exit
fn concatenate_file(filename: &str, config: &Config) -> io::Result<()>{

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut stdout = BufWriter::new(io::stdout());

    let mut line_number = 1;
    let mut last_line_was_empty = false;

    for line in reader.lines(){
        let mut line = line?;

        // If the -s option is provided, remove repeated blank lines
        // If the line is empty and the last line was also empty, skip this line
        if config.squeeze_blank && line.trim().is_empty(){
            if last_line_was_empty {
                continue;
            }
            last_line_was_empty = true;   
        } else {
            last_line_was_empty = false;
        }

        // If the -v option is provided, escape non-printing characters
        if config.show_nonprinting {
            line = escape_nonprinting(&line);
        }

        // If the -n option is provided, show line numbers
        if config.show_line_numbers{
            writeln!(&mut stdout, "{:6}  {}", line_number, line)?;
            line_number += 1;
        } else {
            writeln!(&mut stdout, "{}", line)?;
        }
    }

    stdout.flush()?;
    Ok(())
}

fn escape_nonprinting(s: &str) -> String {
    let mut result = String::new();
    // Iterate over each character in the string
    // and convert control characters to their ascii representation
    for c in s.chars(){
        if c.is_control(){
            match c {
                // convert tab to ^I representation of tab character
                '\t' => result.push_str("^I"),
                '\n' => {}, // ignore newline
                // convert to ascii characters 
                _ => result.push_str(&format!("^{}", (c as u8 + 64) as char)),

            }
        } else {
            // if the character is printable, just add it to the result`
            // otherwise, it will be converted to its ascii representation
            // in the match statement above
            // printable characters are those that are not control characters
            // and are not whitespace characters`
            result.push(c);
        }
    }
    result
}


/// Print usage information
/// This function is called when the user requests help or provides an invalid option
/// or when no files are provided
fn print_usage(program: &str){
    // Print usage information
    // This function is called when the user requests help or provides an invalid option
    // or when no files are provided
    eprintln!("HowTo: {} [options] [file]...", program);
    eprintln!("Options:");
    eprintln!(" -h      Show this help message");
    eprintln!(" -n      Show line numbers");
    eprintln!(" -s      Remove repeated blank lines from output");
    eprintln!(" -v      Show non-printing characters in the output");
}


/// Main function
/// Parses command line arguments, sets configuration options,
/// and calls the `concatenate_file` function for each file provided
/// If an invalid option is provided, prints an error message and exits

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut config = Config::new();
    let mut filenames = Vec::new();

    let mut i = 1;

    while i < args.len() {
        // Check if the argument is an option
        // If it is, set the corresponding config option
        match args[i].as_str() {
            // If the argument is -n, set show_line_numbers to true
            // If the argument is -s, set squeeze_blank to true
            // If the argument is -v, set show_nonprinting to true
            // If the argument is -h, print usage and exit
            "-n" => config.show_line_numbers = true,
            "-s" => config.squeeze_blank = true,
            "-v" => config.show_nonprinting = true,
            "-h" => {
                print_usage(&program);
                process::exit(0);
            },
            // If the argument starts with a dash, but is not a valid option
            // print an error message and exit
            arg if arg.starts_with("-") => {
                eprintln!("{}: invalid option -- '{}' ", program, arg);
                print_usage(&program);
                process::exit(1)
            },
            // Otherwise, treat it as a filename
            // and add it to the list of filenames
            _ => filenames.push(args[i].clone()),
        }
        i += 1;
    }

    // check filename(s) is provided
    if filenames.is_empty(){
        // If no files are provided, read from stdin
        // and write to stdout
        if let Err(e) = io::copy(&mut io::stdin(), &mut io::stdout()) {
            // If an error occurs while reading from stdin, print the error and exit
            eprintln!("{}: stdin: {}", program, e);
            process::exit(1);
        }
        return Ok(());
    }

    let mut exit_code = 0;
    for filename in filenames {
        if concatenate_file(&filename, &config).is_err() {
            eprintln!("{}: {}: No such file or directory", program, filename);
            exit_code = 1;
        }
    }

    if exit_code != 0{
        process::exit(exit_code);
    }

    Ok(())
}