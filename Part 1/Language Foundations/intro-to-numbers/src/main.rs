fn main() {
    // rust will infer the type here
    let twenty = 20;
    // explicitly declaring 21 to be a 32bit integer
    let twenty_one: i32 = 21;
    // same again just a different way of doing it
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    // these underscores are ignored by the compiler, so 1_000_000 becomes 1000000
    let one_million:i64 = 1_000_000;
    println!("{}",one_million.pow(2));

    // create an array of numbers which must all be the same type
    let forty_twos = [
        // inferred type
        42.0,
        // explicit type annotation
        42f32,
        // explicit type annotation with underscores for readability
        42.0_f32,
    ];

    println!("{:02}", forty_twos[0]);
}