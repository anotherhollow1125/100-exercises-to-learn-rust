// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.
use std::thread;

pub fn _sum_ng(mut v: Vec<i32>) -> i32 {
    // let v2 = v.split_off(v.len() / 2);
    // Clone トレイトを要求しないバージョン↓
    let v2 = v.drain((v.len() / 2)..).collect();

    vec![v, v2]
        .into_iter()
        .map(|v| thread::spawn(move || v.iter().sum::<i32>()))
        .map(|handle| handle.join().unwrap())
        .sum()
}

pub fn _sum_ng_jikken(mut v: Vec<i32>) -> i32 {
    // let v2 = v.split_off(v.len() / 2);
    // Clone トレイトを要求しないバージョン↓
    let v2 = v.drain((v.len() / 2)..).collect();

    vec![v, v2]
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            thread::spawn(move || {
                let mut s = 0;

                for (j, n) in v.into_iter().enumerate() {
                    s += n;

                    if j % 100 == 0 {
                        eprintln!("thread {} : {} (sum: {})", i + 1, n, s);
                    }
                }

                s
            })
        })
        .map(|handle| handle.join().unwrap())
        .sum()
}

pub fn _sum(mut v: Vec<i32>) -> i32 {
    // let v2 = v.split_off(v.len() / 2);
    // Clone トレイトを要求しないバージョン↓
    let v2 = v.drain((v.len() / 2)..).collect();

    vec![v, v2]
        .into_iter()
        .map(|v| thread::spawn(move || v.iter().sum::<i32>()))
        .collect::<Vec<_>>()
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum()
}

pub fn sum(mut v: Vec<i32>) -> i32 {
    // let v2 = v.split_off(v.len() / 2);
    // Clone トレイトを要求しないバージョン↓
    let v2 = v.drain((v.len() / 2)..).collect();

    vec![v, v2]
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            thread::spawn(move || {
                let mut s = 0;

                for (j, n) in v.into_iter().enumerate() {
                    s += n;

                    if j % 100 == 0 {
                        eprintln!("thread {} : {} (sum: {})", i + 1, n, s);
                    }
                }

                s
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }

    #[test]
    fn too_big_sum() {
        let v = (0..10000).collect::<Vec<i32>>();
        let vv = v.clone();

        assert_eq!(sum(v), vv.iter().sum())
    }
}
