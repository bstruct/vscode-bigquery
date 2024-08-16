use super::BqsqlDocumentItemType;

#[derive(Debug, Clone, Copy)]
pub(crate) enum BqsqlDelimiter {
    ParenthesesOpen,
    ParenthesesClose,
    SquareBracketsOpen,
    SquareBracketsClose,
    Dot,
    Comma,
    Semicolon,
}
impl BqsqlDelimiter {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            BqsqlDelimiter::ParenthesesOpen => "(",
            BqsqlDelimiter::ParenthesesClose => ")",
            BqsqlDelimiter::SquareBracketsOpen => "[",
            BqsqlDelimiter::SquareBracketsClose => "]",
            BqsqlDelimiter::Dot => ".",
            BqsqlDelimiter::Comma => ",",
            BqsqlDelimiter::Semicolon => ";",
        }
    }

    pub(crate) fn get_item_type(&self) -> BqsqlDocumentItemType {
        match self {
            BqsqlDelimiter::ParenthesesOpen => BqsqlDocumentItemType::ParenthesesOpen,
            BqsqlDelimiter::ParenthesesClose => BqsqlDocumentItemType::ParenthesesClose,
            BqsqlDelimiter::SquareBracketsOpen => BqsqlDocumentItemType::SquareBracketsOpen,
            BqsqlDelimiter::SquareBracketsClose => BqsqlDocumentItemType::SquareBracketsClose,
            BqsqlDelimiter::Dot => BqsqlDocumentItemType::Dot,
            BqsqlDelimiter::Comma => BqsqlDocumentItemType::Comma,
            BqsqlDelimiter::Semicolon => BqsqlDocumentItemType::Semicolon,
        }
    }
}
impl PartialEq<&str> for BqsqlDelimiter {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == other.to_string()
    }
}

impl PartialEq<BqsqlDelimiter> for &str {
    fn eq(&self, other: &BqsqlDelimiter) -> bool {
        self.to_string() == other.as_str()
    }
}

#[test]
fn compare_all() {
    assert_eq!(BqsqlDelimiter::ParenthesesOpen, "(");
    assert_eq!(BqsqlDelimiter::ParenthesesClose, ")");
    assert_eq!(BqsqlDelimiter::SquareBracketsOpen, "[");
    assert_eq!(BqsqlDelimiter::SquareBracketsClose, "]");
    assert_eq!(BqsqlDelimiter::Dot, ".");
    assert_eq!(BqsqlDelimiter::Comma, ",");
    assert_eq!(BqsqlDelimiter::Semicolon, ";");
    assert_eq!("(", BqsqlDelimiter::ParenthesesOpen);
    assert_eq!(")", BqsqlDelimiter::ParenthesesClose);
    assert_eq!("[", BqsqlDelimiter::SquareBracketsOpen);
    assert_eq!("]", BqsqlDelimiter::SquareBracketsClose);
    assert_eq!(".", BqsqlDelimiter::Dot);
    assert_eq!(",", BqsqlDelimiter::Comma);
    assert_eq!(";", BqsqlDelimiter::Semicolon);
}
