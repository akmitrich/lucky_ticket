use std::{
    collections::{hash_map::Entry, HashMap},
    ops::{AddAssign, Mul},
};

pub fn lucky(n: u32) -> u32 {
    let mut sums: HashMap<u32, u32> = HashMap::new();
    let max_number = 10_u32.pow(n);
    for x in 0..max_number {
        let sum = digits_sum(x);
        match sums.entry(sum) {
            Entry::Occupied(mut exist) => exist.get_mut().add_assign(1_u32),
            Entry::Vacant(vacant) => vacant.insert(0_u32).add_assign(1_u32),
        }
    }
    sums.values().map(|x| x.mul(x)).sum()
}

fn digits_sum(x: u32) -> u32 {
    if x == 0 {
        return 0;
    }
    let remain = x / 10;
    let last_digit = x - remain * 10;
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
}
