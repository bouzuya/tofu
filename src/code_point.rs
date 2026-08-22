#[derive(Debug, ::thiserror::Error)]
pub enum CodePointError {
    #[error("invalid code point: {0}")]
    InvalidCodePoint(String),
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CodePoint(u32);

impl CodePoint {
    pub fn from_char(c: char) -> Self {
        Self::from_u32(u32::from(c)).expect("char is always in the range 0x0..=0x10FFFF")
    }

    pub fn from_str_with_u_plus(s: &str) -> Option<Self> {
        s.strip_prefix("U+").and_then(Self::from_str_without_u_plus)
    }

    pub fn from_str_without_u_plus(s: &str) -> Option<Self> {
        u32::from_str_radix(s, 16)
            .ok()
            .and_then(|it| matches!(it, 0x0..=0x10FFFF).then(|| Self(it)))
    }

    pub fn from_u32(code_point: u32) -> Option<Self> {
        matches!(code_point, 0x0..=0x10FFFF).then(|| Self(code_point))
    }

    pub fn to_char(&self) -> Option<char> {
        std::char::from_u32(self.0)
    }

    pub fn to_string_with_u_plus(&self) -> String {
        format!("U+{}", self.to_string_without_u_plus())
    }

    pub fn to_string_without_u_plus(&self) -> String {
        format!("{:04X}", self.0)
    }

    pub fn to_u32(&self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for CodePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_without_u_plus())
    }
}

impl std::str::FromStr for CodePoint {
    type Err = CodePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_without_u_plus(s)
            .ok_or_else(|| CodePointError::InvalidCodePoint(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Context;

    use super::*;

    #[test]
    fn test_from_char() {
        assert_eq!(CodePoint::from_char('A'), CodePoint(0x0041));
        assert_eq!(CodePoint::from_char('あ'), CodePoint(0x3042));
    }

    #[test]
    fn test_from_str_with_u_plus() {
        assert_eq!(
            CodePoint::from_str_with_u_plus("U+0041"),
            CodePoint::from_u32(0x0041)
        );
        assert_eq!(
            CodePoint::from_str_with_u_plus("U+10FFFF"),
            CodePoint::from_u32(0x10FFFF)
        );
        assert_eq!(CodePoint::from_str_with_u_plus("U+110000"), None);
        assert_eq!(CodePoint::from_str_with_u_plus("U+GARBAGE"), None);
    }

    #[test]
    fn test_from_str_without_u_plus() {
        assert_eq!(
            CodePoint::from_str_without_u_plus("0041"),
            CodePoint::from_u32(0x0041)
        );
        assert_eq!(
            CodePoint::from_str_without_u_plus("10FFFF"),
            CodePoint::from_u32(0x10FFFF)
        );
        assert_eq!(CodePoint::from_str_without_u_plus("110000"), None);
        assert_eq!(CodePoint::from_str_without_u_plus("GARBAGE"), None);
    }

    #[test]
    fn test_from_u32() {
        assert_eq!(CodePoint::from_u32(0x0041), Some(CodePoint(0x0041)));
        assert_eq!(CodePoint::from_u32(0x10FFFF), Some(CodePoint(0x10FFFF)));
        assert_eq!(CodePoint::from_u32(0x110000), None);
    }

    #[test]
    fn test_impl_clone_and_copy_trait() -> anyhow::Result<()> {
        fn assert_impls<T: Clone + Copy>() {}
        assert_impls::<CodePoint>();
        Ok(())
    }

    #[test]
    fn test_impl_display_trait() -> anyhow::Result<()> {
        assert_eq!(
            CodePoint::from_str_without_u_plus("0041")
                .context("from_str_without_u_plus(\"0041\")")?
                .to_string(),
            "0041"
        );
        assert_eq!(
            CodePoint::from_str_without_u_plus("10FFFF")
                .context("from_str_without_u_plus(\"10FFFF\")")?
                .to_string(),
            "10FFFF"
        );
        Ok(())
    }

    #[test]
    fn test_impl_partial_ord_trait() -> anyhow::Result<()> {
        fn assert_impls<T: PartialOrd>() {}
        assert_impls::<CodePoint>();
        Ok(())
    }

    #[test]
    fn test_impl_from_str_trait() -> anyhow::Result<()> {
        assert_eq!(
            <CodePoint as std::str::FromStr>::from_str("0041")?.to_string(),
            "0041"
        );
        assert_eq!(
            <CodePoint as std::str::FromStr>::from_str("10FFFF")?.to_string(),
            "10FFFF"
        );
        Ok(())
    }

    #[test]
    fn test_to_string_with_u_plus() -> anyhow::Result<()> {
        assert_eq!(
            CodePoint::from_str_without_u_plus("0041")
                .context("from_str_without_u_plus(\"0041\")")?
                .to_string_with_u_plus(),
            "U+0041"
        );
        assert_eq!(
            CodePoint::from_str_without_u_plus("10FFFF")
                .context("from_str_without_u_plus(\"10FFFF\")")?
                .to_string_with_u_plus(),
            "U+10FFFF"
        );
        Ok(())
    }

    #[test]
    fn test_to_string_without_u_plus() -> anyhow::Result<()> {
        assert_eq!(
            CodePoint::from_str_without_u_plus("0041")
                .context("from_str_without_u_plus(\"0041\")")?
                .to_string_without_u_plus(),
            "0041"
        );
        assert_eq!(
            CodePoint::from_str_without_u_plus("10FFFF")
                .context("from_str_without_u_plus(\"10FFFF\")")?
                .to_string_without_u_plus(),
            "10FFFF"
        );
        Ok(())
    }

    #[test]
    fn test_to_u32() -> anyhow::Result<()> {
        assert_eq!(
            CodePoint::from_u32(0x0041)
                .context("from_u32(0x0041)")?
                .to_u32(),
            0x0041
        );
        assert_eq!(
            CodePoint::from_u32(0x10FFFF)
                .context("from_u32(0x10FFFF)")?
                .to_u32(),
            0x10FFFF
        );
        Ok(())
    }
}
