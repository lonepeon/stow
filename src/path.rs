#[derive(Debug, Eq, PartialEq)]
pub struct Source<'a>(&'a std::path::Path);

impl<'a> Source<'a> {
    pub fn new(path: &'a std::path::Path) -> Self {
        Self(path)
    }

    pub fn join(&self, folder: &str) -> std::path::PathBuf {
        self.0.join(folder)
    }
}

impl<'a> std::convert::AsRef<std::path::Path> for Source<'a> {
    fn as_ref(&self) -> &std::path::Path {
        self.0
    }
}

impl<'a> std::convert::From<&'a std::path::Path> for Source<'a> {
    fn from(path: &'a std::path::Path) -> Self {
        Source::new(path)
    }
}

impl<'a> std::convert::From<&'a str> for Source<'a> {
    fn from(path: &'a str) -> Self {
        Source::new(std::path::Path::new(path))
    }
}

impl<'a> std::fmt::Display for Source<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Destination<'a>(&'a std::path::Path);

impl<'a> std::convert::AsRef<std::path::Path> for Destination<'a> {
    fn as_ref(&self) -> &std::path::Path {
        self.0
    }
}

impl<'a> std::convert::From<&'a std::path::Path> for Destination<'a> {
    fn from(path: &'a std::path::Path) -> Self {
        Destination::new(path)
    }
}

impl<'a> std::convert::From<&'a str> for Destination<'a> {
    fn from(path: &'a str) -> Self {
        Destination::new(std::path::Path::new(path))
    }
}

impl<'a> std::fmt::Display for Destination<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl<'a> Destination<'a> {
    pub fn new(path: &'a std::path::Path) -> Self {
        Self(path)
    }

    pub fn join(&self, folder: &str) -> std::path::PathBuf {
        self.0.join(folder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_from_str() {
        let file: Source = "/some/path".into();
        assert_eq!(Source::new(std::path::Path::new("/some/path")), file)
    }

    #[test]
    fn source_from_path() {
        let file: Source = std::path::Path::new("/some/path").into();
        assert_eq!(Source::new(std::path::Path::new("/some/path")), file)
    }

    #[test]
    fn source_to_path() {
        let file: Source = "/some/path".into();
        assert_eq!(std::path::Path::new("/some/path"), file.as_ref())
    }

    #[test]
    fn join_on_source() {
        let file = Source::new(std::path::Path::new("/some/path")).join("to-file");
        assert_eq!(std::path::Path::new("/some/path/to-file"), file)
    }
    #[test]
    fn destination_from_str() {
        let file: Destination = "/some/path".into();
        assert_eq!(Destination::new(std::path::Path::new("/some/path")), file)
    }

    #[test]
    fn destination_to_path() {
        let file: Destination = "/some/path".into();
        assert_eq!(std::path::Path::new("/some/path"), file.as_ref())
    }

    #[test]
    fn destination_from_path() {
        let file: Destination = std::path::Path::new("/some/path").into();
        assert_eq!(Destination::new(std::path::Path::new("/some/path")), file)
    }

    #[test]
    fn join_on_destination() {
        let file = Destination::new(std::path::Path::new("/some/path")).join("to-file");
        assert_eq!(std::path::Path::new("/some/path/to-file"), file)
    }
}
