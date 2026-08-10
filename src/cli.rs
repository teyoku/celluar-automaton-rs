use crate::{
    config::{AppConfig, AutomatonKind},
    error::AppError,
};

pub fn parse_args(args: Vec<String>) -> Result<AppConfig, AppError> {
    if args.len() < 6 {
        return Err(AppError::MissingArguments {
            expected: 5,
            got: args.len() - 1,
        });
    }

    let automaton = parse_automaton(&args[1])?;
    let width = parse_positive_usize(&args[2], "width")?;
    let height = parse_positive_usize(&args[3], "height")?;
    let scale = parse_positive_usize(&args[4], "scale")?;
    let fps = parse_positive_usize(&args[5], "fps")?;

    Ok(AppConfig {
        automaton,
        width,
        height,
        scale,
        fps,
    })
}

fn parse_automaton(value: &str) -> Result<AutomatonKind, AppError> {
    match value {
        "conway" => Ok(AutomatonKind::Conway),
        "langton" => Ok(AutomatonKind::Langton),
        automaton => Err(AppError::UnknownAutomaton(automaton.to_string())),
    }
}

fn parse_positive_usize(value: &str, field_name: &str) -> Result<usize, AppError> {
    let number = value
        .parse::<usize>()
        .map_err(|_| AppError::InvalidNumber(value.to_string()))?;

    if number == 0 {
        Err(AppError::ZeroSize(field_name.to_string()))
    } else {
        Ok(number)
    }
}
