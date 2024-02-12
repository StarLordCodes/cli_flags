/// A struct representing a command line flag.
#[derive(Debug, Clone)]
pub struct Flag {
    flag: Option<String>, // The name of the flag
    is_short: bool,       // Indicates if the flag is short (e.g., -f)
    is_long: bool,        // Indicates if the flag is long (e.g., --flag)
    arg: Option<String>,  // The argument associated with the flag, if any
}

/// Parses command line arguments into a vector of Flag structs.
pub fn parse_flags() -> Vec<Flag> {
    let mut output_flags = Vec::new();
    let std_input_args: Vec<String> = std::env::args().skip(1).collect();

    let mut index = 0;
    while index < std_input_args.len() {
        let input = &std_input_args[index];
        match check_flag(input) {
            "short" => {
                let (new_index, flag, arg) = parse_flag(&std_input_args, index);
                output_flags.push(Flag {
                    flag,
                    is_short: true,
                    is_long: false,
                    arg,
                });
                index = new_index;
            }
            "long" => {
                let (new_index, flag, arg) = parse_flag(&std_input_args, index);
                output_flags.push(Flag {
                    flag,
                    is_short: false,
                    is_long: true,
                    arg,
                });
                index = new_index;
            }
            _ => {
                // Handle standalone argument
                output_flags.push(Flag {
                    flag: None,
                    is_short: false,
                    is_long: false,
                    arg: Some(input.clone()),
                });
                index += 1; // Move to the next argument
            }
        }
    }
    output_flags
}

/// Parses a flag along with its associated argument, if any.
fn parse_flag(args: &[String], index: usize) -> (usize, Option<String>, Option<String>) {
    let flag = Some(args[index].to_string());
    let next_index = index + 1;
    if next_index < args.len() {
        let next_arg = &args[next_index];
        let argument = match check_flag(next_arg) {
            "argument" => Some(next_arg.to_string()),
            _ => None,
        };
        return (next_index + 1, flag, argument);
    } else {
        return (next_index, flag, None);
    }
}

/// Checks the type of a flag.
fn check_flag(word: &str) -> &str {
    if word.starts_with("--") {
        "long"
    } else if word.starts_with("-") {
        "short"
    } else {
        "argument"
    }
}