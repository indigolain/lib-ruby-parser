#include "bytes.h"
#include "list.h"
#include "impl_blob.h"

IMPL_BLOB(BYTES);

BYTES_BLOB_DATA lib_ruby_parser_bytes_blob_from_list_blob(LIST_OF_Byte_BLOB_DATA list_blob)
{
    BYTES bytes = {.raw = list_blob};
    return PACK_BYTES(bytes);
}

extern void drop_u8(void *p) { (void)p; }

void lib_ruby_parser_bytes_blob_free(BYTES_BLOB_DATA bytes_blob)
{
    lib_ruby_parser_containers_byte_list_blob_free(UNPACK_BYTES(bytes_blob).raw, drop_u8);
}
BYTES_BLOB_DATA lib_ruby_parser_bytes_blob_new()
{
    BYTES bytes = {.raw = lib_ruby_parser_containers_byte_list_blob_new()};
    return PACK_BYTES(bytes);
}
LIST_OF_Byte_BLOB_DATA lib_ruby_parser_list_blob_from_bytes_blob(BYTES_BLOB_DATA bytes_blob)
{
    return UNPACK_BYTES(bytes_blob).raw;
}