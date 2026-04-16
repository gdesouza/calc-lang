mod parser;

fn main() {
    let mut args = std::env::args();
    let current_program_path = args.next().unwrap();
    match args.next() {
        Some(path) if path == "-" => process_stdin(),
        Some(path) => process_file(&path),
        None => {
            if atty::is(atty::Stream::Stdin) {
                eprintln!("Usage: {} <source_file>", current_program_path);
                eprintln!("       ... | {} -", current_program_path);
            } else {
                process_stdin();
            }
        }
    }
}

fn process_stdin() {
    let source_code = std::io::read_to_string(std::io::stdin());
    match source_code {
        Ok(code) => process_source("<stdin>", &code),
        Err(e) => eprintln!("Error reading from stdin: {}", e),
    }
} 

fn process_file(source_path: &str) {
    const CALC_SUFFIX: &str = ".calc";
    if !source_path.ends_with(CALC_SUFFIX) {
        eprintln!("Error: Source file must have a '{}' extension.", CALC_SUFFIX);
        return;
    }
    match std::fs::read_to_string(source_path) {
        Ok(code) => process_source(source_path, &code),
        Err(e) => eprintln!("Error: Could not read source file {}: ({})", source_path, e),
    }
}

fn process_source(source_name: &str, source_code: &str) {
    let parsed_program;
    match parser::parse_program(source_code) {
        Ok((rest, syntax_tree)) => {
            let trimmed_rest = rest.trim();
            if trimmed_rest.len() > 0 {
                eprintln!(
                    "Invalid remaining code in '{}': {}",
                    source_name, 
                    trimmed_rest
                );
                return;
            }
            parsed_program = syntax_tree;
        }
        Err(error) => {
            eprintln!(
                "Error parsing source file {}: ({})", 
                source_name,
                error
            );
            return;
        }
    }

    println!("Parsed program from '{}':\n{:#?}", source_name, parsed_program);
}
