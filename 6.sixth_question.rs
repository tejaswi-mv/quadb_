//Implement a function that finds the longest common prefix of a given set of strings.

use std::cmp::min;

fn longest_common_prefix(strs: &[String]) -> Option<String> {
    if strs.is_empty() {
        return None;
    }
    let n = strs.len();
    if n == 1 {
        return strs[0].clone();
    }

    strs.sort();

    let mini = min(strs[0].len(), strs[n - 1].len());

    let mut i = 0;
    while i < mini && strs[0].chars().nth(i) == strs[n - 1].chars().nth(i) {
        i += 1;
    }

    return Some(strs[0][..i].to_string());
}
