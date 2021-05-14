fn main() {
    let mut numbers = [2, 3, 8, 1, 9];
    println!("{:?}", numbers);

    let mut i = 0;
    numbers[2] = 10;
    while i < numbers.len() {
        println!("numbers[{}] == {}", i, numbers[i]);
        i += 1;
    }

    let mut numbers_vec: Vec<i32> = vec![2, 3, 8, 1, 9];
    numbers_vec.push(10);
    println!("{:?}", numbers_vec);

    let numbers_vec_ref = &vec![0, 1, 2, 3];
    println!("{:?}", &numbers_vec_ref[0..2]);
    let a = &numbers_vec_ref[..];
}

fn sum_rec(arr: &[i32]) -> i32 {
    if arr.len() == 0 {
        return 0;
    } else {
        return arr[0] + sum_rec(&arr[1..]);
    }
}

fn sum_tail_rec(arr: &[i32], acc: i32) -> i32 {
    if arr.is_empty() {
        acc
    } else {
        sum_tail_rec(&arr[1..], acc + arr[0])
    }
}

fn sum_tail_rec_default(arr: &[i32]) -> i32 {
    sum_tail_rec(&arr, 0)
}

fn sum(arr: &[i32]) -> i32 {
    let mut total = 0;
    for i in 0..arr.len() {
        total += arr[i];
    }
    total
}

fn maximum(arr: &[u32]) -> u32 {
    let mut max = 0;
    for i in 0..arr.len() {
        if arr[i] > max {
            max = arr[i];
        }
    }
    max
}

fn reverse(arr: &[i32]) -> Vec<i32> {
    let mut res = vec![];
    let mut i = arr.len();
    while i > 0 {
        res.push(arr[i - 1]);
        i -= 1;
    }
    res
}

fn fibs(count: usize) -> Vec<u32> {
    if count < 2 {
        vec![1]
    } else {
        let mut res = vec![1, 1];
        for i in 2..count {
            res.push(res[i - 1] + res[i - 2]);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given_array_of_numbers_then_can_calculate_sum() {
        let numbers = [2, 3, 8, 1, 9];
        let first_and_snd = &numbers[0..2];
        assert_eq!(23, sum(&numbers));
        assert_eq!(23, sum_rec(&numbers));
        assert_eq!(23, sum_tail_rec_default(&numbers));
        assert_eq!(5, sum_tail_rec_default(&first_and_snd));
        assert_eq!(2, first_and_snd.len());
    }

    #[test]
    fn exercise1_maximum() {
        assert_eq!(maximum(&[1, 2, 3]), 3);
        assert_eq!(maximum(&[4, 5, 2, 8]), 8);
        assert_eq!(maximum(&[]), 0);
    }

    #[test]
    fn exercise2_reverse() {
        assert_eq!(reverse(&[1, 2, 3]), &[3, 2, 1]);
        assert_eq!(reverse(&[]), &[]);
    }

    #[test]
    fn exercise3_fibonacci() {
        assert_eq!(fibs(5), &[1, 1, 2, 3, 5]);
        assert_eq!(fibs(1), &[1]);
        assert_eq!(fibs(8), &[1, 1, 2, 3, 5, 8, 13, 21]);
    }
}
