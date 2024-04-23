//! UUID utility functions

use uuid::Uuid;

/// Don't allow large UUID strings
const UUID_SIZE: usize = 36; // 32 digits, 4 hyphens

/// Returns the UUID if a given string is UUID format, or None if invalid
pub fn to_uuid(s: &str) -> Option<Uuid> {
    // Prevent buffer overflows
    if s.len() != UUID_SIZE {
        // rest_error!("(is_uuid) input string incorrect size: {} (expected {UUID_SIZE}).", s.len());
        return None;
    }

    // the underlying implementation of parse_str does a transmute
    //  <https://doc.rust-lang.org/std/mem/fn.transmute.html>
    //  to convert the string to a byte array, without first checking
    //  the size. We check the length of the string above first
    //  Potential security risk, so we check the length first
    Uuid::parse_str(s).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uuid() {
        let expected = Uuid::new_v4();
        let valid_str = expected.to_string();
        assert_eq!(to_uuid(&valid_str), Some(expected));

        // replace a hyphen with an alphanumeric
        let tmp = valid_str.replacen("-", "a", 1);
        assert_eq!(to_uuid(&tmp), None);

        // one digit too long
        let tmp = valid_str.clone() + "1";
        assert_eq!(to_uuid(&tmp), None);

        // one digit too short
        let tmp = valid_str.get(0..UUID_SIZE - 1).unwrap();
        assert_eq!(to_uuid(&tmp), None);

        // invalid character
        let tmp = valid_str.replacen(char::is_alphanumeric, "%", 1);
        assert_eq!(to_uuid(&tmp), None);
    }
}
