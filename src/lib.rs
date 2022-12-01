use std::fs;

pub fn advent_1() {
    let binding = fs::read_to_string("inputs/day_1").expect("Cannot read input file");
    let input = binding.split("\n");

    let mut highest : i32 = 0;
    let mut current : i32 = 0;
    for line in input {
        if line == "" {
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
    let input = binding.split("\n");

    let mut calorie_list : Vec<i32> = Vec::new();
    let mut current : i32 = 0;
    for line in input {
        if line == "" {
            calorie_list.push(current);
            current = 0;
        }

        let calories : i32 = line.parse().unwrap_or(0);
        current += calories;
    }

    calorie_list.sort();
    let third_last = calorie_list.len().saturating_sub(3);
    println!("{:?}",calorie_list[third_last..].iter().sum::<i32>())

}
