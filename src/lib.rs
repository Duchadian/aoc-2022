#![warn(clippy::all, clippy::pedantic)]
use std::fs;
use std::collections::HashSet;
use std::ops::{Index, Range};
use std::str::{FromStr, Split};
use itertools::Itertools;

pub fn advent_1() {
    let binding = fs::read_to_string("inputs/day_1").expect("Cannot read input file");
    let input = binding.split('\n');

    let mut highest : i32 = 0;
    let mut current : i32 = 0;
    for line in input {
        if line.is_empty(){
            if current > highest {
                highest = current ;
            }
            current = 0;
        }

        let calories : i32 = line.parse().unwrap_or(0);
        current += calories;
    }

    println!("{:?}", highest);
}

pub fn advent_2() {
    let binding = fs::read_to_string("inputs/day_1").expect("Cannot read input file");
    let input = binding.split('\n');

    let mut calorie_list : Vec<i32> = Vec::new();
    let mut current : i32 = 0;
    for line in input {
        if line.is_empty(){
            calorie_list.push(current);
            current = 0;
        }

        let calories : i32 = line.parse().unwrap_or(0);
        current += calories;
    }

    calorie_list.sort_unstable();
    let third_last = calorie_list.len().saturating_sub(3);

    println!("{:?}",calorie_list[third_last..].iter().sum::<i32>());
}

pub fn advent_3() {
    let binding = fs::read_to_string("inputs/day_2").expect("Cannot read input file");

    let input = binding.split('\n');

    #[derive(Debug, Clone)]
    struct Mapping {
        symbol: String,
        counterpart: String,
        wins_from: String,
        value: i32,
    }

    let mut mappings = [
        Mapping{
            symbol: String::from("X"),
            wins_from: String::from("C"),
            counterpart: String::from("A"),
            value: 1,

        },
        Mapping{
            symbol: String::from("Y"),
            wins_from: String::from("A"),
            counterpart: String::from("B"),
            value: 2,

        },
        Mapping{
            symbol: String::from("Z"),
            wins_from: String::from("B"),
            counterpart: String::from("C"),
            value: 3,
        },
    ];

    let mut score : i32 = 0;

    for line in input {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() == 2 {
            let opponent = mappings.clone().into_iter().find(|m| m.counterpart == parts[0]).unwrap();
            let own = mappings.clone().into_iter().find(|m| m.symbol == parts[1]).unwrap();

            score += own.value;
            if own.wins_from == opponent.counterpart {
                score += 6
            } else if own.counterpart == opponent.counterpart {
                score += 3
            }
        }
    }

    println!("{:?}", score)

}

pub fn advent_4() {
    let binding = fs::read_to_string("inputs/day_2").expect("Cannot read input file");

    // let binding = String::from("A Y\nB X\nC Z");
    let input = binding.split('\n');

    #[derive(Debug, Clone)]
    struct Mapping {
        symbol: String,
        wins_from: String,
        loses_to: String,
        value: i32,
    }

    let mut mappings = [
        Mapping{
            symbol: String::from("A"),
            wins_from: String::from("C"),
            loses_to: String::from("B"),
            value: 1,

        },
        Mapping{
            symbol: String::from("B"),
            wins_from: String::from("A"),
            loses_to: String::from("C"),
            value: 2,

        },
        Mapping{
            symbol: String::from("C"),
            wins_from: String::from("B"),
            loses_to: String::from("A"),
            value: 3,
        },
    ];

    let mut score : i32 = 0;

    for line in input {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() == 2 {
            let symbol = parts[0];
            let victory = parts[1];

            match victory {
                "Z" => {
                    let choice = mappings.clone().into_iter().find(
                        |m| m.wins_from == symbol
                    ).unwrap();
                    score += choice.value;
                    score += 6;
                },
                "Y" => {
                    let choice = mappings.clone().into_iter().find(
                        |m| m.symbol == symbol
                    ).unwrap();
                    score += choice.value;
                    score += 3
                },
                _ => {
                    let choice = mappings.clone().into_iter().find(
                        |m| m.loses_to == symbol
                    ).unwrap();
                    score += choice.value;
                    score += 0
                },


            }
        }
    }

    println!("{:?}", score)

}

pub fn advent_5() {
    let binding = fs::read_to_string("inputs/day_3").expect("Cannot read input file");
    // let binding = "\
    // vJrwpWtwJgWrhcsFMMfFFhFp
    // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    // PmmdzqPrVvPwwTWBwg
    // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    // ttgJtRGJQctTZtZT
    // CrZsJsPPZsGzwwsLwLmpwMDw
    // ".replace("    ", "");

    let input = binding.split('\n');

    fn split_in_half(s: &str) -> (&str, &str) {
        let len  = s.clone().len();
        s.split_at(len/2)
    }
    fn compare_parts(comp: (&str, &str)) -> i32{
        let first : &str = comp.0;
        let second : &str = comp.1;

        let mut score = 0;

        let a : HashSet<char> = first.chars().collect();
        let b : HashSet<char> = second.chars().collect();

        for char in a.intersection(&b) {
            let c = *char as u32;
            match c {
                64..=90 => {
                    score += (c - 38) as i32
                }
                97..=122 => {
                    score += (c - 96) as i32
                }
                _ => {
                    println!("{} {}", char, c);
                }
            }
        }
        score

    }

    let total : i32 = input.map(split_in_half).map(compare_parts).sum();

    println!("{:?}", total)
}

pub fn advent_6() {
    let binding = fs::read_to_string("inputs/day_3").expect("Cannot read input file");
//     let binding = "\
//     vJrwpWtwJgWrhcsFMMfFFhFp
//     jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//     PmmdzqPrVvPwwTWBwg
//     wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//     ttgJtRGJQctTZtZT
//     CrZsJsPPZsGzwwsLwLmpwMDw
//     ".replace("    ", "");

    let input = binding.split('\n');


    let mut total = 0;
    fn compare_elves(comp: (&str, &str, &str)) -> i32{
        let mut score = 0;

        let a : HashSet<char> = comp.0.chars().collect();
        let b : HashSet<char> = comp.1.chars().collect();
        let c : HashSet<char> = comp.2.chars().collect();

        for char in a.intersection(&b) {
            if c.contains(char) {
                let c = *char as u32;
                match c {
                    64..=90 => {
                        score += (c - 38) as i32
                    }
                    97..=122 => {
                        score += (c - 96) as i32
                    }
                    _ => {
                        println!("{} {}", char, c);
                    }
                }
            }
        }
        score

    }

    for group in input.collect::<Vec<&str>>().chunks(3) {
        if group.clone().len() >= 3 {
            let a = group[0];
            let b = group[1];
            let c = group[2];
            total += compare_elves((a, b, c));
        }
    }

    println!("{:?}", total);
}

pub fn advent_7() {
    let binding = fs::read_to_string("inputs/day_4").expect("Cannot read input file");
    // let binding = "\
    // 2-4,6-8
    // 2-3,4-5
    // 5-7,7-9
    // 2-8,3-7
    // 6-6,4-6
    // 2-6,4-8".replace("    ", "");

    let input = binding.split('\n');

    fn string_to_range(s: &str) -> Vec<i32> {
        let s: Vec<i32> = s.clone().split('-').map(|x| x.parse::<i32>().unwrap()).collect();
        return s
    }

    #[derive(Debug, Clone)]
    struct RangeComparison {
        elf1 : Range<i32>,
        elf2 : Range<i32>
    }

    impl RangeComparison {
        fn contains_another(&self) -> bool {
            let elf1 = &self.elf1;
            let elf2 = &self.elf2;

            let mut contains = false;
            if (elf1.start >= elf2.start) && (elf1.end <= elf2.end){
                contains = true
            }

            if (elf2.start >= elf1.start) && (elf2.end <= elf1.end){
                contains = true
            }

            return contains
        }
    }

    impl FromStr for RangeComparison {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if !s.is_empty() {
                let elves: Vec<Vec<i32>> = s.split(',').map(string_to_range).collect();

                return Ok(RangeComparison{
                    elf1: (elves[0][0]..elves[0][1]),
                    elf2: (elves[1][0]..elves[1][1]),
                })
            }

            return Err(());
        }
    }

    let mut coll: Vec<RangeComparison> = Vec::new();
    let mut contains : Vec<bool> = Vec::new();
    for line in input {
        let comp = RangeComparison::from_str(line);
        if comp.is_ok() {
            coll.push(comp.clone().unwrap());
            contains.push(comp.clone().unwrap().contains_another());

            println!("{:?}, {:?}", comp.clone(), comp.clone().unwrap().contains_another());
        }
    }

    println!("{}", contains.into_iter().filter(|b| *b).count());
}

pub fn advent_8() {
    let binding = fs::read_to_string("inputs/day_4").expect("Cannot read input file");
    // let binding = "\
    // 2-4,6-8
    // 2-3,4-5
    // 5-7,7-9
    // 2-8,3-7
    // 6-6,4-6
    // 2-6,4-8".replace("    ", "");

    let input = binding.split('\n');

    fn string_to_range(s: &str) -> Vec<i32> {
        let s: Vec<i32> = s.clone().split('-').map(|x| x.parse::<i32>().unwrap()).collect();
        return s
    }

    #[derive(Debug, Clone)]
    struct RangeComparison {
        elf1 : Range<i32>,
        elf2 : Range<i32>
    }

    impl RangeComparison {
        fn overlap(&self) -> bool {
            let elf1 = &self.elf1;
            let elf2 = &self.elf2;

            let mut contains = false;

            if (elf1.end >= elf2.start) && (elf1.start <= elf2.end){
                contains = true
            }

            return contains

        }
    }

    impl FromStr for RangeComparison {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if !s.is_empty() {
                let elves: Vec<Vec<i32>> = s.split(',').map(string_to_range).collect();

                return Ok(RangeComparison{
                    elf1: (elves[0][0]..elves[0][1]),
                    elf2: (elves[1][0]..elves[1][1]),
                })
            }

            return Err(());
        }
    }

    let mut coll: Vec<RangeComparison> = Vec::new();
    let mut contains : Vec<bool> = Vec::new();
    for line in input {
        let comp = RangeComparison::from_str(line);
        if comp.is_ok() {
            coll.push(comp.clone().unwrap());
            contains.push(comp.clone().unwrap().overlap());
        }
    }

    println!("{}", contains.into_iter().filter(|b| *b).count());
}
