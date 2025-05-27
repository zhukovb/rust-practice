fn main() {
    let mut count = 0;

    for m in 1..=8 {
        for u in 1..=8 {
            if u == m { continue; }
            for x in 1..=8 {
                if x == m || x == u { continue; }
                for a in 1..=8 {
                    if [m, u, x].contains(&a) { continue; }

                    // MUXA
                    let mu_xa = m * 1000 + u * 100 + x * 10 + a;
                    // множимо на одноцифровий множник A
                    let prod = mu_xa * a;
                    // маємо отримати чотирицифрове SLON  
                    if prod < 1000 || prod > 9999 {
                        continue;
                    }

                    let s = prod / 1000;
                    let l = (prod / 100) % 10;
                    let o = (prod / 10) % 10;
                    let n = prod % 10;

                    // усі літери повинні бути різними та з діапазону 1..=8
                    let used = [m, u, x, a];
                    if [s, l, o, n].iter().any(|&d| used.contains(&d)) {
                        continue;
                    }
                    if s == l || s == o || s == n || l == o || l == n || o == n {
                        continue;
                    }

                    // якщо дійсно знайшли розв’язок — друкуємо в потрібному форматі
                    println!("  {}{}{}{}", m, u, x, a);
                    println!("x     {}", a);
                    println!("------+");
                    println!("  {}{}{}{}", s, l, o, n);
                    println!();
                    count += 1;
                }
            }
        }
    }

    println!("Кількість рішень: {}", count);
}
