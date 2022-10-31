use std::{
    collections::{hash_map::Entry, HashMap},
    ops::{AddAssign, Mul},
};

pub fn lucky_with_base(n: u32, base: u32) -> u32 {
    let sums = calc_sums(n, base);
    sums.values().map(|x| x.mul(x)).sum()
}

pub fn lucky(n: u32) -> u32 {
    lucky_with_base(n, 10)
}

fn digits_sum(x: u32) -> u32 {
    if x < 10 {
        return x;
    }
    let remain = x / 10;
    let last_digit = x - remain * 10;
    last_digit + digits_sum(remain)
}

fn calc_sums(n: u32, base: u32) -> HashMap<u32, u32> {
    let mut sums: HashMap<u32, u32> = HashMap::new();
    let max_number = base.pow(n);
    for x in 0..max_number {
        let sum = digits_sum_with_base(x, base);
        match sums.entry(sum) {
            Entry::Occupied(mut exist) => exist.get_mut().add_assign(1_u32),
            Entry::Vacant(vacant) => vacant.insert(0_u32).add_assign(1_u32),
        }
    }
    sums
}

fn digits_sum_with_base(x: u32, base: u32) -> u32 {
    if x < base {
        return x;
    }
    let remain = x / base;
    let last_digit = x - remain * base;
    last_digit + digits_sum(remain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic() {
        let result = dbg!(lucky(3));
        assert_eq!(55_252, result);
    }

    #[test]
    fn test_4() {
        let result = dbg!(lucky(4));
        assert_eq!(4_816_030, result);
    }

    #[test]
    fn test_digits_sum() {
        assert_eq!(0, digits_sum(0));
        assert_eq!(7, digits_sum(7));
        assert_eq!(6, digits_sum(42));
        assert_eq!(1, digits_sum(100));
        assert_eq!(28, digits_sum(7777));
    }

    #[test]
    fn test_lucky_with_base() {
        assert_eq!(55_252, lucky_with_base(3, 10));
        assert_eq!(4_816_030, lucky_with_base(4, 10));
    }
}
