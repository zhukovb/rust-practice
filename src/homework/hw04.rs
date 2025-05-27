// src/homework/hw04.rs
fn main() {
    const H: usize = 6; // висота верхньої половини ромба (разом із серединним рядком)

    let mut output = String::new();

    // Верхня половина (включно з серединним рядком)
    for i in 0..H {
        // спочатку пропуски
        output.push_str(&" ".repeat(H - i - 1));
        // потім зірочки
        output.push_str(&"*".repeat(2 * i + 1));
        output.push('\n');
    } 

    // Нижня половина
    for i in (0..H-1).rev() {
        output.push_str(&" ".repeat(H - i - 1));
        output.push_str(&"*".repeat(2 * i + 1));
        output.push('\n');
    }

    // Друкуємо результат одним print! і додаємо ще один println! для безпечного завершення рядка
    print!("{}", output);
    println!();
}
