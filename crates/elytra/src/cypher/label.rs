use crate::cypher::ToCypher;

/// https://neo4j.com/docs/cypher-manual/current/patterns/reference/#label-expressions-syntax
#[derive(Debug, Clone)]
pub enum Term {
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

impl Term {
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

    pub fn not(term: Self) -> Self {
        Self::Not(Box::new(term))
    }

    pub fn any() -> Self {
        Self::Any
    }
}

impl ToCypher for Term {
    fn to_cypher(&self) -> String {
        match self {
            Term::Identifier(ident) => format!("{ident}"),
            Term::Not(term) => {
                let inner = term.to_cypher();

                if matches!(term.as_ref(), Self::And(_, _) | Self::Or(_, _)) {
                    format!("!({})", inner)
                } else {
                    format!("!{}", inner)
                }
            }
            Term::Any => "%".to_string(),
            Term::Or(left, right) => {
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
            Term::And(left, right) => {
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

    // Helper function to convert a Term directly to a label expression
    fn to_label_expression(term: &Term) -> String {
        format!(":{}", term.to_cypher())
    }

    #[test]
    fn test_simple_identifier() {
        let term = Term::ident("Person");
        assert_eq!(term.to_cypher(), "Person");
        assert_eq!(to_label_expression(&term), ":Person");
    }

    #[test]
    fn test_wildcard() {
        let term = Term::any();
        assert_eq!(term.to_cypher(), "%");
        assert_eq!(to_label_expression(&term), ":%");
    }

    #[test]
    fn test_not() {
        let term = Term::not(Term::ident("Person"));
        assert_eq!(term.to_cypher(), "!Person");
        assert_eq!(to_label_expression(&term), ":!Person");
    }

    #[test]
    fn test_and() {
        let term = Term::and(Term::ident("Person"), Term::ident("Employee"));
        assert_eq!(term.to_cypher(), "Person&Employee");
        assert_eq!(to_label_expression(&term), ":Person&Employee");
    }

    #[test]
    fn test_or() {
        let term = Term::or(Term::ident("Person"), Term::ident("Employee"));
        assert_eq!(term.to_cypher(), "Person|Employee");
        assert_eq!(to_label_expression(&term), ":Person|Employee");
    }

    #[test]
    fn test_complex_not() {
        // !( A | B )
        let term = Term::not(Term::or(Term::ident("A"), Term::ident("B")));
        assert_eq!(term.to_cypher(), "!(A|B)");
        assert_eq!(to_label_expression(&term), ":!(A|B)");
    }

    #[test]
    fn test_not_with_and() {
        // !( A & B )
        let term = Term::not(Term::and(Term::ident("A"), Term::ident("B")));
        assert_eq!(term.to_cypher(), "!(A&B)");
        assert_eq!(to_label_expression(&term), ":!(A&B)");
    }

    #[test]
    fn test_not_simple() {
        // !A
        let term = Term::not(Term::ident("A"));
        assert_eq!(term.to_cypher(), "!A");
        assert_eq!(to_label_expression(&term), ":!A");
    }

    #[test]
    fn test_or_with_and_left() {
        // (A & B) | C
        let term = Term::or(
            Term::and(Term::ident("A"), Term::ident("B")),
            Term::ident("C"),
        );
        assert_eq!(term.to_cypher(), "(A&B)|C");
        assert_eq!(to_label_expression(&term), ":(A&B)|C");
    }

    #[test]
    fn test_or_with_and_right() {
        // A | (B & C)
        let term = Term::or(
            Term::ident("A"),
            Term::and(Term::ident("B"), Term::ident("C")),
        );
        assert_eq!(term.to_cypher(), "A|(B&C)");
        assert_eq!(to_label_expression(&term), ":A|(B&C)");
    }

    #[test]
    fn test_and_with_or_left() {
        // (A | B) & C
        let term = Term::and(
            Term::or(Term::ident("A"), Term::ident("B")),
            Term::ident("C"),
        );
        assert_eq!(term.to_cypher(), "(A|B)&C");
        assert_eq!(to_label_expression(&term), ":(A|B)&C");
    }

    #[test]
    fn test_and_with_or_right() {
        // A & (B | C)
        let term = Term::and(
            Term::ident("A"),
            Term::or(Term::ident("B"), Term::ident("C")),
        );
        assert_eq!(term.to_cypher(), "A&(B|C)");
        assert_eq!(to_label_expression(&term), ":A&(B|C)");
    }

    #[test]
    fn test_double_and() {
        // A & B & C  (equivalent to (A & B) & C due to left-associativity)
        let term = Term::and(
            Term::and(Term::ident("A"), Term::ident("B")),
            Term::ident("C"),
        );
        assert_eq!(term.to_cypher(), "A&B&C");
        assert_eq!(to_label_expression(&term), ":A&B&C");
    }

    #[test]
    fn test_double_or() {
        // A | B | C  (equivalent to (A | B) | C due to left-associativity)
        let term = Term::or(
            Term::or(Term::ident("A"), Term::ident("B")),
            Term::ident("C"),
        );
        assert_eq!(term.to_cypher(), "A|B|C");
        assert_eq!(to_label_expression(&term), ":A|B|C");
    }

    #[test]
    fn test_complex_expression1() {
        // A & (B | C) & D
        let term = Term::and(
            Term::and(
                Term::ident("A"),
                Term::or(Term::ident("B"), Term::ident("C")),
            ),
            Term::ident("D"),
        );
        assert_eq!(term.to_cypher(), "A&(B|C)&D");
        assert_eq!(to_label_expression(&term), ":A&(B|C)&D");
    }

    #[test]
    fn test_complex_expression2() {
        // A | (B & C & D) | E
        let term = Term::or(
            Term::or(
                Term::ident("A"),
                Term::and(
                    Term::and(Term::ident("B"), Term::ident("C")),
                    Term::ident("D"),
                ),
            ),
            Term::ident("E"),
        );
        assert_eq!(term.to_cypher(), "A|(B&C&D)|E");
        assert_eq!(to_label_expression(&term), ":A|(B&C&D)|E");
    }

    #[test]
    fn test_complex_expression3() {
        // !(A | B) & (C | !D)
        let term = Term::and(
            Term::not(Term::or(Term::ident("A"), Term::ident("B"))),
            Term::or(Term::ident("C"), Term::not(Term::ident("D"))),
        );
        assert_eq!(term.to_cypher(), "!(A|B)&(C|!D)");
        assert_eq!(to_label_expression(&term), ":!(A|B)&(C|!D)");
    }

    #[test]
    fn test_actor_director_example() {
        // Person&(Actor|!Director)
        let term = Term::and(
            Term::ident("Person"),
            Term::or(Term::ident("Actor"), Term::not(Term::ident("Director"))),
        );
        assert_eq!(term.to_cypher(), "Person&(Actor|!Director)");
        assert_eq!(to_label_expression(&term), ":Person&(Actor|!Director)");
    }

    #[test]
    fn test_complex_not_with_and_or() {
        // !(A & (B | C))
        let term = Term::not(Term::and(
            Term::ident("A"),
            Term::or(Term::ident("B"), Term::ident("C")),
        ));
        assert_eq!(term.to_cypher(), "!(A&(B|C))");
        assert_eq!(to_label_expression(&term), ":!(A&(B|C))");
    }

    #[test]
    fn test_complex_nested_and_or() {
        // (A & (B | C)) | (D & E)
        let term = Term::or(
            Term::and(
                Term::ident("A"),
                Term::or(Term::ident("B"), Term::ident("C")),
            ),
            Term::and(Term::ident("D"), Term::ident("E")),
        );
        assert_eq!(term.to_cypher(), "(A&(B|C))|(D&E)");
        assert_eq!(to_label_expression(&term), ":(A&(B|C))|(D&E)");
    }

    #[test]
    fn test_not_any() {
        // !%
        let term = Term::not(Term::any());
        assert_eq!(term.to_cypher(), "!%");
        assert_eq!(to_label_expression(&term), ":!%");
    }

    #[test]
    fn test_any_with_and() {
        // % & Person
        let term = Term::and(Term::any(), Term::ident("Person"));
        assert_eq!(term.to_cypher(), "%&Person");
        assert_eq!(to_label_expression(&term), ":%&Person");
    }

    #[test]
    fn test_any_with_or() {
        // % | Person
        let term = Term::or(Term::any(), Term::ident("Person"));
        assert_eq!(term.to_cypher(), "%|Person");
        assert_eq!(to_label_expression(&term), ":%|Person");
    }
}
