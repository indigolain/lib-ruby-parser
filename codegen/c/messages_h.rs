use crate::codegen::c::helpers;

fn contents() -> String {
    let messages = lib_ruby_parser_nodes::messages().messages();

    format!(
        "#ifndef LIB_RUBY_PARSER_EXTERNAL_C_SHARED_MESSAGES_H
#define LIB_RUBY_PARSER_EXTERNAL_C_SHARED_MESSAGES_H

// This file is autogenerated by {generator}

#include \"string_ptr.h\"
#include \"byte.h\"
#include \"declare_blob.h\"
#include <stdbool.h>

{structs}

typedef enum {{
    {enum_variants}
}} DiagnosticMessageTag;

typedef struct {{
    DiagnosticMessageTag tag;
    union {{
        {union_members}
    }} as;
}} DiagnosticMessage;

_Static_assert(sizeof(DiagnosticMessage) == 40, \"wrong sizeof(DiagnosticMessage) == 40\");
DECLARE_BLOB_FOR(DiagnosticMessage);

{constructors}

{getters}

{predicates}

#endif // LIB_RUBY_PARSER_EXTERNAL_C_SHARED_MESSAGES_H
",
        generator = file!(),
        structs = messages.map(&struct_definition).join("\n\n"),
        enum_variants = messages.map(&enum_variant).join(",\n    "),
        union_members = messages.map(&union_member).join("\n        "),
        constructors = messages.map(&constructor).join("\n"),
        getters = messages.flat_map(&getters).join("\n"),
        predicates = messages.map(&predicate).join("\n")
    )
}

pub(crate) fn codegen() {
    std::fs::write("external/c/messages.h", contents()).unwrap();
}

fn struct_definition(message: &lib_ruby_parser_nodes::Message) -> String {
    let fields_declaration = {
        let decls = message.fields.map(&|field| {
            let type_name = helpers::messages::field_type(field);
            format!(
                "{t} {name};",
                t = type_name,
                name = helpers::messages::field_name(field)
            )
        });

        if decls.is_empty() {
            String::from("")
        } else {
            format!("\n    {}", decls.join("\n    "))
        }
    };

    format!(
        "typedef struct
{{{fields_declaration}
}} {name};",
        name = message.camelcase_name(),
        fields_declaration = fields_declaration
    )
}
fn enum_variant(message: &lib_ruby_parser_nodes::Message) -> String {
    message.upper_name()
}
fn union_member(message: &lib_ruby_parser_nodes::Message) -> String {
    format!(
        "{struct_name} {variant_name};",
        struct_name = message.camelcase_name(),
        variant_name = message.lower_name()
    )
}
fn constructor(message: &lib_ruby_parser_nodes::Message) -> String {
    format!(
        "{signature};",
        signature = helpers::messages::constructor_signature(message)
    )
}
fn getters(message: &lib_ruby_parser_nodes::Message) -> Vec<String> {
    message.fields.map(&|field| {
        format!(
            "{signature};",
            signature = helpers::messages::getter_signature(message, field)
        )
    })
}
fn predicate(message: &lib_ruby_parser_nodes::Message) -> String {
    format!(
        "{signature};",
        signature = helpers::messages::type_predicate_signature(message)
    )
}