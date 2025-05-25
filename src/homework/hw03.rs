fn main() {
    // Розміри конверта (можна міняти в межах 10..=80)
    const W: usize = 40;
    const H: usize = 20;

    // Збираємо весь малюнок у рядок
    let mut output = String::new();
    for i in 0..H {
        for j in 0..W {
            // межа прямокутника
            if i == 0 || i == H - 1 || j == 0 || j == W - 1 {
                output.push('*');
            }
            // діагоналі «конверта»
            else if j == i || j == W - 1 - i {
                output.push('*');
            }

            else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
    println!();
}
