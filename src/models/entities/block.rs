use rocket::serde::{Deserialize, Serialize};

use crate::models::blocks::{heading::Heading, text::{Text}};

#[derive(Debug, Deserialize, Serialize)]
#[serde(
    crate = "rocket::serde",
    tag = "type",
    content = "content",
    rename_all = "lowercase"
)]
pub enum Block {
    Doc(Vec<Block>),

    // Simple paragraph
    Paragraph(Vec<Text>),

    // Headings
    Heading {
        heading: Heading 
    },
}

#[cfg(test)]
mod tests {
    use crate::models::blocks::{heading::{Attrs, Heading}, text::Text};
    use serde_json::{from_str, to_string_pretty, to_string};

    use super::Block;
    use super::super::test_str::TEST_VAL;

    #[test]
    fn block_schema() {
        let block = Block::Doc(vec![
            Block::Paragraph(vec![Text {
                text: "Hello World".to_string(),
                marks: None,
            }]),
            Block::Heading{ 
                heading: Heading {
                attrs: Attrs { level: 2 },
                content: vec![Text {
                    text: "Hello".to_string(),
                    marks: None
                }]
            }},
        ]);
        let res = to_string_pretty::<Block>(&block).unwrap();
        println!("{}", res);
    }

    #[test]
    fn block_deser_and_ser() {
        let test = TEST_VAL;
        let res = from_str::<Block>(test).unwrap();
        let serialized = to_string(&res).unwrap();
        assert_eq!(test, serialized.as_str())
    }
}
