use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    // this is another alternative to casting with AS. 
    let b_ = b.try_into().unwrap();

    // This comparison is a compile fail as  we cannot compare mixed numerical types
    // if a < b {
    //     println!("Ten is less than one hundred.");
    // }

    // to fix it was can cast one of the variables (safer to cast the lower sized variable up)
    if a < (b as i32) {
        println!("Ten is less than one hundred.");
    }

    // This also works because b_ 
    if a < b_ {
        println!("Ten is less than one hundred.");
    }


}
