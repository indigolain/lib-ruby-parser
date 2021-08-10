fn contents() -> String {
    let nodes = lib_ruby_parser_nodes::nodes();

    format!(
        "// This file is autogenerated by {generator}

use crate::nodes::InnerNode;
use crate::nodes::*;
use crate::Loc;
use crate::Bytes;

/// Generic combination of all known nodes.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub enum Node {{
{variants}
}}

impl Node {{
    pub(crate) fn inner_ref(&self) -> &dyn InnerNode {{
        match &self {{
            {match_branches}
        }}
    }}

    // make_<node> FNs
    {make_fns}

    {getters}
}}
",
        generator = file!(),
        variants = nodes.map(&variant).join(",\n    "),
        match_branches = nodes.map(&match_branch).join("\n            "),
        make_fns = nodes.map(&make_fn).join("\n    "),
        getters = nodes.map(&getter).join("\n    ")
    )
}

pub(crate) fn codegen() {
    std::fs::write("src/nodes/node_enum/native.rs", contents()).unwrap();
}

fn variant(node: &lib_ruby_parser_nodes::Node) -> String {
    format!("{name}({name})", name = node.camelcase_name())
}

fn match_branch(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "Node::{name}(inner) => inner,",
        name = node.camelcase_name()
    )
}

fn getter(node: &lib_ruby_parser_nodes::Node) -> String {
    let as_ref_mid = format!("as_{}", node.lower_name()).replace("__", "_");
    let as_ref_mut_mid = format!("as_{}_mut", node.lower_name()).replace("__", "_");

    format!(
        "/// Returns true if `self` is `Node::{node_type}`
    pub fn is_{lower_node_type}(&self) -> bool {{ matches!(self, Self::{node_type}(_)) }}
    /// Casts `&Node` to `Option<&nodes::{node_type}>`
    pub fn {as_ref_mid}(&self) -> Option<&{node_type}> {{
        if let Self::{node_type}(inner) = self {{
            Some(inner)
        }} else {{
            None
        }}
    }}
    /// Casts `&Node` to `Option<&mut nodes::{node_type}>`
    pub fn {as_ref_mut_mid}(&mut self) -> Option<&mut {node_type}> {{
        if let Self::{node_type}(inner) = self {{
            Some(inner)
        }} else {{
            None
        }}
    }}
    /// Casts `self` to `nodes::{node_type}`, panics if variant doesn't match
    pub fn into_{lower_node_type}(self) -> {node_type} {{
        if let Self::{node_type}(inner) = self {{
            inner
        }} else {{
            panic!(\"bug: expected type {node_type}, got {{:?}}\", self)
        }}
    }}
",
        lower_node_type = node.lower_name(),
        node_type = node.camelcase_name(),
        as_ref_mid = as_ref_mid,
        as_ref_mut_mid = as_ref_mut_mid,
    )
}

fn make_fn(node: &lib_ruby_parser_nodes::Node) -> String {
    let arglist = node
        .fields
        .map(&|field| {
            format!(
                "{name}: {t}",
                name = field.field_name,
                t = field_type(field)
            )
        })
        .join(", ");

    let fields = node
        .fields
        .map(&|field| field.field_name.to_string())
        .join(", ");

    format!(
        "/// Constructs `Node::{node_type}` variant
    pub(crate) fn make_{lower_node_type}({arglist}) -> Self {{
        Self::{node_type}({node_type} {{ {fields} }})
    }}",
        node_type = node.camelcase_name(),
        lower_node_type = node.lower_name(),
        arglist = arglist,
        fields = fields
    )
}

fn field_type(field: &lib_ruby_parser_nodes::NodeField) -> &str {
    use lib_ruby_parser_nodes::NodeFieldType;

    match field.field_type {
        NodeFieldType::Node => "Box<Node>",
        NodeFieldType::Nodes => "Vec<Node>",
        NodeFieldType::MaybeNode => "Option<Box<Node>>",
        NodeFieldType::Loc => "Loc",
        NodeFieldType::MaybeLoc => "Option<Loc>",
        NodeFieldType::Str => "String",
        NodeFieldType::MaybeStr => "Option<String>",
        NodeFieldType::Chars => "Option<String>",
        NodeFieldType::StringValue => "Bytes",
        NodeFieldType::U8 => "u8",
        NodeFieldType::Usize => "usize",
        NodeFieldType::RawString => "String",
        NodeFieldType::RegexOptions => "Option<Box<Node>>",
    }
}
