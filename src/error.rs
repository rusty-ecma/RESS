#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub line: usize,
    pub column: usize,
    pub msg: String,
    pub idx: usize,
}

impl ::std::error::Error for Error {}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{} at {}:{}", self.msg, self.line, self.column)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawError {
    pub idx: usize,
    pub msg: String,
}

impl ::std::error::Error for RawError {}

impl ::std::fmt::Display for RawError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{} at {}", self.msg, self.idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            Error {
                line: 1,
                column: 1,
                msg: "err".to_string(),
                idx: 0,
            }
            .to_string(),
            "err at 1:1"
        );
        assert_eq!(
            RawError {
                msg: "err".to_string(),
                idx: 0,
            }
            .to_string(),
            "err at 0"
        );
    }
}
