use crate::codegen::c::helpers as c_helpers;
use crate::codegen::rust::nodes::helpers::{node_field_name, struct_name};

fn contents() -> String {
    let nodes = lib_ruby_parser_nodes::nodes();

    format!(
        "// This file is autogenerated by {generator}

use crate::nodes::InnerNode;
use crate::nodes::*;
use crate::containers::size::NODE_SIZE;

use crate::Loc;
use crate::Bytes;
use crate::containers::ExternalMaybePtr as MaybePtr;
use crate::containers::ExternalPtr as Ptr;
use crate::containers::ExternalList as List;
use crate::containers::ExternalMaybeLoc as MaybeLoc;
use crate::containers::ExternalStringPtr as StringPtr;
use crate::containers::ExternalMaybeStringPtr as MaybeStringPtr;

use crate::loc::LocBlob;
use crate::bytes::BytesBlob;
use crate::containers::MaybePtrBlob;
use crate::containers::PtrBlob;
use crate::containers::ListBlob;
use crate::containers::MaybeLocBlob;
use crate::containers::StringPtrBlob;
use crate::containers::MaybeStringPtrBlob;

use crate::containers::IntoBlob;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct NodeBlob {{
    blob: [u8; NODE_SIZE],
}}

/// Generic combination of all known nodes.
#[repr(C)]
pub struct Node {{
    pub(crate) blob: NodeBlob,
}}

impl std::fmt::Debug for Node {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        {debug_impl}
    }}
}}

impl Clone for Node {{
    fn clone(&self) -> Self {{
        {clone_impl}
    }}
}}

impl PartialEq for Node {{
    fn eq(&self, other: &Self) -> bool {{
        {partial_eq_impl}
    }}
}}

impl IntoBlob for Node {{
    type Output = NodeBlob;

    fn into_blob(self) -> Self::Output {{
        let blob = self.blob;
        std::mem::forget(self);
        blob
    }}
}}

impl Node {{
    pub(crate) fn inner_ref(&self) -> &dyn InnerNode {{
        {inner_ref}

        panic!(\"bug: unknown node type\")
    }}

    // make_<node> FNs
    {make_fns}

    // is_<node> FNs

    {is_fns}

    // as_<node> FNs
    {as_fns}

    // as_<node>_mut FNs
    {as_mut_fns}

    // into_<node> FNs
    {into_fns}
}}

use crate::nodes::blobs::*;
extern \"C\"
{{
    {extern_fns}

    fn lib_ruby_parser__internal__containers__node__drop(blob: *mut NodeBlob);
}}

impl Drop for Node {{
    fn drop(&mut self) {{
        unsafe {{ lib_ruby_parser__internal__containers__node__drop(&mut self.blob) }}
    }}
}}
",
        generator = file!(),
        // trait impls
        debug_impl = debug_impl(&nodes),
        clone_impl = clone_impl(&nodes),
        partial_eq_impl = partial_eq_impl(&nodes),
        // fns
        inner_ref = nodes.map(&inner_ref).join("\n        "),
        make_fns = nodes.map(&make_fn).join("\n    "),
        is_fns = nodes.map(&is_fn).join("\n    "),
        as_fns = nodes.map(&as_fn).join("\n    "),
        as_mut_fns = nodes.map(&as_mut_fn).join("\n    "),
        into_fns = nodes.map(&into_fn).join("\n    "),
        // extern fns
        extern_fns = nodes.flat_map(&extern_fns).join("\n    ")
    )
}

pub(crate) fn codegen() {
    std::fs::write("src/nodes/node_enum/external.rs", contents()).unwrap();
}

fn debug_impl(nodes: &lib_ruby_parser_nodes::NodeList) -> String {
    let branches = nodes
        .map(&|node| {
            format!(
                "if let Some(inner) = self.as_{lower}() {{
            write!(f, \"{struct_name}({{:?}})\", inner)
        }}",
                lower = node.lower_name(),
                struct_name = struct_name(node)
            )
        })
        .join(" else ");

    format!(
        "{branches} else {{
            panic!(\"bug: unknown node type\")
        }}",
        branches = branches
    )
}
fn clone_impl(nodes: &lib_ruby_parser_nodes::NodeList) -> String {
    let branches = nodes
        .map(&|node| {
            let clone_fields = node
                .fields
                .map(&|field| format!("inner.get_{}().clone()", field.field_name))
                .join(", ");

            format!(
                "if let Some(inner) = self.as_{lower}() {{
            Self::make_{lower}({clone_fields})
        }}",
                lower = node.lower_name(),
                clone_fields = clone_fields
            )
        })
        .join(" else ");

    format!(
        "{branches} else {{
            panic!(\"bug: unknown node type\")
        }}",
        branches = branches
    )
}
fn partial_eq_impl(nodes: &lib_ruby_parser_nodes::NodeList) -> String {
    let branches = nodes
        .map(&|node| {
            format!(
                "if let Some(lhs) = self.as_{lower}() {{
            if let Some(rhs) = other.as_{lower}() {{
                lhs == rhs
            }} else {{
                false
            }}
        }}",
                lower = node.lower_name(),
            )
        })
        .join(" else ");

    format!(
        "{branches} else {{
            panic!(\"bug: unknown node type\")
        }}",
        branches = branches
    )
}

fn inner_ref(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "if let Some(inner) = self.as_{lower}() {{
            return inner;
        }}",
        lower = node.lower_name()
    )
}
fn make_fn(node: &lib_ruby_parser_nodes::Node) -> String {
    let arglist = node
        .fields
        .map(&|field| {
            format!(
                "{name}: {t}",
                name = node_field_name(field),
                t = field_type(field)
            )
        })
        .join(", ");

    let fields = node
        .fields
        .map(&|field| format!("{}.into_blob()", node_field_name(field)))
        .join(", ");

    format!(
        "/// Constructs `Node::{node_type}` variant
    pub(crate) fn make_{lower_node_type}({arglist}) -> Self {{
        let blob = unsafe {{ {extern_constructor}({fields}) }};
        Self {{ blob }}
    }}",
        node_type = struct_name(node),
        lower_node_type = node.lower_name(),
        extern_constructor = c_helpers::nodes::constructor::name(node),
        arglist = arglist,
        fields = fields
    )
}
fn is_fn(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "/// Returns true if `self` is `Node::{node_type}`
    pub fn is_{lower_node_type}(&self) -> bool {{
        unsafe {{ {extern_fn_name}(&self.blob) }}
    }}",
        node_type = struct_name(node),
        lower_node_type = node.lower_name(),
        extern_fn_name = c_helpers::nodes::variant_predicate::name(node)
    )
}
fn as_fn(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "/// Casts `&Node` to `Option<&nodes::{node_type}>`
    pub fn as_{lower}(&self) -> Option<&{node_type}> {{
        unsafe {{ ({extern_fn_name}(&self.blob) as *const {node_type}).as_ref() }}
    }}",
        node_type = struct_name(node),
        lower = node.lower_name(),
        extern_fn_name = c_helpers::nodes::variant_getter::name(node)
    )
}
fn as_mut_fn(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "/// Casts `&Node` to `Option<&mut nodes::{node_type}>`
    pub fn as_{lower}_mut(&mut self) -> Option<&mut {node_type}> {{
        unsafe {{ ({extern_fn_name}(&mut self.blob) as *mut {node_type}).as_mut() }}
    }}",
        node_type = struct_name(node),
        lower = node.lower_name(),
        extern_fn_name = c_helpers::nodes::variant_getter::name(node)
    )
}
fn into_fn(node: &lib_ruby_parser_nodes::Node) -> String {
    format!(
        "/// Casts `self` to nodes::{node_type}, panics if variant doesn't match
    pub fn into_{lower}(self) -> {node_type} {{
        let blob = unsafe {{ {into_variant_fn_name}(self.into_blob()) }};
        {node_type} {{ blob }}
    }}",
        node_type = struct_name(node),
        lower = node.lower_name(),
        into_variant_fn_name = c_helpers::nodes::into_variant::name(node),
    )
}

fn extern_fns(node: &lib_ruby_parser_nodes::Node) -> Vec<String> {
    let mut result = vec![];

    // constructor
    {
        let ctor_args = node
            .fields
            .map(&|field| format!("{}: {}", node_field_name(field), blob_type(field)))
            .join(", ");
        result.push(format!(
            "fn {name}({ctor_args}) -> NodeBlob;",
            name = c_helpers::nodes::constructor::name(node),
            ctor_args = ctor_args,
        ));
    }

    // predicates
    {
        result.push(format!(
            "fn {name}(blob_ptr: *const NodeBlob) -> bool;",
            name = c_helpers::nodes::variant_predicate::name(node)
        ))
    }

    // variant getters
    {
        result.push(format!(
            "fn {name}(blob_ptr: *const NodeBlob) -> *mut {node_type}Blob;",
            name = c_helpers::nodes::variant_getter::name(node),
            node_type = struct_name(node)
        ))
    }

    // into_internal fn
    {
        let line = format!(
            "fn {fn_name}(blob: NodeBlob) -> {struct_name}Blob;",
            fn_name = c_helpers::nodes::into_variant::name(node),
            struct_name = struct_name(node)
        );
        result.push(line);
    }

    result
}

fn field_type(field: &lib_ruby_parser_nodes::NodeField) -> &str {
    use lib_ruby_parser_nodes::NodeFieldType;

    match field.field_type {
        NodeFieldType::Node => "Ptr<Node>",
        NodeFieldType::Nodes => "List<Node>",
        NodeFieldType::MaybeNode { .. } => "MaybePtr<Node>",
        NodeFieldType::Loc => "Loc",
        NodeFieldType::MaybeLoc => "MaybeLoc",
        NodeFieldType::Str { .. } => "StringPtr",
        NodeFieldType::MaybeStr { .. } => "MaybeStringPtr",
        NodeFieldType::StringValue => "Bytes",
        NodeFieldType::U8 => "u8",
    }
}

fn blob_type(field: &lib_ruby_parser_nodes::NodeField) -> &str {
    use lib_ruby_parser_nodes::NodeFieldType;

    match field.field_type {
        NodeFieldType::Node => "PtrBlob",
        NodeFieldType::Nodes => "ListBlob",
        NodeFieldType::MaybeNode { .. } => "MaybePtrBlob",
        NodeFieldType::Loc => "LocBlob",
        NodeFieldType::MaybeLoc => "MaybeLocBlob",
        NodeFieldType::Str { .. } => "StringPtrBlob",
        NodeFieldType::MaybeStr { .. } => "MaybeStringPtrBlob",
        NodeFieldType::StringValue => "BytesBlob",
        NodeFieldType::U8 => "u8",
    }
}