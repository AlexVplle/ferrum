use crate::{parse_optional_str_arg, parse_repeated_str_arg};

pub struct RunConfig {
    pub memory: Option<String>,
    pub window: bool,
    pub gdb: bool,
    pub debug: bool,
    pub numa: Option<usize>,
    pub smp: Option<usize>,
    pub distances: Vec<String>,
}

impl RunConfig {
    pub fn from_args(args: &[String]) -> Self {
        Self {
            memory: parse_optional_str_arg(args, "--memory"),
            window: args.contains(&"--window".to_string()),
            gdb: args.contains(&"--gdb".to_string()),
            debug: args.contains(&"--debug".to_string()),
            numa: parse_optional_str_arg(args, "--numa")
                .map(|value: String| value.parse::<usize>().unwrap_or_else(|_| panic!("--numa requires a number"))),
            smp: parse_optional_str_arg(args, "--smp")
                .map(|value: String| value.parse::<usize>().unwrap_or_else(|_| panic!("--smp requires a number"))),
            distances: parse_repeated_str_arg(args, "--distance"),
        }
    }
}
