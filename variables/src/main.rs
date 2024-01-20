const MY_CONST_IS_GLOBAL: &str = "Global const";

fn main() {
    let mut x = 5;
    println!("The value is: {x}");

    {
        let x = x + 2;
        println!("Shadowing variable: {x}");
    }

    x = 6;
    println!("The value is: {x} {MY_CONST_IS_GLOBAL}");
}
