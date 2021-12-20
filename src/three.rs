use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
struct Binary(Vec<u32>);

impl Binary {
    fn to_integer(&self) -> u32 {
        let mut result = 0;
        for d in &self.0 {
            result = result * 2 + d;
        }
        return result;
    }
}
impl Add for Binary {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self ( self.0.into_iter().zip(other.0.into_iter()).map(|(x, y)| x+y).collect())
    }
}
impl Add for &Binary {
    type Output = Binary;

    fn add(self, other: Self) -> Binary {
        Binary ( self.0.iter().zip(other.0.iter()).map(|(x, y)| x+y).collect())
    }
}

#[test]
fn part1() {
    let file = File::open("3.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();
    let binary_digits: Vec<Binary> = lines
        .map(|l| Binary(l.unwrap().chars().map(|c| c.to_digit(2).unwrap()).collect()))
        .collect();
    let num_words = binary_digits.len().try_into().unwrap();
    let mut sum = binary_digits[0].clone();
    for word in binary_digits.into_iter().skip(1) {
        sum = sum + word;
    }
    println!("{:?}", sum);
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut place_value = 1;
    for i in sum.0.into_iter().rev() {
        if i*2 > num_words  {
            gamma += place_value;
        } else {
            epsilon += place_value;
        }
        place_value*=2;
    }
    println!("Overall product {} {}", gamma*epsilon, num_words);
}


fn find_code(mut digits: Vec<Binary> , co2:bool) -> Binary {
    let word_length = digits[0].0.len();
    for i in 0..word_length {
        let num_words = digits.len().try_into().unwrap();
        if num_words <= 1 {
            break;
        }
        let ones_count :u32 = digits.iter().map(|b| b.0[i]).sum();
        let bit_match = (ones_count*2 >= num_words) ^ co2;
        digits.retain(|b| (b.0[i] == 1) == bit_match);
    }
    return digits[0].clone();
}
#[test]
fn part2() {
    let file = File::open("3.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();
    let binary_digits: Vec<Binary> = lines
        .map(|l| Binary(l.unwrap().chars().map(|c| c.to_digit(2).unwrap()).collect()))
        .collect();
    let oxygen_bin = find_code(binary_digits.clone(), false);
    let co2_bin = find_code(binary_digits, true);
    
    let oxygen = oxygen_bin.to_integer();
    let co2 = co2_bin.to_integer();
    println!("oxygen {} co2 {} product {}", oxygen, co2, oxygen*co2);
   
}