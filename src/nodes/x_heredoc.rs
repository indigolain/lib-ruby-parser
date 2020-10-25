use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct XHeredoc {
    pub parts: Vec<Node>,

    pub heredoc_body_l: Range,
    pub heredoc_end_l: Range,
    pub expression_l: Range,
}

impl InnerNode for XHeredoc {
    fn expression(&self) -> &Range {
        &self.expression_l
    }

    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_nodes(&self.parts);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "xstr"
    }
}