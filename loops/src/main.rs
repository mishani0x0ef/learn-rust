use std::io;

fn main() {
    let position = get_input();

    println!("[loop] Fibonacci at position {position} is {}", {
        get_fibonacci_with_loop(position)
    });

    println!("[while] Fibonacci at position {position} is {}", {
        get_fibonacci_with_while(position)
    });

    println!("[for] Fibonacci at position {position} is {}", {
        get_fibonacci_with_for(position)
    });

    println!("Checking if all even: {}", {
        check_if_all_even([2, 4, 8, 10, 3])
    })
}

fn get_input() -> u32 {
    println!("Input position of the fibonacci number you want to get");

    let mut position = String::new();

    io::stdin()
        .read_line(&mut position)
        .expect("Cannot read input");

    let position: u32 = position.trim().parse().expect("Position is not a number");

    position
}

fn get_fibonacci_with_loop(position: u32) -> u64 {
    let mut previous: u64 = 1;
    let mut current: u64 = 2;
    let mut counter: u64 = 2;

    if position == 1 {
        return previous;
    }

    if position == 2 {
        return current;
    }

    let result = loop {
        let latest_current = current;

        counter = counter + 1;
        current = previous + current;
        previous = latest_current;

        if counter == position.into() {
            break current;
        }
    };

    result
}

fn get_fibonacci_with_while(position: u32) -> u64 {
    let mut previous: u64 = 1;
    let mut current: u64 = 2;
    let mut counter: u64 = 2;

    if position == 1 {
        return previous;
    }

    while counter != position.into() {
        let latest_current = current;

        counter = counter + 1;
        current = previous + current;
        previous = latest_current;
    }

    current
}

fn get_fibonacci_with_for(position: u32) -> u64 {
    let mut previous: u64 = 0;
    let mut current: u64 = 1;

    for _ in 0..position {
        let latest_current = current;

        current = previous + current;
        previous = latest_current;
    }

    current
}

fn check_if_all_even(array: [i32; 5]) -> bool {
    for item in array {
        if item % 2 > 0 {
            return false;
        }
    }

    return true;
}
