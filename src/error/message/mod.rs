#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::DiagnosticMessage;

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::DiagnosticMessage;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use external::DiagnosticMessageBlob;

mod render;

#[cfg(test)]
mod tests;
