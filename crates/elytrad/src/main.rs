use elytra::document::Document;

fn main() {
    snowflake::set_node_id(1);

    let doc = Document::new("user");

    dbg!(doc);
}
