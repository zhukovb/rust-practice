use std::collections::HashSet;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Rectangle {
    pub a: Point,
    pub b: Point,
}

pub fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut covered = HashSet::new();
    for rect in xs {
        let x1 = rect.a.x.min(rect.b.x);
        let x2 = rect.a.x.max(rect.b.x);
        let y1 = rect.a.y.min(rect.b.y);
        let y2 = rect.a.y.max(rect.b.y);
        for x in x1..x2 {
            for y in y1..y2 {
                covered.insert((x, y));
            }
        }
    }
    covered.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Rectangle> {
        vec![
            Rectangle {
                a: Point { x: 2, y: 9 },
                b: Point { x: 5, y: 3 },
            },
            Rectangle {
                a: Point { x: 1, y: 8 },
                b: Point { x: 11, y: 6 },
            },
            Rectangle {
                a: Point { x: 9, y: 10 },
                b: Point { x: 13, y: 2 },
            },
        ]
    }

    #[test]
    fn area_occupied_test() {
        let data = test_data();
        let occupied = area_occupied(&data);
        assert_eq!(occupied, 60);
    }
}   
//cd D:\Project\rust-practice-clean

