use rand::Rng;

pub fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let n = shipments.len() as u32;
    let total: u32 = shipments.iter().sum();
    if total % n != 0 || shipments.len() < 5 {
        return None;
    }
    let avg = (total / n) as i32;
    let moves: usize = shipments
        .iter()
        .filter_map(|&w| {
            let diff = w as i32 - avg;
            if diff > 0 {
                Some(diff as usize)
            } else {
                None
            }
        })
        .sum();
    Some(moves)
}

pub fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut v: Vec<u32> = (0..n).map(|_| rng.gen_range(1..100)).collect();
    let total: u32 = v.iter().sum();
    let rem = total % n as u32;
    if rem != 0 {
        v[0] += n as u32 - rem;
    }
    v
}

#[cfg(test)]
mod tests {   
    use super::*;
   
    #[test]
    fn test_count_permutation_examples() {
        let ex1 = vec![8, 2, 2, 4, 4];
        assert_eq!(count_permutation(&ex1), Some(4));

        let ex2 = vec![9, 3, 7, 2, 9];
        assert_eq!(count_permutation(&ex2), Some(7));

        let imposs = vec![1, 2, 3];
        assert_eq!(count_permutation(&imposs), None);
    }
 

    #[test]
    fn test_gen_shipments() {
        for &n in &[5, 10, 20] {
            let v = gen_shipments(n);
            let sum: u32 = v.iter().sum();
            assert_eq!(sum % n as u32, 0);
            assert_eq!(v.len(), n);
        }
    }
}
