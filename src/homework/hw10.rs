/// Повертає true, якщо десятковий запис x читається однаково зліва направо й справа наліво.
pub fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    let rev: String = s.chars().rev().collect();
    s == rev
}

#[cfg(test)]
mod tests {
    use super::is_palindrome;

    #[test]
    fn test() {
        let data = [
            (123, false),
            (121, true),
            (1221, true),
        ];

        for &(n, exp) in &data {
            assert_eq!(is_palindrome(n), exp);
        }
    }
}
