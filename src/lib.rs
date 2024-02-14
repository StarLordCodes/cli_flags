use std::collections::HashMap;

/// A struct representing a command line flag.
#[derive(Debug, Clone)]
pub struct Flag {
    pub flag: Option<String>, // The name of the flag
    pub is_short: bool,       // Indicates if the flag is short (e.g., -f)
    pub is_long: bool,        // Indicates if the flag is long (e.g., --flag)
    pub arg: Option<String>,  // The argument associated with the flag, if any
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

/// Trait for extracting various types of flags and arguments from a vector of Flag structs.
pub trait ExtractFromVecFlags {
    /// Extracts arguments without associated flags.
    fn flagless_args(&self) -> Vec<String>;
    /// Extracts short boolean flags without arguments.
    fn short_bool_flags(&self) -> Vec<String>;
    /// Extracts long boolean flags without arguments.
    fn long_bool_flags(&self) -> Vec<String>;
    /// Extracts all boolean flags without arguments.
    fn all_bool_flags(&self) -> Vec<String>;
    /// Extracts short flags along with their associated arguments.
    fn short_flags_with_args(&self) -> HashMap<String, String>;
    /// Extracts long flags along with their associated arguments.
    fn long_flags_with_args(&self) -> HashMap<String, String>;
    /// Extracts all flags along with their associated arguments.
    fn all_flags_with_args(&self) -> HashMap<String, String>;
}

impl ExtractFromVecFlags for Vec<Flag> {
    fn flagless_args(&self) -> Vec<String> {
        self.iter()
            .filter_map(|flag_obj| {
                if !(flag_obj.is_short || flag_obj.is_long) {
                    flag_obj.arg.clone()
                } else {
                    None
                }
            })
            .collect()
    }
    fn short_bool_flags(&self) -> Vec<String> {
        self.iter()
            .filter_map(|flag_obj| {
                if flag_obj.is_short && flag_obj.arg.is_none() {
                    flag_obj.flag.clone()
                } else {
                    None
                }
            })
            .collect()
    }
    fn long_bool_flags(&self) -> Vec<String> {
        self.iter()
            .filter_map(|flag_obj| {
                if flag_obj.is_long && flag_obj.arg.is_none() {
                    flag_obj.flag.clone()
                } else {
                    None
                }
            })
            .collect()
    }

    fn all_bool_flags(&self) -> Vec<String> {
        self.iter()
            .filter_map(|flag_obj| {
                if (flag_obj.is_short || flag_obj.is_long) && flag_obj.arg.is_none() {
                    flag_obj.flag.clone()
                } else {
                    None
                }
            })
            .collect()
    }

    fn short_flags_with_args(&self) -> HashMap<String, String> {
        self.iter()
            .filter_map(|flag_obj| {
                if flag_obj.is_short && flag_obj.arg.is_some() {
                    flag_obj.flag.clone().zip(flag_obj.arg.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    fn long_flags_with_args(&self) -> HashMap<String, String> {
        self.iter()
            .filter_map(|flag_obj| {
                if flag_obj.is_long && flag_obj.arg.is_some() {
                    flag_obj.flag.clone().zip(flag_obj.arg.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    fn all_flags_with_args(&self) -> HashMap<String, String> {
        self.iter()
            .filter_map(|flag_obj| {
                if (flag_obj.is_short || flag_obj.is_long) && flag_obj.arg.is_some() {
                    flag_obj.flag.clone().zip(flag_obj.arg.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
