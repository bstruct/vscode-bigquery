//https://cloud.google.com/bigquery/docs/reference/standard-sql/operators

#[derive(Debug, Clone, Copy)]
pub(crate) enum BqsqlOperator {
    Addition,
    And,
    Between,
    BitwiseAnd,
    BitwiseLeftShift,
    BitwiseNot,
    BitwiseOr,
    BitwiseRightShift,
    BitwiseXor,
    Concatenation,
    Division,
    Equal,
    GreaterThan,
    GreaterThanOrEqualTo,
    In,
    IsFalse,
    IsNotFalse,
    IsNotNull,
    IsNotTrue,
    IsNull,
    IsTrue,
    LessThan,
    LessThanOrEqualTo,
    Like,
    Multiplication,
    Not,
    NotBetween,
    NotEqual1,
    NotEqual2,
    NotIn,
    NotLike,
    Or,
    Subtraction,
}

impl BqsqlOperator {
    pub(crate) fn to_vec(&self) -> Vec<&str> {
        match self {
            BqsqlOperator::Addition => Vec::from(vec!["+"]),
            BqsqlOperator::And => Vec::from(vec!["AND"]),
            BqsqlOperator::Between => Vec::from(vec!["BETWEEN"]),
            BqsqlOperator::BitwiseAnd => Vec::from(vec!["&"]),
            BqsqlOperator::BitwiseLeftShift => Vec::from(vec!["<", "<"]),
            BqsqlOperator::BitwiseNot => Vec::from(vec!["~"]),
            BqsqlOperator::BitwiseOr => Vec::from(vec!["|"]),
            BqsqlOperator::BitwiseRightShift => Vec::from(vec![">", ">"]),
            BqsqlOperator::BitwiseXor => Vec::from(vec!["^"]),
            BqsqlOperator::Concatenation => Vec::from(vec!["|", "|"]),
            BqsqlOperator::Division => Vec::from(vec!["/"]),
            BqsqlOperator::Equal => Vec::from(vec!["="]),
            BqsqlOperator::GreaterThan => Vec::from(vec![">"]),
            BqsqlOperator::GreaterThanOrEqualTo => Vec::from(vec![">", "="]),
            BqsqlOperator::In => Vec::from(vec!["IN"]),
            BqsqlOperator::IsFalse => Vec::from(vec!["IS", "FALSE"]),
            BqsqlOperator::IsNotFalse => Vec::from(vec!["IS", "NOT", "FALSE"]),
            BqsqlOperator::IsNotNull => Vec::from(vec!["IS", "NOT", "NULL"]),
            BqsqlOperator::IsNotTrue => Vec::from(vec!["IS", "NOT", "TRUE"]),
            BqsqlOperator::IsNull => Vec::from(vec!["IS", "NULL"]),
            BqsqlOperator::IsTrue => Vec::from(vec!["IS", "TRUE"]),
            BqsqlOperator::LessThan => Vec::from(vec!["<"]),
            BqsqlOperator::LessThanOrEqualTo => Vec::from(vec!["<", "="]),
            BqsqlOperator::Like => Vec::from(vec!["LIKE"]),
            BqsqlOperator::Multiplication => Vec::from(vec!["*"]),
            BqsqlOperator::Not => Vec::from(vec!["NOT"]),
            BqsqlOperator::NotBetween => Vec::from(vec!["NOT", "BETWEEN"]),
            BqsqlOperator::NotEqual1 => Vec::from(vec!["!", "="]),
            BqsqlOperator::NotEqual2 => Vec::from(vec!["<", ">"]),
            BqsqlOperator::NotIn => Vec::from(vec!["NOT", "IN"]),
            BqsqlOperator::NotLike => Vec::from(vec!["NOT", "LIKE"]),
            BqsqlOperator::Or => Vec::from(vec!["OR"]),
            BqsqlOperator::Subtraction => Vec::from(vec!["-"]),
        }
    }

    pub(crate) fn get_all() -> Vec<BqsqlOperator> {
        vec![
            BqsqlOperator::Addition,
            BqsqlOperator::And,
            BqsqlOperator::Between,
            BqsqlOperator::BitwiseAnd,
            BqsqlOperator::BitwiseLeftShift,
            BqsqlOperator::BitwiseNot,
            BqsqlOperator::BitwiseOr,
            BqsqlOperator::BitwiseRightShift,
            BqsqlOperator::BitwiseXor,
            BqsqlOperator::Concatenation,
            BqsqlOperator::Division,
            BqsqlOperator::Equal,
            BqsqlOperator::GreaterThan,
            BqsqlOperator::GreaterThanOrEqualTo,
            BqsqlOperator::In,
            BqsqlOperator::IsFalse,
            BqsqlOperator::IsNotFalse,
            BqsqlOperator::IsNotNull,
            BqsqlOperator::IsNotTrue,
            BqsqlOperator::IsNull,
            BqsqlOperator::IsTrue,
            BqsqlOperator::LessThan,
            BqsqlOperator::LessThanOrEqualTo,
            BqsqlOperator::Like,
            BqsqlOperator::Multiplication,
            BqsqlOperator::Not,
            BqsqlOperator::NotBetween,
            BqsqlOperator::NotEqual1,
            BqsqlOperator::NotEqual2,
            BqsqlOperator::NotIn,
            BqsqlOperator::NotLike,
            BqsqlOperator::Or,
            BqsqlOperator::Subtraction,
        ]
    }
}

impl PartialEq<Vec<&str>> for BqsqlOperator {
    fn eq(&self, other: &Vec<&str>) -> bool {
        let upper: Vec<String> = other.iter().map(|i| i.to_uppercase()).collect();

        self.to_vec().eq(&upper)
    }
}

impl PartialEq<BqsqlOperator> for Vec<&str> {
    fn eq(&self, other: &BqsqlOperator) -> bool {
        other.eq(self)
    }
}

#[test]
fn compare_all() {
    assert_eq!(BqsqlOperator::Addition, vec!["+"]);
    assert_eq!(BqsqlOperator::And, vec!["AnD"]);
    assert_eq!(BqsqlOperator::Between, vec!["Between"]);
    assert_eq!(BqsqlOperator::BitwiseAnd, vec!["&"]);
    assert_eq!(BqsqlOperator::BitwiseLeftShift, vec!["<", "<"]);
    assert_eq!(BqsqlOperator::BitwiseNot, vec!["~"]);
    assert_eq!(BqsqlOperator::BitwiseOr, vec!["|"]);
    assert_eq!(BqsqlOperator::BitwiseRightShift, vec![">", ">"]);
    assert_eq!(BqsqlOperator::BitwiseXor, vec!["^"]);
    assert_eq!(BqsqlOperator::Concatenation, vec!["|", "|"]);
    assert_eq!(BqsqlOperator::Division, vec!["/"]);
    assert_eq!(BqsqlOperator::Equal, vec!["="]);
    assert_eq!(BqsqlOperator::GreaterThan, vec![">"]);
    assert_eq!(BqsqlOperator::GreaterThanOrEqualTo, vec![">", "="]);
    assert_eq!(BqsqlOperator::In, vec!["in"]);
    assert_eq!(BqsqlOperator::IsFalse, vec!["is", "false"]);
    assert_eq!(BqsqlOperator::IsNotFalse, vec!["is", "not", "false"]);
    assert_eq!(BqsqlOperator::IsNotNull, vec!["is", "not", "null"]);
    assert_eq!(BqsqlOperator::IsNotTrue, vec!["is", "not", "true"]);
    assert_eq!(BqsqlOperator::IsNull, vec!["is", "null"]);
    assert_eq!(BqsqlOperator::IsTrue, vec!["is", "true"]);
    assert_eq!(BqsqlOperator::LessThan, vec!["<"]);
    assert_eq!(BqsqlOperator::LessThanOrEqualTo, vec!["<", "="]);
    assert_eq!(BqsqlOperator::Like, vec!["like"]);
    assert_eq!(BqsqlOperator::Multiplication, vec!["*"]);
    assert_eq!(BqsqlOperator::Not, vec!["not"]);
    assert_eq!(BqsqlOperator::NotBetween, vec!["not", "between"]);
    assert_eq!(BqsqlOperator::NotEqual1, vec!["!", "="]);
    assert_eq!(BqsqlOperator::NotEqual2, vec!["<", ">"]);
    assert_eq!(BqsqlOperator::NotIn, vec!["not", "in"]);
    assert_eq!(BqsqlOperator::NotLike, vec!["not", "like"]);
    assert_eq!(BqsqlOperator::Or, vec!["or"]);
    assert_eq!(BqsqlOperator::Subtraction, vec!["-"]);

    assert_eq!(vec!["+"], BqsqlOperator::Addition);
    assert_eq!(vec!["and"], BqsqlOperator::And);
    assert_eq!(vec!["Between"], BqsqlOperator::Between);
}
