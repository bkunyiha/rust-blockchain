use crate::primitives::transaction::Transaction;

/// Functional utilities for transaction operations
pub mod transaction {
    use super::*;

    /// Process a collection of transactions functionally
    pub fn process_transactions<F>(transactions: &[Transaction], processor: F) -> Vec<Transaction>
    where
        F: Fn(&Transaction) -> bool,
    {
        transactions
            .iter()
            .filter(|tx| processor(tx))
            .cloned()
            .collect()
    }

    /// Transform transactions using a mapping function
    pub fn transform_transactions<F, T>(transactions: &[Transaction], mapper: F) -> Vec<T>
    where
        F: Fn(&Transaction) -> T,
    {
        transactions.iter().map(mapper).collect()
    }

    /// Validate transactions using a predicate
    pub fn validate_transactions<F>(transactions: &[Transaction], validator: F) -> bool
    where
        F: Fn(&Transaction) -> bool,
    {
        transactions.iter().all(validator)
    }

    /// Find transactions matching a predicate
    pub fn find_transactions<F>(transactions: &[Transaction], predicate: F) -> Vec<&Transaction>
    where
        F: Fn(&Transaction) -> bool,
    {
        transactions.iter().filter(|tx| predicate(tx)).collect()
    }
}

/// Functional utilities for error handling
pub mod error_handling {
    /// Execute a function and map the result to a different type
    pub fn map_result<T, U, F>(
        result: crate::error::Result<T>,
        mapper: F,
    ) -> crate::error::Result<U>
    where
        F: FnOnce(T) -> U,
    {
        result.map(mapper)
    }

    /// Execute a function and map the error to a different type
    pub fn map_err<T, F>(result: crate::error::Result<T>, mapper: F) -> crate::error::Result<T>
    where
        F: FnOnce(crate::error::BtcError) -> crate::error::BtcError,
    {
        result.map_err(mapper)
    }

    /// Execute a function and handle the result with a handler
    pub fn handle_result<T, F, G>(
        result: crate::error::Result<T>,
        handler: F,
        error_handler: G,
    ) -> T
    where
        F: FnOnce(T) -> T,
        G: FnOnce(crate::error::BtcError) -> T,
    {
        result.map(handler).unwrap_or_else(error_handler)
    }
}

/// Functional utilities for collections
pub mod collections {
    /// Transform a collection using a mapping function
    pub fn map_collection<T, U, F>(collection: &[T], mapper: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        collection.iter().map(mapper).collect()
    }

    /// Filter a collection using a predicate
    pub fn filter_collection<T, F>(collection: &[T], predicate: F) -> Vec<&T>
    where
        F: Fn(&T) -> bool,
    {
        collection.iter().filter(|item| predicate(item)).collect()
    }

    /// Reduce a collection using a reducer function
    pub fn reduce_collection<T, F>(collection: &[T], initial: T, reducer: F) -> T
    where
        F: Fn(T, &T) -> T,
    {
        collection.iter().fold(initial, reducer)
    }

    /// Check if all elements in a collection satisfy a predicate
    pub fn all_elements<T, F>(collection: &[T], predicate: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        collection.iter().all(predicate)
    }

    /// Check if any element in a collection satisfies a predicate
    pub fn any_element<T, F>(collection: &[T], predicate: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        collection.iter().any(predicate)
    }
}

/// Functional utilities for option handling
pub mod option_utils {
    /// Transform an option using a mapping function
    pub fn map_option<T, U, F>(option: Option<T>, mapper: F) -> Option<U>
    where
        F: FnOnce(T) -> U,
    {
        option.map(mapper)
    }

    /// Transform an option using a mapping function that returns an option
    pub fn and_then_option<T, U, F>(option: Option<T>, mapper: F) -> Option<U>
    where
        F: FnOnce(T) -> Option<U>,
    {
        option.and_then(mapper)
    }

    /// Provide a default value for an option
    pub fn unwrap_or_default<T>(option: Option<T>) -> T
    where
        T: Default,
    {
        option.unwrap_or_default()
    }
}

/// Functional utilities for string operations
pub mod string_utils {
    /// Transform a string using a mapping function
    pub fn map_string<F>(s: &str, mapper: F) -> String
    where
        F: FnOnce(&str) -> String,
    {
        mapper(s)
    }

    /// Filter characters from a string using a predicate
    pub fn filter_string<F>(s: &str, predicate: F) -> String
    where
        F: FnMut(&char) -> bool,
    {
        s.chars().filter(predicate).collect()
    }

    /// Transform characters in a string using a mapping function
    pub fn map_chars<F>(s: &str, mapper: F) -> String
    where
        F: Fn(char) -> char,
    {
        s.chars().map(mapper).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_collection() {
        let numbers = vec![1, 2, 3, 4, 5];
        let doubled = collections::map_collection(&numbers, |&x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_filter_collection() {
        let numbers = vec![1, 2, 3, 4, 5];
        let evens = collections::filter_collection(&numbers, |&x| x % 2 == 0);
        assert_eq!(evens, vec![&2, &4]);
    }

    #[test]
    fn test_reduce_collection() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = collections::reduce_collection(&numbers, 0, |acc, &x| acc + x);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_all_elements() {
        let numbers = vec![2, 4, 6, 8];
        let all_even = collections::all_elements(&numbers, |&x| x % 2 == 0);
        assert!(all_even);
    }

    #[test]
    fn test_any_element() {
        let numbers = vec![1, 3, 5, 6];
        let has_even = collections::any_element(&numbers, |&x| x % 2 == 0);
        assert!(has_even);
    }

    #[test]
    fn test_map_option() {
        let some_value = Some(5);
        let doubled = option_utils::map_option(some_value, |x| x * 2);
        assert_eq!(doubled, Some(10));
    }

    #[test]
    fn test_map_string() {
        let result = string_utils::map_string("hello", |s| s.to_uppercase());
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_filter_string() {
        let result = string_utils::filter_string("hello123", |c| c.is_alphabetic());
        assert_eq!(result, "hello");
    }
}
