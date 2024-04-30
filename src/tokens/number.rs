use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
/// A JS number literal. There are 4 kinds of number
/// literals allowed in JS.
///
/// - Decimal Literals - This includes integers and decimals with
///     optional exponent notation
/// - Hexadecimal Literals - These begin with 0x and consist of numbers
///     0-9 and letters A-F (case insensitive)
/// - Octal Literals - These being with 0o and consist of numbers
///     0-7
/// - Binary Literals - These begin with 0b and consist of numbers 0 and 1
pub struct Number<T>(T);

/// Extension methods for allowing Number
/// to work with both &str and String
pub trait NumberExt {
    fn kind(&self) -> NumberKind;
    fn is_hex(&self) -> bool;
    fn is_bin(&self) -> bool;
    fn is_oct(&self) -> bool;
    fn is_dec(&self) -> bool;
    fn has_exponent(&self) -> bool;
    fn is_big_int(&self) -> bool;
}

impl<T> Number<T>
where
    T: AsRef<str>,
{
    pub fn kind(&self) -> NumberKind {
        let s = self.0.as_ref();
        match self.0.as_ref().get(0..2) {
            Some("0x") | Some("0X") => NumberKind::Hex,
            Some("0b") | Some("0B") => NumberKind::Bin,
            Some("0o") | Some("0O") => NumberKind::Oct,
            _ => {
                if s.ends_with('n') {
                    NumberKind::BigInt
                } else {
                    NumberKind::Dec
                }
            }
        }
    }

    pub fn is_hex(&self) -> bool {
        self.kind() == NumberKind::Hex
    }

    pub fn is_bin(&self) -> bool {
        self.kind() == NumberKind::Bin
    }

    pub fn is_oct(&self) -> bool {
        self.kind() == NumberKind::Oct
    }

    pub fn is_dec(&self) -> bool {
        self.kind() == NumberKind::Dec
    }

    pub fn has_exponent(&self) -> bool {
        match self.kind() {
            NumberKind::Dec => self.0.as_ref().contains(|c| c == 'e' || c == 'E'),
            _ => false,
        }
    }

    pub fn is_big_int(&self) -> bool {
        self.kind() == NumberKind::BigInt
    }
}

impl<'a> From<&'a str> for Number<&'a str> {
    fn from(s: &'a str) -> Self {
        Number(s)
    }
}

impl<T> Display for Number<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> PartialEq<str> for &Number<T>
where
    T: AsRef<str>,
{
    fn eq(&self, other: &str) -> bool {
        self.0.as_ref().eq(other)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// The 5 kinds of numbers
pub enum NumberKind {
    Dec,
    Hex,
    Bin,
    Oct,
    BigInt,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind() {
        assert_eq!(Number::from("0x0").kind(), NumberKind::Hex);
        assert_eq!(Number::from("0o0").kind(), NumberKind::Oct);
        assert_eq!(Number::from("0b0").kind(), NumberKind::Bin);
        assert_eq!(Number::from("0.0").kind(), NumberKind::Dec);
        assert_eq!(Number::from("0.0n").kind(), NumberKind::BigInt);
    }

    #[test]
    fn helpers() {
        assert_helpers(
            "hex",
            &Number::from("0x0"),
            &Number::is_hex,
            &[
                &Number::is_big_int,
                &Number::is_oct,
                &Number::is_dec,
                &Number::is_bin,
            ],
        );
        assert_helpers(
            "oct",
            &Number::from("0o0"),
            &Number::is_oct,
            &[
                &Number::is_big_int,
                &Number::is_hex,
                &Number::is_dec,
                &Number::is_bin,
            ],
        );
        assert_helpers(
            "bin",
            &Number::from("0b0"),
            &Number::is_bin,
            &[
                &Number::is_big_int,
                &Number::is_hex,
                &Number::is_dec,
                &Number::is_oct,
            ],
        );
        assert_helpers(
            "dec",
            &Number::from("0.0"),
            &Number::is_dec,
            &[
                &Number::is_big_int,
                &Number::is_hex,
                &Number::is_bin,
                &Number::is_oct,
            ],
        );
        assert_helpers(
            "big",
            &Number::from("0.0n"),
            &Number::is_big_int,
            &[
                &Number::is_dec,
                &Number::is_hex,
                &Number::is_bin,
                &Number::is_oct,
            ],
        );
        assert!(Number::from("0.0e0").has_exponent());
        assert!(!Number::from("0.0").has_exponent());
        assert!(!Number::from("0.0n").has_exponent());
        assert!(!Number::from("0x0").has_exponent());
        assert!(!Number::from("0o0").has_exponent());
        assert!(!Number::from("0b0").has_exponent());
    }

    fn assert_helpers<'a>(
        name: &'static str,
        num: &'a Number<&'a str>,
        yes: impl Fn(&'a Number<&'a str>) -> bool,
        nos: &'a [&dyn Fn(&Number<&'a str>) -> bool],
    ) {
        assert!(yes(num), "`{}` ({}) failed for yes", num, name);
        for (i, f) in nos.into_iter().enumerate() {
            assert!(!f(num), "`{}` ({}) failed for no {}", num, name, i)
        }
    }
}
