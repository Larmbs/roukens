
/// Default charset for TokenGen
pub const DEFAULT_CHARSET: &str = concat!("abcdefghijklmnopqrstuvwxyz", "ABCDEFGHIJKLMNOPQRSTUVWXYZ", "0123456789");

/// Helps create tokens of correct security
pub struct TokenGen {
    token_len: u32,
    charset: String,
}

impl TokenGen {
    /// Create new TokenGen instance
    pub fn new(token_len: u32, charset: String) -> Self {
        Self {
            token_len,
            charset
        }
    }

    /// Creates a token
    pub fn generate_token(&self) -> String {
        random_string::generate(self.token_len as usize, &self.charset)
    }
}
