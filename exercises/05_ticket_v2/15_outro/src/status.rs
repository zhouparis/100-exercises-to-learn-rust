// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, thiserror::Error)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl TryFrom<String> for Status {
    type Error = NewStatusError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case("todo") { return Ok(Status::ToDo)
        } else if value.eq_ignore_ascii_case("inprogress") { return Ok(Status::InProgress)
        } else if value.eq_ignore_ascii_case("done") { return Ok(Status::Done)
        } else { return Err(NewStatusError {invalid_status: value})}
    }
}

impl TryFrom<&str> for Status {
    type Error = NewStatusError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case("todo") { return Ok(Status::ToDo)
        } else if value.eq_ignore_ascii_case("inprogress") { return Ok(Status::InProgress)
        } else if value.eq_ignore_ascii_case("done") { return Ok(Status::Done)
        } else { return Err(NewStatusError {invalid_status: value.to_string()})}
    }
}


#[derive(Debug, thiserror::Error)]
pub struct NewStatusError {
    invalid_status: String,
}

impl Display for NewStatusError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "something went wrong with status")
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
