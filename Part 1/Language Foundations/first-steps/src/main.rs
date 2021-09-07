fn main() {
    // let the type be inferred by the compiler
    let a = 10;
    // explicitly declare the type  when creating the variable
    let b: i32 = 20;
    // number type can have their type annotation as part of their value declaration
    let c = 30i32;
    // and you can add an underscore to inscrease readability
    let d = 30_i32;

    let e = add(add(a,b), add(c,d));

    println!("(a + b)+(c + d) = {}", e);
    
}


fn add(i: i32, j: i32) -> i32 {
    i + j
}