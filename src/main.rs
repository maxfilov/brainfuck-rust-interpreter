mod brainfuck;
mod io;

#[derive(Debug)]
struct AppError {
    s: String
}

impl std::fmt::Display for AppError {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.s)
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> AppError {
        AppError { s: format!("IO Error: {}", error) }
    }
}

impl From<brainfuck::InterpretationError> for AppError {
    fn from(error: brainfuck::InterpretationError) -> AppError {
        AppError { s: format!("Interpretation error: {}", error) }
    }
}

fn main() -> std::result::Result<(), AppError> {
    let source_code = io::get_source_code(&mut std::io::stdin())?;
    let mut interpreter = brainfuck::Interpreter::new();
    let result = interpreter.interpret(source_code)?;
    println!("{}", result);
    Ok(())
}
