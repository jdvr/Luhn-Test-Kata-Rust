#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_past_with_the_sample_number() {
        let sample_number = vec![4, 9, 9, 2, 7, 3, 9, 8, 7, 1, 6];
        assert_eq!(true, luhn(sample_number));
    }
}

fn sum_digits(number: u32) -> u32 {
    let digits = vec![(number / 10), number % 10];
    digits.iter().fold(0, |sum, n| sum + n)
}

pub fn luhn(number_as_vec: Vec<u32>) -> bool {
    let s1 = number_as_vec.iter()
        .rev()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .fold(0, |sum, (_, n)| sum + n);

    let s2 = number_as_vec.iter()
        .rev()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 1)
        .map(|(_, n)| n * 2)
        .map(|n| sum_digits(n))
        .fold(0, |sum, n| sum + n);
    let sum = s1 + s2;
    sum % 10 == 0
}
