#[derive(Default, Debug, Clone, PartialEq, Eq, strum::Display, strum::EnumString)]
pub enum Boolean {
    #[strum(default)]
    Unknown(String),
    #[strum(serialize = "-1")]
    True,
    #[default]
    #[strum(serialize = "0")]
    False,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("-1", Boolean::True)]
    #[case("0", Boolean::False)]
    #[case("1", Boolean::Unknown("1".to_string()))]
    fn test_boolean_from_str(#[case] got: &str, #[case] should: Boolean) {
        let result = Boolean::from_str(got).unwrap();
        assert_eq!(result, should);
    }

    #[rstest]
    #[case(Boolean::True, "-1")]
    #[case(Boolean::False, "0")]
    #[case(Boolean::Unknown("1".to_string()), "1")]
    fn test_boolean_to_string(#[case] got: Boolean, #[case] should: &str) {
        let result = got.to_string();
        assert_eq!(result, should);
    }
}
