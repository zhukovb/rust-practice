// Зсув рядка вправо на n позицій (негативні n — вліво)
pub fn rotate2(s: &str, n: isize) -> String {
    let len = s.chars().count() as isize;
    if len == 0 {
        return String::new();
    }
    // Нормалізуємо зсув у діапазоні [0..len)
    let shift = ((n % len) + len) % len;
    let split = (len - shift) as usize;
    let first: String = s.chars().skip(split).collect();
    let second: String = s.chars().take(split).collect();
    first + &second
}

#[cfg(test)]
mod tests {
    use super::rotate2;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        for &(n, exp) in &shifts {
            assert_eq!(rotate2(s, n), exp.to_string());
        }
    }
}//git add src/homewor k/hw09.rs git commit -m "Додав hw09: реалізація rotate2" git push origin master
