pub enum AppError {
    MissingArguments { expected: usize, got: usize },
    UnknownAutomaton(String),
    InvalidNumber(String),
    ZeroSize(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_type = match self {
            AppError::MissingArguments { .. } => "MissingArguments",
            AppError::UnknownAutomaton(_) => "UnknownAutomaton",
            AppError::InvalidNumber(_) => "InvalidNumber",
            AppError::ZeroSize(_) => "ZeroSize",
        };

        let error_msg = match self {
            AppError::MissingArguments { expected, got } => {
                format!("Not enough arguments: excepted {expected}, got {got}.")
            }
            AppError::UnknownAutomaton(val) => {
                format!("Unknown automaton \"{val}\". Available: conway, langton.")
            }
            AppError::InvalidNumber(val) => format!("Failed to parse number from \"{val}\"."),
            AppError::ZeroSize(val) => format!("Value \"{val}\" must be greather than zero."),
        };

        write!(f, "({}) {}", error_type, error_msg)
    }
}
