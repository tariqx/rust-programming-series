use std::env; 


// struct EchoOptions holds the options for the echo command
// It contains flags for -e (escaped characters)
// It is used to parse command line arguments and determine how to format the output
#[derive(Debug, Default)]
struct EchoOptions {
    escaped_args: bool,
}

// This function parses command line arguments to extract options for the echo command
// It processes flags like -e, and returns an EchoOptions struct
// It modifies the args vector in place, removing the flags as they are processed
// It stops processing flags when it encounters a non-flag argument or an unrecognized flag
// Example usage:
// echocli.exe -e "Hello, World!\nThis is a test."

fn parse_options(args: &mut Vec<String>) -> EchoOptions {
    // Initialize default options 
    let mut options = EchoOptions::default();
    
    while !args.is_empty() && args[0].starts_with('-') && args[0] != "-" {
        match args[0].as_str() {

            // more flags can be added here
            "-e" => {
                options.escaped_args = true;
                args.remove(0); // Remove the -e argument
            },
            _ => break, // If it's not a recognized flag, stop processing flags
        }
    }

    options
}

// This function handles escape sequences in a string
// It processes common escape sequences like \n, \t, \r, and 
// also handles quotes and slashes.
// It returns a new string with the escape sequences replaced 
// by their actual characters.
// It does not handle unicode or other complex escape sequences
// Example usage: echo "Hello, World!\nThis is a test."
// Note: This function is not a complete implementation of the "echo"
// command, but rather a simplified version.
fn handle_escapes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next_char) = chars.next(){
                match next_char {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    // using ASCII equivalent \x0B for vertical tab \v
                    // This is not a common escape sequence, but included for completeness
                    // Vertical tab is less commonly used
                    // It may not be supported in all terminals, but included for completeness 
                    'v' => result.push('\x0B'), // vertical tab
                    // slashes and quotes
                    '\\' => result.push('\\'),

                    '\'' => result.push('\''),
                    '"' => result.push('"'),
                    // default case for unrecognized escape sequences
                    // If it's not a recognized escape, just add it
                    _ => result.push(next_char), 
                }
            } else {
                // If the backslash is the last character, just add it
                result.push('\\');
            }

        } else {
            result.push(c);
        }
        
    }

    result
}

fn main() {

    // Collect command line arguments, skipping the first one (the program name)
    // Parse options from the command line arguments
    // The options are stored in an EchoOptions struct
    // The args vector is modified in place to remove the flags
    // The output is generated based on the remaining arguments
    // If the -e flag is present, escape sequences will be processed
    // If no arguments are provided, an empty string is returned
    let mut args: Vec<String> = env::args().skip(1).collect();
    let options = parse_options(&mut args);

    let output = if !args.is_empty() {
        let joined = args.join(" ");
        if options.escaped_args {
            handle_escapes(&joined)
        } else {
            joined
        }
    } else {
        String::new() // If no arguments, return an empty string
    };

    println!("{}", output);
    
}