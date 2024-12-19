use std::collections::HashMap;
use std::fs;

const INDEX_FILE_PATH: &str = "../input.txt";

fn main() {
    let file = fs::read_to_string(INDEX_FILE_PATH).unwrap();
    let mut frequency_counts: HashMap<i32, [i32; 2]> = HashMap::new();

    let mut _iter = file.split_whitespace();
    let mut counts_index = 0;

    // count occurences in a hashmap
    _iter.for_each(|item: &str| {
        let num: i32 = match item.trim().parse() {
            Ok(n) => n,
            Err(_) => panic!("{item} cannot be converted into a number!"),
        };

        // insert new counts if needed
        frequency_counts.entry(num).or_insert([0; 2]);

        // modify current count at counts_index
        frequency_counts
            .entry(num)
            .and_modify(|counts| counts[counts_index] += 1);

        counts_index = counts_index ^ 1;
    });

    // sum occurrence multiplication
    let mut sum = 0;
    for (n, counts) in frequency_counts {
        sum += n * counts[0] * counts[1];
    }

    println!("{}", sum);
}
