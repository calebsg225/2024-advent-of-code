use std::fs;
use std::vec::Vec;

const INPUT_TEXT_FILE_PATH: &str = "./input.txt";

fn main() {
    let file = fs::read_to_string(INPUT_TEXT_FILE_PATH).unwrap();
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    let mut even = false;
    let mut _iter = file.split_whitespace();

    // put input into two vectors
    _iter.for_each(|item: &str| {
        let num: i32 = match item.trim().parse() {
            Ok(n) => n,
            Err(_) => panic!("{item} cannot be converted to a number!"),
        };
        if even {
            left.push(num);
        } else {
            right.push(num);
        }
        even = !even;
    });

    // sort vectors
    // bubble sort for now since I am still completely new to this language and bubble sort is easy
    let v_size = left.len();
    for i in 0..v_size {
        let end_ind = v_size - i - 1;
        let mut l_max = 0;
        let mut r_max = 0;
        for j in 0..end_ind + 1 {
            if left[l_max] < left[j] {
                l_max = j
            }
            if right[r_max] < right[j] {
                r_max = j
            }
        }
        left.swap(l_max, end_ind);
        right.swap(r_max, end_ind);
    }

    // combine vectors into abs sum
    let mut sum = 0;
    for i in 0..v_size {
        sum += (left[i] - right[i]).abs();
    }

    println!("{}", sum);
}
