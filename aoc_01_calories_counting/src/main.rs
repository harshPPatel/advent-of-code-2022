use std::fs;

static FILEPATH: &str = "./src/input.txt";

fn main() {
    let contents = fs::read_to_string(FILEPATH).expect("Something went wrong reading the file");
    let elfs = contents.split("\n\n");

    let mut total_calories_vector: Vec<i32> = vec![];

    for elf in elfs {
        let elf_calories = elf.split("\n");
        let mut total_calories = 0;
        for calories in elf_calories {
            let parsed_calories = calories.parse::<i32>().expect("Invalid Calories");
            total_calories = total_calories + parsed_calories;
        }
        total_calories_vector.push(total_calories)
    }

    total_calories_vector.sort();
    total_calories_vector.reverse();

    let highest_total = total_calories_vector.first().expect("Something went wrong for sorted total calories");
    let top_three: i32 = total_calories_vector.iter().take(3).sum();

    println!("Highest: {}", highest_total);
    println!("Sum of Top 3: {}", top_three);
}
