use std::fs;

pub fn solve_day1() {
    println!("[Day1] Reading input.");

    let contents = fs::read_to_string("inputs/day1.txt")
        .expect("[Day1] File read.");

    let splits = contents.split("\n");
    let array_size = 3;

    let mut max_calories = vec![0; array_size];

    let mut current_elf = 0;
    let mut current_calories = 0;

    for s in splits {
        if !s.is_empty() {
            current_calories += s.parse::<i32>().unwrap();
            continue;
        }

        if current_calories > max_calories[0] {
            max_calories[0] = current_calories;
            max_calories.sort();
        }

        current_elf += 1;
        current_calories = 0;
    }

    println!("[Day 1.1] Most calories: {}", max_calories[2]);
    println!("[Day 1.2] Top {} combined calories: {}", array_size, max_calories.iter().sum::<i32>());
}