fn main() {
    println!("Hello, world!");
    another_function(5);

    println!("{}", {
        let array = [2; 5];
        calculate_sum(array)
    });
}

fn another_function(x: i32) {
    println!("Another function {x}");
}

fn calculate_sum(array: [i32; 5]) -> i32 {
    let mut sum: i32 = 0;

    for i in 0..array.len() {
        sum = sum + array[i];
    }

    sum
}
