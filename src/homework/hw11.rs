use rand::Rng;

/// 1) Генерує рандомний вектор довжиною n зі значеннями [10..99]
pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

/// 2) Знаходить індекс i та суму i + (i+1) так, щоб сума була мінімальною.
/// Повертає None, якщо вектор менший за 2.
pub fn min_adjacent_sum(data: &[i32]) -> Option<(usize, i32)> {
    if data.len() < 2 {
        return None;
    }
    let mut min_idx = 0;
    let mut min_sum = data[0] + data[1];
    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_idx = i;
        }
    }
    Some((min_idx, min_sum))
}

/// 3) Друкує в консоль у форматі з вашого прикладу.
pub fn display_min_adjacent_pair(data: &[i32]) {
    // Лінія з індексами
    print!("indexes:");
    for i in 0..data.len() {
        print!(" {:>2}.", i);
    }
    println!();

    // Лінія з даними
    print!("data:    [");
    for (i, v) in data.iter().enumerate() {
        print!("{}", v);
        if i < data.len() - 1 {
            print!(", ");
        }
    }
    println!("]");

    // Лінія з вказівником
    if let Some((idx, _sum)) = min_adjacent_sum(data) {
        // порахуємо, скільки пробілів до позиції idx
        let prefix_len = "indexes:".len() + 1 + idx * 4;
        let mut arrow = String::new();
        for _ in 0..prefix_len {
            arrow.push(' ');
        }
        arrow.push_str("\\__ __/");
        println!("{}", arrow);

        // Підсумкова лінія
        println!(
            "min adjacent sum={}+{}={} at indexes:{},{}",
            data[idx],
            data[idx + 1],
            _sum,
            idx,
            idx + 1
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_outputs() {
        let examples = [
            [45, 87, 49, 64, 50, 37, 45, 72, 55, 64, 90, 86, 60, 54, 78, 72, 83, 44, 89, 22],
            [29, 92, 14, 65, 57, 98, 10, 45, 11, 48, 69, 21, 12, 75, 51, 69, 72, 36, 47, 45],
            [19, 86, 66, 95, 40, 24, 90, 74, 98, 37, 26, 44, 76, 86, 48, 63, 11, 38, 29, 40],
            [30, 18, 68, 87, 99, 81, 88, 45, 34, 79, 81, 79, 93, 55, 26, 24, 32, 55, 59, 97],
        ];

        for data in &examples {
            // Переконаємося, що викликається без панік і повертає Some
            assert!(min_adjacent_sum(data.as_ref()).is_some());
        } 
    }
}
