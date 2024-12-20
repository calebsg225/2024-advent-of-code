use std::fs;

/*

- input contains lines of text
- each line (report) contains a number of space-separated intergers representing levels

- for a report to be considered 'safe':
    1) the levels must all either increase or decrease
    2) the distance between 2 consecutive levels must be between 1 and 3 (inclusive)

Given these requirements, calculate the number of 'safe' reports in the input

*/

const INPUT_FILE_PATH: &str = "../input.txt";

fn main() {
    let _file = fs::read_to_string(INPUT_FILE_PATH).unwrap();
    let mut safe_report_count = 0;
    for report in _file.lines() {
        if is_report_safe(report) {
            safe_report_count += 1
        }
    }
    println!("{}", safe_report_count);
}

// returns true if level is considered 'safe'
fn is_report_safe(report: &str) -> bool {
    // attempt to convert str to number
    fn convert_to_i32(string: &str) -> i32 {
        match string.trim().parse() {
            Ok(n) => n,
            Err(_) => panic!("{string} cannot be converted into an integer!"),
        }
    }

    fn passes_level_safety_conditions(t: i32, prev: i32, cur: i32) -> bool {
        if prev == cur {
            return false;
        } // prev level is identical to cur level
        if t == 1 && (prev > cur || prev + 3 < cur) {
            return false;
        } // prev level is decreasing or is too far from cur level
        if t == -1 && (prev < cur || prev - 3 > cur) {
            return false;
        } // prev level is increasing or is too far from cur level
        true
    }

    let mut tendency = 0; // positive for incrementing, vice versa
    let mut prev_level: i32 = 0; // keep track of the previous value to track tendency changes or tolerance deviations

    // iterate through each level of the result
    for (i, level_str) in report.split_whitespace().enumerate() {
        let cur_level = convert_to_i32(level_str);
        if i == 1 {
            // tendency not set
            // set tendency
            if cur_level - prev_level < 0 {
                tendency = -1
            } else {
                tendency = 1
            } // edge case is when they are equal. This will be caught later, no need to duplicate
        }
        // for all levels excluding the first, test each level pair
        if i != 0 && !passes_level_safety_conditions(tendency, prev_level, cur_level) {
            return false;
        };
        prev_level = cur_level;
    }
    true
}
