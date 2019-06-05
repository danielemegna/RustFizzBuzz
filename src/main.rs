use im_rc::ordmap;

fn fizz_buzz(i: i32) -> String {
    let dictionary = ordmap!{
        3 => "Fizz",
        5 => "Buzz",
        7 => "Pop"
    };

    let mut result = "".to_string();

    for n in dictionary.keys() {
        if i%n != 0 { continue; }
        let w = dictionary.get(&n).unwrap();
        result = result + &w.to_string();
    }

    if result.is_empty() {
        return i.to_string();
    }

    return result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_number() {
        assert_eq!(fizz_buzz(1), "1");
        assert_eq!(fizz_buzz(2), "2");
    }

    #[test]
    fn fizz_when_number_divisible_by_three() {
        assert_eq!(fizz_buzz(3), "Fizz");
        assert_eq!(fizz_buzz(6), "Fizz");
        assert_eq!(fizz_buzz(192), "Fizz");
    }

    #[test]
    fn buzz_when_number_divisible_by_five() {
        assert_eq!(fizz_buzz(5), "Buzz");
        assert_eq!(fizz_buzz(10), "Buzz");
        assert_eq!(fizz_buzz(470), "Buzz");
    }

    #[test]
    fn fizzbuzz_when_number_divisible_by_three_and_by_five() {
        assert_eq!(fizz_buzz(15), "FizzBuzz");
        assert_eq!(fizz_buzz(30), "FizzBuzz");
        assert_eq!(fizz_buzz(195), "FizzBuzz");
    }

    #[test]
    fn pop_when_number_divisible_by_seven() {
        assert_eq!(fizz_buzz(7), "Pop");
        assert_eq!(fizz_buzz(28), "Pop");
        assert_eq!(fizz_buzz(77), "Pop");
    }

    #[test]
    fn fizz_buzz_pop() {
        assert_eq!(fizz_buzz(105), "FizzBuzzPop");
        assert_eq!(fizz_buzz(210), "FizzBuzzPop");
        assert_eq!(fizz_buzz(945), "FizzBuzzPop");
    }
}
