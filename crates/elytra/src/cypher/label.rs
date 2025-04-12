use crate::cypher::ToCypher;

/// https://neo4j.com/docs/cypher-manual/current/patterns/reference/#label-expressions-syntax
#[derive(Debug, Clone)]
pub enum Label {
    /// A simple label identifier (:A)
    Identifier(String),

    /// A logical AND between labels (:A&B)
    And(Box<Self>, Box<Self>),

    /// A logical OR between labels (:A|B)
    Or(Box<Self>, Box<Self>),

    /// A logical NOT of a label: (:!A)
    Not(Box<Self>),

    /// Wildcard (%)
    Any,
}

impl Label {
    pub fn ident<T>(ident: T) -> Self
    where
        T: AsRef<str>,
    {
        Self::Identifier(ident.as_ref().to_string())
    }

    pub fn and(left: Self, right: Self) -> Self {
        Self::And(Box::new(left), Box::new(right))
    }

    pub fn or(left: Self, right: Self) -> Self {
        Self::Or(Box::new(left), Box::new(right))
    }

    pub fn not(label: Self) -> Self {
        Self::Not(Box::new(label))
    }

    pub fn any() -> Self {
        Self::Any
    }
}

impl ToCypher for Label {
    fn to_cypher(&self) -> String {
        match self {
            Label::Identifier(ident) => format!("{ident}"),
            Label::Not(label) => {
                let inner = label.to_cypher();

                if matches!(label.as_ref(), Self::And(_, _) | Self::Or(_, _)) {
                    format!("!({})", inner)
                } else {
                    format!("!{}", inner)
                }
            }
            Label::Any => "%".to_string(),
            Label::Or(left, right) => {
                let left = if matches!(left.as_ref(), Self::And(_, _)) {
                    format!("({})", left.to_cypher())
                } else {
                    left.to_cypher()
                };

                let right = if matches!(right.as_ref(), Self::And(_, _)) {
                    format!("({})", right.to_cypher())
                } else {
                    right.to_cypher()
                };

                format!("{left}|{right}")
            }
            Label::And(left, right) => {
                let left = if matches!(left.as_ref(), Self::Or(_, _)) {
                    format!("({})", left.to_cypher())
                } else {
                    left.to_cypher()
                };

                let right = if matches!(right.as_ref(), Self::Or(_, _)) {
                    format!("({})", right.to_cypher())
                } else {
                    right.to_cypher()
                };

                format!("{left}&{right}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to convert a Label directly to a label expression
    fn to_label_expression(label: &Label) -> String {
        format!(":{}", label.to_cypher())
    }

    #[test]
    fn test_simple_identifier() {
        let label = Label::ident("Person");
        assert_eq!(label.to_cypher(), "Person");
        assert_eq!(to_label_expression(&label), ":Person");
    }

    #[test]
    fn test_wildcard() {
        let label = Label::any();
        assert_eq!(label.to_cypher(), "%");
        assert_eq!(to_label_expression(&label), ":%");
    }

    #[test]
    fn test_not() {
        let label = Label::not(Label::ident("Person"));
        assert_eq!(label.to_cypher(), "!Person");
        assert_eq!(to_label_expression(&label), ":!Person");
    }

    #[test]
    fn test_and() {
        let label = Label::and(Label::ident("Person"), Label::ident("Employee"));
        assert_eq!(label.to_cypher(), "Person&Employee");
        assert_eq!(to_label_expression(&label), ":Person&Employee");
    }

    #[test]
    fn test_or() {
        let label = Label::or(Label::ident("Person"), Label::ident("Employee"));
        assert_eq!(label.to_cypher(), "Person|Employee");
        assert_eq!(to_label_expression(&label), ":Person|Employee");
    }

    #[test]
    fn test_complex_not() {
        // !( A | B )
        let label = Label::not(Label::or(Label::ident("A"), Label::ident("B")));
        assert_eq!(label.to_cypher(), "!(A|B)");
        assert_eq!(to_label_expression(&label), ":!(A|B)");
    }

    #[test]
    fn test_not_with_and() {
        // !( A & B )
        let label = Label::not(Label::and(Label::ident("A"), Label::ident("B")));
        assert_eq!(label.to_cypher(), "!(A&B)");
        assert_eq!(to_label_expression(&label), ":!(A&B)");
    }

    #[test]
    fn test_not_simple() {
        // !A
        let label = Label::not(Label::ident("A"));
        assert_eq!(label.to_cypher(), "!A");
        assert_eq!(to_label_expression(&label), ":!A");
    }

    #[test]
    fn test_or_with_and_left() {
        // (A & B) | C
        let label = Label::or(
            Label::and(Label::ident("A"), Label::ident("B")),
            Label::ident("C"),
        );
        assert_eq!(label.to_cypher(), "(A&B)|C");
        assert_eq!(to_label_expression(&label), ":(A&B)|C");
    }

    #[test]
    fn test_or_with_and_right() {
        // A | (B & C)
        let label = Label::or(
            Label::ident("A"),
            Label::and(Label::ident("B"), Label::ident("C")),
        );
        assert_eq!(label.to_cypher(), "A|(B&C)");
        assert_eq!(to_label_expression(&label), ":A|(B&C)");
    }

    #[test]
    fn test_and_with_or_left() {
        // (A | B) & C
        let label = Label::and(
            Label::or(Label::ident("A"), Label::ident("B")),
            Label::ident("C"),
        );
        assert_eq!(label.to_cypher(), "(A|B)&C");
        assert_eq!(to_label_expression(&label), ":(A|B)&C");
    }

    #[test]
    fn test_and_with_or_right() {
        // A & (B | C)
        let label = Label::and(
            Label::ident("A"),
            Label::or(Label::ident("B"), Label::ident("C")),
        );
        assert_eq!(label.to_cypher(), "A&(B|C)");
        assert_eq!(to_label_expression(&label), ":A&(B|C)");
    }

    #[test]
    fn test_double_and() {
        // A & B & C  (equivalent to (A & B) & C due to left-associativity)
        let label = Label::and(
            Label::and(Label::ident("A"), Label::ident("B")),
            Label::ident("C"),
        );
        assert_eq!(label.to_cypher(), "A&B&C");
        assert_eq!(to_label_expression(&label), ":A&B&C");
    }

    #[test]
    fn test_double_or() {
        // A | B | C  (equivalent to (A | B) | C due to left-associativity)
        let label = Label::or(
            Label::or(Label::ident("A"), Label::ident("B")),
            Label::ident("C"),
        );
        assert_eq!(label.to_cypher(), "A|B|C");
        assert_eq!(to_label_expression(&label), ":A|B|C");
    }

    #[test]
    fn test_complex_expression1() {
        // A & (B | C) & D
        let label = Label::and(
            Label::and(
                Label::ident("A"),
                Label::or(Label::ident("B"), Label::ident("C")),
            ),
            Label::ident("D"),
        );
        assert_eq!(label.to_cypher(), "A&(B|C)&D");
        assert_eq!(to_label_expression(&label), ":A&(B|C)&D");
    }

    #[test]
    fn test_complex_expression2() {
        // A | (B & C & D) | E
        let label = Label::or(
            Label::or(
                Label::ident("A"),
                Label::and(
                    Label::and(Label::ident("B"), Label::ident("C")),
                    Label::ident("D"),
                ),
            ),
            Label::ident("E"),
        );
        assert_eq!(label.to_cypher(), "A|(B&C&D)|E");
        assert_eq!(to_label_expression(&label), ":A|(B&C&D)|E");
    }

    #[test]
    fn test_complex_expression3() {
        // !(A | B) & (C | !D)
        let label = Label::and(
            Label::not(Label::or(Label::ident("A"), Label::ident("B"))),
            Label::or(Label::ident("C"), Label::not(Label::ident("D"))),
        );
        assert_eq!(label.to_cypher(), "!(A|B)&(C|!D)");
        assert_eq!(to_label_expression(&label), ":!(A|B)&(C|!D)");
    }

    #[test]
    fn test_actor_director_example() {
        // Person&(Actor|!Director)
        let label = Label::and(
            Label::ident("Person"),
            Label::or(Label::ident("Actor"), Label::not(Label::ident("Director"))),
        );
        assert_eq!(label.to_cypher(), "Person&(Actor|!Director)");
        assert_eq!(to_label_expression(&label), ":Person&(Actor|!Director)");
    }

    #[test]
    fn test_complex_not_with_and_or() {
        // !(A & (B | C))
        let label = Label::not(Label::and(
            Label::ident("A"),
            Label::or(Label::ident("B"), Label::ident("C")),
        ));
        assert_eq!(label.to_cypher(), "!(A&(B|C))");
        assert_eq!(to_label_expression(&label), ":!(A&(B|C))");
    }

    #[test]
    fn test_complex_nested_and_or() {
        // (A & (B | C)) | (D & E)
        let label = Label::or(
            Label::and(
                Label::ident("A"),
                Label::or(Label::ident("B"), Label::ident("C")),
            ),
            Label::and(Label::ident("D"), Label::ident("E")),
        );
        assert_eq!(label.to_cypher(), "(A&(B|C))|(D&E)");
        assert_eq!(to_label_expression(&label), ":(A&(B|C))|(D&E)");
    }

    #[test]
    fn test_not_any() {
        // !%
        let label = Label::not(Label::any());
        assert_eq!(label.to_cypher(), "!%");
        assert_eq!(to_label_expression(&label), ":!%");
    }

    #[test]
    fn test_any_with_and() {
        // % & Person
        let label = Label::and(Label::any(), Label::ident("Person"));
        assert_eq!(label.to_cypher(), "%&Person");
        assert_eq!(to_label_expression(&label), ":%&Person");
    }

    #[test]
    fn test_any_with_or() {
        // % | Person
        let label = Label::or(Label::any(), Label::ident("Person"));
        assert_eq!(label.to_cypher(), "%|Person");
        assert_eq!(to_label_expression(&label), ":%|Person");
    }
}
