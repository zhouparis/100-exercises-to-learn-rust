// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

use std::convert::{TryFrom,TryInto};
use std::fmt::Error;

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl From<String> for Status {
    fn from(value: String) -> Self {
        Status::try_from(&value).unwrap()
    }
}

impl TryFrom<&String> for Status {
    type Error = Error;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case("todo") {return Ok(Status::ToDo)
        } else if value.eq_ignore_ascii_case("inprogress") { return Ok(Status::InProgress)
        } else if value.eq_ignore_ascii_case("done") { return Ok(Status::Done) 
        } else {
            return Err(Error)
        } 
    }
}

impl TryFrom<&str> for Status {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case("todo") {return Ok(Status::ToDo)
        } else if value.eq_ignore_ascii_case("inprogress") { return Ok(Status::InProgress)
        } else if value.eq_ignore_ascii_case("done"){ return Ok(Status::Done) 
        } else {
            return Err(Error)
        } 
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
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
