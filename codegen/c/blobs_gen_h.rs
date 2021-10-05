use lib_ruby_parser_nodes::template::*;

const TEMPLATE: &str = "#ifndef LIB_RUBY_PARSER_EXTERNAL_C_BLOBS_H
#define LIB_RUBY_PARSER_EXTERNAL_C_BLOBS_H

// This file is autogenerated by <helper generated-by>

#include \"structs.h\"
#include \"declare_blob.h\"

// Nodes
<each-node>
DECLARE_BLOB_FOR(LIB_RUBY_PARSER_<helper node-camelcase-name>);
#define UNPACK_<helper node-camelcase-name>(blob) ((LIB_RUBY_PARSER_<helper node-camelcase-name>_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_<helper node-camelcase-name>(value) ((LIB_RUBY_PARSER_<helper node-camelcase-name>_BLOB_UNION){.as_value = value}).as_blob
</each-node>

// Messages
<each-message>
DECLARE_BLOB_FOR(LIB_RUBY_PARSER_<helper message-camelcase-name>);
#define UNPACK_<helper message-camelcase-name>(blob) ((LIB_RUBY_PARSER_<helper message-camelcase-name>_BLOB_UNION){.as_blob = blob}).as_value
#define PACK_<helper message-camelcase-name>(value) ((LIB_RUBY_PARSER_<helper message-camelcase-name>_BLOB_UNION){.as_value = value}).as_blob
</each-message>

#endif // LIB_RUBY_PARSER_EXTERNAL_C_BLOBS_H";

pub(crate) fn codegen() {
    let template = TemplateRoot::new(TEMPLATE).unwrap();
    let fns = crate::codegen::fns::default_fns!();

    let contents = template.render(ALL_DATA, &fns);
    std::fs::write("external/c/blobs_gen.h", contents).unwrap();
}
