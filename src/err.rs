use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("A Serenity API error has occurred: {0}")]
    APIError(#[from] serenity::Error),
    #[error("A GPT error has occurred: {0}")]
    GPTError(#[from] chatgpt::err::Error),
    #[error("An unknown error has occurred!")]
    Unknown,
}

pub type Res<T> = Result<T, CommandError>;

#[macro_export]
macro_rules! check {
    ($opt:expr) => {{
        let v = $opt;
        match v {
            Some(v) => Ok(v),
            None => Err($crate::err::CommandError::Unknown),
        }
    }?};
    ($opt:expr, $against:path) => {{
        let v = $opt;
        match v {
            $against(v) => Ok(v),
            _ => Err($crate::err::CommandError::Unknown),
        }
    }?};
}

#[macro_export]
macro_rules! embed {
    () => {{}};
}
