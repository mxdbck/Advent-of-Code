mod data;

fn main() {
    let data = data::get_data();
    let calories_per_elf = data.split("\r\n\r\n");
    let mut max_calories = u32::MIN;
    for calorie_per_elf in calories_per_elf {
        let mut total_calories = 0;
        let calorie_array = calorie_per_elf.split("\n");
        for calorie in calorie_array {
            // println!("{}", calorie);
            // println!("{}", "51".parse::<u32>().unwrap());
            match calorie.trim().parse::<u32>() {
                Ok(_) => (),
                Err(_) => continue,
            }
            let calorie = calorie.trim().parse::<u32>().unwrap();
            total_calories += calorie;
        }
        // println!("{}", total_calories);
        max_calories = max_calories.max(total_calories);
    }
    println!("{}", max_calories);
}

fn make_newlines_visible(s: &str) -> String {
    s.chars().map(|c| match c {
        '\n' => "\\n".to_string(),
        '\r' => "\\r".to_string(),
        _ => c.to_string(),
    }).collect()
}