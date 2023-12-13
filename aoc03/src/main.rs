
fn main() {
    println!("Hello, world!");

    let input = include_str!("../input.txt");
    let array= create_array(input);



}

fn create_array(input: &str) -> Vec<Vec<char>> {
    let mut array_dim1:Vec<Vec<char>> = Vec::with_capacity(input.lines().count());

    for (x, line) in input.lines().enumerate() {
        let inner_vec = Vec::with_capacity(line.chars().count());
        array_dim1.push(inner_vec);

        for (y, char) in line.chars().enumerate() {
            array_dim1[x].push(char);
        }
    }

    println!("{:?}", array_dim1);
    println!("{} ", array_dim1.len());

    return array_dim1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_array() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let array = create_array(input);
        assert_eq!(array.len(), 10);
        for i in 0..array.len() {
            assert_eq!(array[i].len(), 10);
        }
    }
}