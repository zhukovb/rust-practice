pub fn draw_christmas_tree(triangles_count: usize) {
    // Перевірка на нульове значення
    if triangles_count == 0 {
        println!("Ялинка повинна мати хоча б один трикутник!");
        return;
    }

    // Максимальна ширина ялинки (осн.) для центрування всіх ярусів
    let max_width = 2 * triangles_count - 1;

    // Малюємо кожен трикутник
    for triangle_num in 1..=triangles_count {
        // Ширина основи поточного трикутника
        let base_width = 2 * triangle_num - 1;

        // Крок зростання зірок по 2 (1, 3, 5, …)
        for stars_count in (1..=base_width).step_by(2) {
            let padding = (max_width - stars_count) / 2;
            let row = " ".repeat(padding) + &"★".repeat(stars_count);
            println!("{}", row);
        }
    }

    // Додаємо стовбур (2 рядки по центру)
    let trunk_width = if triangles_count > 1 { 3 } else { 1 };
    let padding = (max_width - trunk_width) / 2;
    let trunk = " ".repeat(padding) + &"★".repeat(trunk_width);
    println!("{}", trunk);
    println!("{}", trunk);
}

fn main() {
    // Регулюйте тут кількість ярусів:
    let levels = 5;
    draw_christmas_tree(levels);
}
