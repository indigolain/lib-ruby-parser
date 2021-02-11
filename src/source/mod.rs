pub mod buffer;
mod comment;
mod decoder;
mod magic_comment;
mod source_line;

pub use comment::{Comment, CommentType};
pub(crate) use decoder::decode_input;
pub use decoder::InputError;
pub use decoder::{CustomDecoder, RustFnBasedCustomDecoder};
pub use magic_comment::{MagicComment, MagicCommentKind};
pub(crate) use source_line::SourceLine;
