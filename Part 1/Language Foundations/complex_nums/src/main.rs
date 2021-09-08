// from the num crate, bring is the Complex type which is under the complex scope in that crate
use num::complex::Complex;

fn main() {
    // create 'a' as a Complex type and initialise it manually by assigning each field a value
    let a = Complex { re:2.1, im: -1.2};
    // create 'b' as a Complex type but initialise it with a constructor method that just needs values passed in to return a Complex type
    // update: ok apparently its not a constructor method in the traditional sense because rust doesnt have them by default, but i think its the same effect
    // the Complex author created a 'new' method which is essentially a constructor for the object type
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}
