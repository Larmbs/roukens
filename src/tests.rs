
/// Testing date_in_range
use super::date_in_range;

#[test]
fn date_in_range_pass() {
   // Hours to roll back start time
   let hours_setback = chrono::Duration::hours(19);
   let start_time = chrono::Utc::now() - hours_setback;

   // Range that time frame takes place in
   let range = chrono::Duration::hours(20);
   
   let result = date_in_range(&start_time, range);

   assert!(result);
}

#[test]
fn date_in_range_fail() {
    // Hours to roll back start time
    let hours_setback = chrono::Duration::hours(21);
    let start_time = chrono::Utc::now() - hours_setback;

    // Range that time frame takes place in
    let range = chrono::Duration::hours(20);
    
    let result = date_in_range(&start_time, range);

    assert_eq!(false, result);
}

#[test]
fn date_in_range_edge_fail() {
    // Hours to roll back start time
    let hours_setback = chrono::Duration::hours(20);
    let start_time = chrono::Utc::now() - hours_setback;

    // Range that time frame takes place in
    let range = chrono::Duration::hours(20);
    
    let result = date_in_range(&start_time, range);

    assert_eq!(false, result);
}

/// Testing token generator
use super::token_gen::{TokenGen, DEFAULT_CHARSET};

#[test]
/// Tests if generator returns correct length token
fn token_generator_default_pass() {
    let desired_len = 15;
    let generator = TokenGen::new(desired_len, DEFAULT_CHARSET.to_string());
    let token = generator.generate_token();

    assert_eq!(token.len() as u32, desired_len);
}

/// Testing token cache
use super::TokenCache;

#[test]
fn token_cache_inserting() {
    let mut token_cache:TokenCache<String> = TokenCache::new();
    token_cache.insert("User1".to_string(), "Hello There".to_string());

    assert_eq!(token_cache.cache_count(), 1);

    let data = token_cache.get_user_data(&"User1".to_string()).unwrap();
    
    assert_eq!(data, &"Hello There".to_string());
}

#[test]
fn token_cache_clear() {
    let mut token_cache:TokenCache<String> = TokenCache::new();
    token_cache.insert("User1".to_string(), "Hello There".to_string());
    token_cache.insert("User2".to_string(), "Hi There".to_string());
    token_cache.insert("User3".to_string(), "Whats up".to_string());

    assert_eq!(token_cache.cache_count(), 3);

    token_cache.clear();
    
    assert_eq!(token_cache.cache_count(), 0);
}

#[test]
fn token_cache_validate_token_pass() {
    let mut token_cache:TokenCache<String> = TokenCache::new();
    let token = token_cache.insert("User1".to_string(), "Hello There".to_string());

    assert!(token_cache.valid_user_token(&"User1".to_string(), &token));
}

#[test]
fn token_cache_validate_token_fail() {
    let mut token_cache:TokenCache<String> = TokenCache::new();
    token_cache.insert("User1".to_string(), "Hello There".to_string());

    assert_eq!(false, token_cache.valid_user_token(&"User1".to_string(), &String::from("This is an incorrect token")));
}
