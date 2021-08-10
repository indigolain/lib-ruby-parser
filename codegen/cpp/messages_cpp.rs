use crate::codegen::c::helpers as c_helpers;
use crate::codegen::cpp::helpers;

fn contents() -> String {
    let messages = lib_ruby_parser_nodes::messages();

    format!(
        "// This file is autogenerated by {generator}
#include \"messages.hpp\"
#include \"impl_blob.hpp\"
#include \"forget.hpp\"

IMPL_BLOB(DiagnosticMessage);

{cpp_constructors}

DiagnosticMessage::DiagnosticMessage(diagnostic_message_variant_t variant): variant(std::move(variant)) {{}}

extern \"C\"
{{
    {extern_c_constructors}

    {extern_c_getters}

    {extern_c_predicates}
}}
",
        generator = file!(),
        cpp_constructors = messages.map(&cpp_constructor).join("\n"),
        extern_c_constructors = messages.map(&extern_c_constructor).join("\n    "),
        extern_c_getters = messages.flat_map(&extern_c_getters).join("\n    "),
        extern_c_predicates = messages.map(&extern_c_predicate).join("\n    ")
    )
}

pub(crate) fn codegen() {
    std::fs::write("external/cpp/messages.cpp", contents()).unwrap();
}

fn cpp_constructor(message: &lib_ruby_parser_nodes::Message) -> String {
    let initializer_list = message.fields.map(&|field| {
        format!(
            "{name}(std::move({name}))",
            name = helpers::messages::field_name(field)
        )
    });

    let initializer_list = if initializer_list.is_empty() {
        format!("")
    } else {
        format!(" : {}", initializer_list.join(", "))
    };

    format!(
        "{name}::{name}({constructor_arglist}){initializer_list} {{}}",
        name = message.camelcase_name(),
        constructor_arglist = helpers::messages::constructor_arglist(message),
        initializer_list = initializer_list
    )
}
fn extern_c_constructor(message: &lib_ruby_parser_nodes::Message) -> String {
    let unpack_args = message
        .fields
        .map(&|field| format!("UNPACK({})", helpers::messages::field_name(field)));

    format!(
        "{signature}
    {{
        return PACK(DiagnosticMessage({variant_name}({unpack_args})));
    }}",
        signature = c_helpers::messages::constructor::sig(message),
        variant_name = message.camelcase_name(),
        unpack_args = unpack_args.join(", ")
    )
}
fn extern_c_getters(message: &lib_ruby_parser_nodes::Message) -> Vec<String> {
    message.fields.map(&|field| {
        let get_byte_field = format!(
            "return variant->{field_name};",
            field_name = helpers::messages::field_name(field)
        );

        let get_string_field = format!(
            "return (STRING_PTR_BLOB *)(&variant->{field_name});",
            field_name = helpers::messages::field_name(field)
        );

        let get_field = match field.field_type {
            lib_ruby_parser_nodes::MessageFieldType::Str => get_string_field,
            lib_ruby_parser_nodes::MessageFieldType::Byte => get_byte_field,
        };

        format!(
            "{signature}
    {{
        {variant} *variant = std::get_if<{variant}>(&((DiagnosticMessage *)blob)->variant);
        {get_field}
    }}",
            signature = c_helpers::messages::getter::sig(message, field),
            variant = message.camelcase_name(),
            get_field = get_field
        )
    })
}
fn extern_c_predicate(message: &lib_ruby_parser_nodes::Message) -> String {
    format!(
        "{signature}
    {{
        return std::get_if<{variant_name}>(&((DiagnosticMessage *)blob)->variant) != nullptr;
    }}",
        signature = c_helpers::messages::type_predicate::sig(message),
        variant_name = message.camelcase_name()
    )
}
