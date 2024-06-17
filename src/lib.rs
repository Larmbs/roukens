use hashbrown::HashMap;
use chrono::{Duration, DateTime, Utc};
pub mod token_gen;
use token_gen::{TokenGen, DEFAULT_CHARSET};

#[inline]
/// Getting current UTC time
fn get_current_time() -> DateTime<Utc> {
    Utc::now()
}

/// Checks if item has expired its lifetime
fn date_in_range(start_time: &DateTime<Utc>, timeout: Duration) -> bool {
    let current_time = Utc::now();
    let within_range = start_time < &current_time && current_time <= *start_time + timeout;
    within_range
}

/// Represents a pool of user tokens that can help speed up identification
pub struct TokenCache<T> {
    token_gen: TokenGen,
    entry_lifetime: Duration,
    cache: HashMap<String, (String, DateTime<Utc>, T)>
}

impl<T> TokenCache<T> {
    /// Creates new token cache object
    pub fn new() -> Self {
        let token_gen = TokenGen::new(30, DEFAULT_CHARSET.to_string());
        let entry_lifetime = Duration::days(1);
        let cache = HashMap::new();

        Self {
            token_gen,
            entry_lifetime,
            cache,
        }
    }

    /// Creates new token cache object but user can specify lifetime
    pub fn new_custom_time(token_gen: TokenGen, entry_lifetime: Duration) -> Self {
        Self {
            token_gen,
            entry_lifetime,
            cache: HashMap::new(),
        }
    }

    /// Gets amount of cached entries
    pub fn cache_count(&self) -> usize {
        self.cache.len()
    }

    /// Clears all entires in cache
    pub fn clear(&mut self) {
        self.cache = HashMap::new();
    }

    /// Cleans out expired tokens
    pub fn clean(&mut self) {
        // Removes all instances where time has expired
        self.cache.retain(|_, (_, time, _)| date_in_range(time, self.entry_lifetime));
    }

    /// Inserts a user into cache and returns users new token
    pub fn insert(&mut self, user_id: String, data: T) -> String {
        // Generate a token
        let token = self.token_gen.generate_token();
        
        // Insert user into cache
        self.cache.insert(user_id, (token.clone(), get_current_time(), data));

        // Return token
        token
    }

    /// Checks if user has a valid token
    pub fn valid_user_token(&mut self, user_id: &String, user_token: &String) -> bool {
        match self.cache.get(user_id) {
            None => false, // If user does not exist in cache
            Some((correct_token, time, _)) => {
                // Checks if token is the correct one
                if correct_token != user_token {return false;};

                // Checks if it is expired, if so clean entires
                if !date_in_range(time, self.entry_lifetime) {self.clean(); return false;}

                // Else return false
                true
            }
        }
    }

    /// Get user cached token data (Note: assumes user token has already been verified)
    pub fn get_user_data(&self, user_id: &String) -> Option<&T> {
        let (_, _, data) = self.cache.get(user_id)?;
        Some(data)
    }
}

#[cfg(test)]
mod tests;
