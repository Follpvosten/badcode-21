#![feature(proc_macro_hygiene)]
use badcodegen::generate_valid_inputs;

use std::collections::HashMap;

fn get_valid_inputs() -> HashMap<Vec<char>, &'static str> {
    generate_valid_inputs!()
}

pub fn find_missing(input: &[char]) -> &'static str {
    let valid_inputs = get_valid_inputs();
    valid_inputs[input]
}

#[cfg(test)]
mod tests {
    use crate::find_missing;
    #[test]
    fn abdefg_works() {
        assert_eq!(
            find_missing(&['a', 'b', 'd', 'e', 'f', 'g']),
            "c is missing"
        );
    }
    #[test]
    fn np_works() {
        assert_eq!(find_missing(&['n', 'p']), "o is missing");
    }
    #[test]
    fn ac_works() {
        assert_eq!(find_missing(&['a', 'c']), "b is missing");
    }
    #[test]
    #[should_panic]
    fn abc_panics() {
        find_missing(&['a', 'b', 'c']);
    }
    #[test]
    #[should_panic]
    fn cba_panics() {
        find_missing(&['c', 'b', 'a']);
    }
}
