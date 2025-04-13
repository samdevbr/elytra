use crate::cypher::ToCypher;

#[derive(Debug)]
pub struct Label {
    ident: String,
}

impl ToCypher for Label {
    fn to_cypher(self) -> String {
        self.ident.to_owned()
    }
}

impl<T> ToCypher for T
where
    T: Iterator<Item = Label>,
{
    fn to_cypher(self) -> String {
        let labels = self.map(|l| l.to_cypher()).collect::<Vec<_>>();

        format!(":{}", labels.join(":"))
    }
}
