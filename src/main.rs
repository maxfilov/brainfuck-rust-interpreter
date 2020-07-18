mod brainfuck;
mod io;

#[derive(Debug)]
struct AppError {
    s: &'static str
}

impl std::fmt::Display for AppError {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.s)
    }
}

impl From<std::io::Error> for AppError {
    fn from(_error: std::io::Error) -> AppError {
        AppError { s: "IO Error" }
    }
}

impl From<brainfuck::InterpretationError> for AppError {
    fn from(error: brainfuck::InterpretationError) -> AppError {
        match error {
            brainfuck::InterpretationError::NonAsciiCode => AppError { s: "Non ascii character" },
            brainfuck::InterpretationError::UnmatchedLeftBracket => AppError { s: "Unmatched '['" },
            brainfuck::InterpretationError::UnmatchedRightBracket => AppError { s: "Unmatched ']'" },
        }
    }
}

fn main() -> std::result::Result<(), AppError> {
    let source_code = io::get_source_code(&mut std::io::stdin())?;
    let mut interpreter = brainfuck::Interpreter::new();
    let result = interpreter.interpret(source_code)?;
    println!("{}", result);
    Ok(())
}
