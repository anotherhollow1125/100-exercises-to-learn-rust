// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

use Status::*;

#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct InvalidStatusError(String);

fn str2status(value: &str) -> Result<Status, InvalidStatusError> {
    match value.to_lowercase().as_str() {
        "todo" => Ok(ToDo),
        "inprogress" => Ok(InProgress),
        "done" => Ok(Done),
        s => Err(InvalidStatusError(s.to_string())),
    }
}

impl TryFrom<String> for Status {
    type Error = InvalidStatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        str2status(&value)
    }
}

impl TryFrom<&str> for Status {
    type Error = InvalidStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        str2status(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
