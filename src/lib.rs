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
    factors.sort();
    factors
}

pub fn multiply_vector(values: &[usize]) -> usize {
    values.iter()
        .fold(1, |a, b| a * b)
}

pub fn find_factors_sieve(num: usize, primes: &[usize]) -> Vec<usize> {
    let mut n: usize = num;
    let mut factors: Vec<usize> = vec![];
    for prime in primes {
        if *prime > n {
            break;
        }
        while n % *prime == 0 {
            factors.push(*prime);
            n /= *prime;
        }
    }
    if n != 1 { factors.push(n); } // Not guaranteed to be prime!
    factors.sort();
    factors
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
    fn factors_312680865509917() {
        let result = find_factors(312680865509917);
        //assert_eq!(result, vec![40129483, 45339457]);
        assert_eq!(result, vec![7791799, 40129483]);
    }

    #[test]
    fn factors_12345678901234() {
        let result = find_factors(12345678901234);
        assert_eq!(result, vec![2, 7, 73, 12079920647]);
    }

    #[test]
    fn factors_64374108854777() {
        let result = find_factors(64374108854777);
        assert_eq!(result, vec![64374108854777]);
    }

    #[test]
    fn factors_6795742697625173() {
        let result = find_factors(6795742697625173);
        assert_eq!(result, vec![6880691, 987654103]);
    }

    #[test]
    fn factors_44711100255155897() {
        let result = find_factors(44711100255155897);
        assert_eq!(result, vec![45269999, 987654103]);
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

    #[test]
    fn test_find_factors_sieve() {
        let primes: Vec<usize> = vec![
            2usize, 3usize, 5usize, 7usize, 11usize, 13usize, 17usize, 19usize, 23usize, 29usize, 31usize,
            37usize, 41usize, 43usize, 47usize, 53usize, 59usize, 61usize, 67usize, 71usize, 73usize,
            79usize, 83usize, 89usize, 97usize,
        ];
        for i in 2..100 {
            let p1 = find_factors(i);
            let p2 = find_factors_sieve(i, &primes);
            assert_eq!(p1, p2);
        }
    }
}
