/// Перевіряє, чи є `n` простим числом.
/// Повертає `false` для n < 2.
pub fn is_prime(n: &u32) -> bool {
    let n = *n;
    if n < 2 {
        return false;
    }
    // Перевіряємо дільники від 2 до sqrt(n)
    let limit = (n as f64).sqrt() as u32;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::is_prime;

    #[test]
    fn test_is_prime() {
        let test_data = [
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (100, false),
            (10007, true),
        ];

        for &(n, prime) in &test_data {
            assert_eq!(is_prime(&n), prime);
        }
    }
}
//git add src/homework/hw08.rs git commit -m "Додав hw08: реалізація is_prime" git push origin master
 