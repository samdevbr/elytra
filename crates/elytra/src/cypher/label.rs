use crate::cypher::ToCypher;

#[derive(Debug)]
pub struct Label {
    ident: String,
}

impl Label {
    pub fn new<I>(ident: I) -> Self
    where
        I: AsRef<str>,
    {
        Self {
            ident: ident.as_ref().to_string(),
        }
    }
}

pub fn label<I>(ident: I) -> Label
where
    I: AsRef<str>,
{
    Label::new(ident)
}

impl ToCypher for Label {
    fn to_cypher(self) -> String {
        self.ident
    }
}

impl<T> ToCypher for T
where
    T: Iterator<Item = Label>,
{
    ///
    /// ```rust
    /// use elytra::cypher::{self, ToCypher};
    ///
    /// let label = cypher::label("Person");
    /// assert_eq!("Person", label.to_cypher());
    ///
    /// let label_expr = vec![cypher::label("Person")];
    /// assert_eq!(":Person", label_expr.into_iter().to_cypher());
    ///
    /// let label_expr = vec![cypher::label("Person"), cypher::label("Employee")];
    /// assert_eq!(":Person:Employee", label_expr.into_iter().to_cypher());
    /// ```
    ///
    fn to_cypher(self) -> String {
        let labels = self.map(ToCypher::to_cypher).collect::<Vec<_>>();

        format!(":{}", labels.join(":"))
    }
}
