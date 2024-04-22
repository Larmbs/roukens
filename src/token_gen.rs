
/// Default charset for TokenGen
pub const DEFAULT_CHARSET: &str = concat!("abcdefghijklmnopqrstuvwxyz", "ABCDEFGHIJKLMNOPQRSTUVWXYZ", "0123456789");

/// Helps create tokens of correct security
pub struct TokenGen {
    token_len: usize,
    charset: String,
}

impl TokenGen {
    /// Create new TokenGen instance
    pub fn new(token_len: usize, charset: String) -> Self {
        Self {
            token_len,
            charset
        }
    }

    /// Creates a token
    pub fn generate_token(&self) -> String {
        random_string::generate(self.token_len as usize, &self.charset)
    }

    /// Checks token matches generators constraints
    pub fn is_valid_token(&self, token: &String) -> bool {
        // Return false if lengths dont match
        if token.len() != self.token_len {return false;}

        // Checks that all chars in token are from list
        let res = token.chars().all(|c| self.charset.contains(c));

        res
    }
}
