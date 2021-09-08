use std::time::{Duration, Instant};

fn main() {
    // The following code chunk tests how high the computer can increment the counter in 1 second. 
    // not sure what its meant to teach but I did it anyway. 
    let mut count = 0;
    let time_limit = Duration::new(1,0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }

    println!("{}", count);


    // While true is avoided in rust         
    #[warn(while_true)]
    while true {
        break;
    }

    // in favour of the loop keyword which does the same job but cleaner
    loop {
        break;      
    }

    // break will break out of the closest loop
    for (x,y) in (0..).zip(0..) {
        if x + y > 100 {
            break;
        }
    }


    // if you want to break out of an outer loop you can use loop labels to specify which loop to break from
    'outer: for x in 0.. {
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 1000 {
                    break 'outer;
                }
            }
        }
    }
}                   
