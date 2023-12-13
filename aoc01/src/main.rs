
fn main() {

    let input = include_str!("../input.txt");
    let result = sum_numbers(input);
    println!("Result: {}", result);
}

fn sum_numbers(input: &str) -> u32 {

    let mut sum = 0;
    for mut line in input.lines() {
        let mut sum_chars:[char;2] = ['#', '#'];
        let line_replaced = &replace_num_literals(line.to_string());
        //println!("Line previous: \"{}\" Line replaced: \"{}\"", line, line_replaced);

        line = line_replaced.as_str();
        let mut chars: Vec<char> = line.chars().collect();

        while chars.len() > 0 {
            left_operation(&mut sum_chars, &mut chars);
            right_operation(&mut sum_chars, &mut chars);

            if is_all_numbers_found(&mut sum_chars) { break; }
        }
        fix_edge_cases(&mut sum_chars, 0, 1);
        fix_edge_cases(&mut sum_chars, 1, 0);

        let result = char_array_to_int(sum_chars);
        println!("Line {} replaced {} =>  Result: {}", line, line_replaced, result);
        sum += result;
    }
    return sum;
}

fn is_all_numbers_found(sum_chars: &mut [char; 2]) -> bool {
    return sum_chars[0] != '#' && sum_chars[1] != '#'
}

fn right_operation(sum_chars: &mut [char; 2], chars: &mut Vec<char>) {
    if sum_chars[1] == '#' && chars.len() > 0 {
        if let Some(_num) = chars.last().unwrap().to_digit(10) {
            sum_chars[1] = *chars.last().unwrap();
            //println!("Found number and setting it sum_chars[1]: {:#?}", sum_chars[1]);
        }
        chars.pop();
    }
}

fn left_operation(sum_chars: &mut [char; 2], chars: &mut Vec<char>) {
    if sum_chars[0] == '#' {
        if let Some(_num) = chars.first().unwrap().to_digit(10) {
            sum_chars[0] = *chars.first().unwrap();
            //println!("Found number and setting it as sum_chars[0]: {:#?}", sum_chars[0]);
        }

        chars.remove(0);
    }
}

fn char_array_to_int(sum_chars: [char; 2]) -> u32 {
    let result: u32 = String::from_iter(sum_chars).parse::<u32>().unwrap_or(0);
    result
}

fn fix_edge_cases(sum_chars: &mut [char; 2], index_left : usize, index_right: usize) {
    if sum_chars[index_left] == '#' {
        sum_chars[index_left] = sum_chars[index_right];
    }
}

fn replace_num_literals(mut line: String) -> String {
    let numbers = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let chars: Vec<char> = line.chars().collect();

    let index_start = 0;
    let index_end = chars.len();

    for (index_char, _c) in chars.iter().enumerate() {
        let substring_left = String::from_iter(chars[index_start..index_char].iter());
        let substring_right = String::from_iter(chars[index_end-index_char..index_end].iter());
        for (index_num, number) in numbers.iter().enumerate() {
            if substring_left.contains(number){
                line = line.replace(number, &index_num.to_string());
            }

            if substring_right.contains(number){
                line = line.replace(number, &index_num.to_string());
            }
        }
    }

    return line
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
        assert_eq!(result, 69);
    }

    #[test]
    fn test_plxfoursc41five() {
        let input = "plxfoursc41five";

        let result = sum_numbers(input);
        assert_eq!(result, 45);
    }

    #[test]
    fn test_multiple() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        let result = sum_numbers(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_multiple_literal(){
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

        let result = sum_numbers(input);
        assert_eq!(result, 281);
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
        assert_eq!(result, 53);
    }

    #[test]
    fn test_literals_two1nine(){
        let input = "two1nine";

        let result = sum_numbers(input);
        assert_eq!(result, 29);
    }

    #[test]
    fn test_literals_4nineeightseven2(){
        let input = "4nineeightseven2";

        let result = sum_numbers(input);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_literals_eightwothree(){
        let input = "eightwothree";

        let result = sum_numbers(input);
        assert_eq!(result, 83);
    }

    #[test]
    fn test_literals_7pqrstsixteen(){
        let input = "7pqrstsixteen";

        let result = sum_numbers(input);
        assert_eq!(result, 76);
    }

    #[test]
    fn test_line_replacement(){
        let input = "eightwothree";
        let result = super::replace_num_literals(input.to_string());

        assert_eq!(result, "8wo3");
    }

    #[test]
    fn test_literal_s8twoned(){
        let input = "s8twoned";

        let result = sum_numbers(input);
        assert_eq!(result, 81);
    }

    #[test]
    fn test_literal_eightsixhnsbnine1twonevrs(){
        let input = "eightsixhnsbnine1twonevrs";

        let result = sum_numbers(input);
        assert_eq!(result, 81);
    }


    #[test]
    fn test_literal_six3niners7eightwokjj(){
        let input = "six3niners7eightwokjj";

        let result = sum_numbers(input);
        assert_eq!(result, 62);
    }

    #[test]
    fn test_literal_1sevenrjstpdxfiveseven(){
        let input = "1sevenrjstpdxfiveseven";

        let result = sum_numbers(input);
        assert_eq!(result, 17);
    }

    #[test]
    fn test_literal_3zmbbqdqqnineeightlvlfqqxleighttloneightv(){
        let input = "3zmbbqdqqnineeightlvlfqqxleighttloneightv ";

        let result = sum_numbers(input);
        assert_eq!(result, 38);
    }

    #[test]
    fn test_literal_7sixsevensix(){
        let input = "7sixsevensix";

        let result = sum_numbers(input);
        assert_eq!(result, 76);
    }

    #[test]
    fn test_litearl_oneight(){
        let input = "oneight";

        let result = sum_numbers(input);
        assert_eq!(result, 11);
    }
}