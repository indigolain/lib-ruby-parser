/// An enum with all possible kinds of errors that can be returned
/// from a decoder
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum InputError {
    /// Emitted when no custom decoder provided but input has custom encoding.
    ///
    /// You can return this error from your custom decoder if you don't support given encoding.
    UnsupportedEncoding(String),

    /// Generic error that can be emitted from a custom decoder
    DecodingError(String),
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for InputError {}

/// Result that is returned from decoding function
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DecoderResult {
    /// Ok + decoded bytes
    Ok(Vec<u8>),

    /// Err + reason
    Err(InputError),
}

impl DecoderResult {
    pub(crate) fn into_result(self) -> Result<Vec<u8>, InputError> {
        match self {
            Self::Ok(value) => Ok(value),
            Self::Err(err) => Err(err),
        }
    }
}

/// Decoder is what is used if input source has encoding
/// that is not supported out of the box.
///
/// Supported encoding are:
/// 1. UTF-8
/// 2. ASCII-8BIT (or BINARY, it's an alias)
///
/// So if your source looks like this:
///
/// ```text
/// # encoding: koi8-r
/// \xFF = 42
/// ```
///
/// you need to provide a decoder that converts this byte sequence
/// into UTF-8 bytes.
///
/// Decoding function
///
/// Takes encoding name and initial input as arguments
/// and returns `Ok(decoded)` vector of bytes or `Err(error)` that will be returned
/// in the `ParserResult::diagnostics` vector.
pub type DecoderFn = dyn Fn(String, Vec<u8>) -> DecoderResult;

/// Custom decoder, a wrapper around a function
pub struct Decoder {
    f: Box<DecoderFn>,
}

impl Decoder {
    /// Constructs a rewriter based on a given function
    pub fn new(f: Box<DecoderFn>) -> Self {
        Self { f }
    }

    pub(crate) fn call(&self, encoding: String, input: Vec<u8>) -> DecoderResult {
        let f = &*self.f;
        f(encoding, input)
    }
}

impl std::fmt::Debug for Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decoder").finish()
    }
}

pub fn decode_input(input: Vec<u8>, enc: String, decoder: &mut Option<Decoder>) -> DecoderResult {
    match enc.to_uppercase().as_str() {
        "UTF-8" | "ASCII-8BIT" | "BINARY" => DecoderResult::Ok(input),
        _ => {
            if let Some(f) = decoder.as_mut() {
                f.call(enc, input)
            } else {
                DecoderResult::Err(InputError::UnsupportedEncoding(enc))
            }
        }
    }
}
