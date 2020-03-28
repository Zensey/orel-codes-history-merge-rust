#![allow(warnings)]
use std::io::{self, BufRead, Write};

fn process<W: Write, R: BufRead> (out: &mut W, mut rdr: R) {
    let (mut list_local, mut list_serv) = (read_vec(&mut rdr), read_vec(&mut rdr));
    let (res, diff) = merge(&mut list_serv, &mut list_local);
    write!(out, "{}\n", vec_to_string(&res));
    write!(out, "{}\n", vec_to_string(&diff));
}

fn vec_to_string(v: &Vec<i32>) -> String {
    v.into_iter()
        .map(|i| i.to_string() + " ")
        .collect::<String>()
        .trim()
        .to_string()
}

fn merge(list_serv: &mut Vec<i32>, list_local: &mut Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut merged_list: Vec<i32> = Vec::with_capacity((list_serv.len() + list_local.len())/2);
    let mut diff_list: Vec<i32> = Vec::new();
    let mut idx_local = 0;

    for elem_s in list_serv.iter() {
        while idx_local < list_local.len() && list_local[idx_local] < *elem_s {
            merged_list.push(list_local[idx_local]);
            idx_local += 1;
        }

        if idx_local < list_local.len() {
            if list_local[idx_local] == *elem_s {
                idx_local += 1;
            } else if list_local[idx_local] > *elem_s {
                diff_list.push(*elem_s)
            }
        } else {
            diff_list.push(*elem_s)
        }

        merged_list.push(*elem_s);
    }

    for elem_l in list_local[idx_local..].iter() {
        merged_list.push(*elem_l);
    }
    (merged_list, diff_list)
}

fn read_vec<R: BufRead>(mut cin: R) -> Vec<i32> {
    let mut s = String::new();
    s.clear();
    cin.read_line(&mut s).unwrap();
    let list_sz = s.trim().parse::<i32>().unwrap();

    s.clear();
    cin.read_line(&mut s).unwrap();
    let list = s
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();

    assert!(list.len() == list_sz as usize);
    list
}

fn main() {
    let mut out = io::stdout();
    let input = std::io::BufReader::new(io::stdin());
    process(&mut out, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Instant};
    use rand::{Rng, SeedableRng, StdRng};

    #[test]
    fn test_merge() {
        use std::io::Cursor;
        use std::io::prelude::*;

        let mut out = Cursor::new(Vec::new());
        let mut input = Cursor::new(Vec::new());
        input.write(b"7 \n");
        input.write(b"1 2 3 4 5 8 9 \n");
        input.write(b"6 \n");
        input.write(b"5 6 7 8 9 10 \n");
        input.seek(std::io::SeekFrom::Start(0));

        process(&mut out, input);

        let mut l = String::new();
        out.seek(std::io::SeekFrom::Start(0));
        out.read_line(&mut l).unwrap();
        assert_eq!(l.trim(), "1 2 3 4 5 6 7 8 9 10");
        l.clear();
        out.read_line(&mut l).unwrap();
        assert_eq!(l.trim(), "6 7 10");
    }

    fn gen_rand_seq<R: Rng> (n: i32, rng: &mut R) -> Vec<i32> {
        let mut m: Vec<i32> = Vec::with_capacity(n as usize);
        let mut i: i32 = 0;
        let mut skip_after: i32 = 0;

        while m.len() < n as usize {
            if skip_after == 0 {
                skip_after = rng.gen_range(1, 20);
            } else {
                m.push(i);
                skip_after -= 1;
            }
            i+= 1;
        }
        m
    }

    fn gen_seq (n: i32) -> Vec<i32> {
        let mut m: Vec<i32> = Vec::with_capacity(n as usize);
        let mut i: i32 = 0;
        while m.len() < n as usize {
            i+= 1;
            m.push(i);
        }
        m
    }

    #[test]
    fn bech_merge() {
        let mut rng = rand::thread_rng();
        let history_length = 100_000_000;
        let bench_iters = 21;

        let mut array: Vec<std::time::Duration> = Vec::new();
        for i in 0..bench_iters {
            println!("bench iteration {}", i);
            let mut list_local = gen_rand_seq(history_length, &mut rng);
            let mut list_serv = gen_rand_seq(history_length, &mut rng);
            assert_eq!(list_local.len(), history_length as usize);
            assert_eq!(list_serv.len(), history_length as usize);

            let now = Instant::now();
            let (_result, _diff) = merge(&mut list_serv, &mut list_local);
            array.push(now.elapsed());
        }
        array.sort();
        println!("test tests::bech_merge: median time-> {:?}", array[bench_iters/2]);
    }
}
