pub fn find_factors(num: usize) -> Vec<usize> {
    let mut factors = vec![];
    let mut n = num;

    for j in [2, 3] {
        while n % j == 0 {
            factors.push(j);
            n /= j;
        }
    }

    let mut upper = ((n as f64).sqrt() as usize) + 1;
    for i in [5, 7] {
        for j in (i..(upper + 1)).step_by(6) {
            // println!("{j} {upper} {n}");
            while j < upper && (n % j) == 0 {
                factors.push(j);
                n /= j;  
                upper = ((n as f64).sqrt() as usize) + 1;
            }
        }
    }
    if n != 1 {factors.push(n);}
    factors
}

pub fn multiply_vector(values: &[usize]) -> usize {
    values.iter()
        .fold(1, |a, b| a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factors_2() {
        let result = find_factors(2);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn factors_3() {
        let result = find_factors(3);
        assert_eq!(result, vec![3]);
    }

    #[test]
    fn factors_4() {
        let result = find_factors(4);
        assert_eq!(result, vec![2, 2]);
    }

    #[test]
    fn factors_5() {
        let result = find_factors(5);
        assert_eq!(result, vec![5]);
    }

    #[test]
    fn factors_6() {
        let result = find_factors(6);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn factors_10() {
        let result = find_factors(10);
        assert_eq!(result, vec![2, 5]);
    }

    #[test]
    fn factors_160() {
        let result = find_factors(160);
        assert_eq!(result, vec![2, 2, 2, 2, 2, 5]);
    }

    #[test]
    fn factors_60() {
        let result = find_factors(60);
        assert_eq!(result, vec![2, 2, 3, 5]);
    }

    #[test]
    fn factors_2_3_5() {
        let result = multiply_vector(&[2usize, 3usize, 5usize]);
        assert_eq!(result, 30);
    }

    #[test]
    fn factors_general() {
        for i in 1..1000 {
            let factors = find_factors(i);
            let product = multiply_vector(&factors);
            assert_eq!(i, product);
        }
    }
}
