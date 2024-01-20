use std::io;

fn main() {
    work_with_integers();
    work_with_tuples();
    work_with_array();
}

fn work_with_integers() {
    println!("\nStart work with integers");

    let small: u16 = u16::MAX;

    let integer: u32 = (small as u32) + 1;

    println!("Small: {small}, Int: {integer}");

    let not_so_small: u16 = match small.checked_add(1) {
        Some(value) => value,
        None => u16::MAX,
    };

    println!("Checked add returns {not_so_small}");
}

fn work_with_tuples() {
    println!("\nStart work with tuples");

    let point: (char, u8, u8) = ('S', 255, 16);
    let (name, x, y) = point;
    let point_name = point.0;

    println!("The point '{name}' is placed in: {x}:{y} position");
    println!("You may want to move '{point_name}' to other position");
}

fn work_with_array() {
    println!("\nStart work with array");

    let dif_value_array: [char; 10] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'x', 'y', 'z'];
    let same_value_array = ['a'; 9];

    let mut index = String::new();

    println!("Input an index:");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read the line");

    let index: usize = index.trim().parse().expect("Index is not a number");

    let selected_value_dif = dif_value_array[index];
    let selected_value_same = same_value_array[index];

    println!("Dif: {selected_value_dif}; Same: {selected_value_same}");
}
