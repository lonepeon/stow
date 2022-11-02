#[derive(Debug, PartialEq, Eq)]
pub struct Path(String);

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Path {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !std::path::Path::new(s).exists() {
            return Err(Error(format!("path {} does not exist", s)));
        }

        Ok(Path(s.to_string()))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_path() {
        let err = "not-existing.file".parse::<Path>().unwrap_err();
        assert_eq!(
            Error("path not-existing.file does not exist".to_string()),
            err
        );
    }

    #[test]
    fn valid_path() {
        let err = "src/path.rs".parse::<Path>().unwrap();
        assert_eq!(Path("src/path.rs".to_string()), err);
    }
}
