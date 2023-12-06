
fn main() {
    let input = include_str!("../input.txt");
    let result = sum_numbers(input);
    println!("Result: {}", result);

}

fn sum_numbers(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut sum_chars:[char;2] = ['#', '#'];
        let mut chars: Vec<char> = line.chars().collect();
        while chars.len() > 0 {
            if sum_chars[0] == '#' {
                if let Some(_num) = chars.first().unwrap().to_digit(10) {
                    sum_chars[0] = *chars.first().unwrap();
                    println!("Found number and setting it as sum_chars[0]: {:#?}", sum_chars[0]);
                }
                chars.remove(0);
            }

            if sum_chars[1] == '#' && chars.len() > 0 {
                if let Some(_num) = chars.last().unwrap().to_digit(10) {
                    sum_chars[1] = *chars.last().unwrap();
                    println!("Found number and setting it sum_chars[1]: {:#?}", sum_chars[1]);
                }
                chars.pop();
            }

            if sum_chars[0] != '#' && sum_chars[1] != '#' {
                break;
            }
        }
        if sum_chars[1] == '#' {
            sum_chars[1] = sum_chars[0];
        }

        if sum_chars[0] == '#' {
            sum_chars[0] = sum_chars[1];
        }
        let result:u32 = String::from_iter(sum_chars).parse::<u32>().unwrap_or(0);
        println!("Line {} =>  Result: {}", line, result);
        sum += result;
    }
    return sum;
}

#[cfg(test)]
mod test_aoc01
{
    use crate::sum_numbers;

    #[test]
    fn test_1abc2() {
        let input = "1abc2";
        let result = sum_numbers(input);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_pqr3stu8vwx() {
        let input = "pqr3stu8vwx";
        let result = sum_numbers(input);
        assert_eq!(result, 38);
    }

    #[test]
    fn test_a1b2c3d4e5f() {
        let input = "a1b2c3d4e5f";
        let result = sum_numbers(input);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_65nineszqvcgghgg(){
        let input = "65nineszqvcgghgg";

        let result = sum_numbers(input);
        assert_eq!(result, 65);
    }

    #[test]
    fn test_plxfoursc41five() {
        let input = "plxfoursc41five";

        let result = sum_numbers(input);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_multiple() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        let result = sum_numbers(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_edge_treb7uchet() {
        let input = "treb7uchet";

        let result = sum_numbers(input);
        assert_eq!(result, 77);
    }


    #[test]
    fn test_edge_mvmcg5three(){
        let input = "mvmcg5three";

        let result = sum_numbers(input);
        assert_eq!(result, 55);
    }
}