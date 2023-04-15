fn main() {
    let result = get_pyramid(3, 10);
    match result {
        Ok(pyramid) => println!("{}", pyramid),
        Err(error) => println!("[ERROR]: {}", error),
    }
}

fn get_pyramid(start: i32, total_numbers: i32) -> Result<String, String> {
    if start < 0 || 9 < start {
        return Err("Start value must be between 0 and 9".to_string());
    }

    if !can_create_pyramid(total_numbers) {
        return Err("Pyramid cannot be created".to_string());
    }

    let number_of_rows = get_number_of_rows(total_numbers);
    let number_of_columns = get_number_of_columns(number_of_rows);
    let mut grid: Vec<i32> = Vec::with_capacity(total_numbers as usize);
    let mut jumper = number_of_rows;
    let mut current = start;

    for _ in 0..total_numbers {
        grid.push(0);
    }

    grid[0] = current;
    current += 1;

    let mut skip = 2;

    while jumper > 0 {
        // Moving down
        for i in 0..jumper {
            println!("current: {}, inserting: {}", current, i + skip);
            grid.insert((i + skip) as usize, current);
            current += 1;
            skip += 1;
        }

        jumper -= 1;

        // Moving left
        for i in 0..jumper {}

        jumper -= 1;

        // Moving Up
        for i in 0..jumper {}

        jumper -= 1;
    }

    println!("{:?}", grid);

    Ok("end".to_string())
}

fn can_create_pyramid(total_numbers: i32) -> bool {
    let mut level = 1;
    let mut jumper = 2;

    while level < total_numbers {
        level += jumper;
        jumper += 1;
    }

    level == total_numbers
}

fn get_number_of_rows(mut total_numbers: i32) -> i32 {
    let mut jumper = 1;

    while total_numbers > 0 {
        total_numbers -= jumper;
        jumper += 1;
    }

    jumper - 1
}

fn get_number_of_columns(number_of_rows: i32) -> i32 {
    number_of_rows * 2 - 1
}
