mod parser;

fn main() {
    let mut args = std::env::args();
    let current_program_path = args.next().unwrap();
    let source_path = args.next();
    if source_path.is_none() {
        eprintln!("Usage: {} <source_file>", current_program_path);
    } else {
        process_file(&source_path.unwrap());
    }
} 

fn process_file(source_path: &str) {
    const CALC_SUFFIX: &str = ".calc";
    if !source_path.ends_with(CALC_SUFFIX) {
        eprintln!("Error: Source file must have a '{}' extension.", CALC_SUFFIX);
        return;
    }
    let source_code = std::fs::read_to_string(source_path);
    if source_code.is_err() {
        eprintln!(
            "Error: Could not read source file {}: ({})", 
            source_path,
            source_code.err().unwrap()        
        );
        return;
    }
    let source_code = source_code.unwrap();

    let parsed_program;
    match parser::parse_program(&source_code) {
        Ok((rest, syntax_tree)) => {
            let trimmed_rest = rest.trim();
            if trimmed_rest.len() > 0 {
                eprintln!(
                    "Invalid remaining code in '{}': {}",
                    source_path, 
                    trimmed_rest
                );
                return;
            }
            parsed_program = syntax_tree;
        }
        Err(error) => {
            eprintln!(
                "Error parsing source file {}: ({})", 
                source_path,
                error
            );
            return;
        }
    }

    println!("Parsed program from '{}':\n{:#?}", source_path, parsed_program);
}
