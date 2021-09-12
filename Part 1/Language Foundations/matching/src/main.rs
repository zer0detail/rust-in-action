fn main() {
    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "Hit!",
            _ => "miss",
        };
    if result == "Hit!" {
        println!("{}: {}", item, result);
    }
    }
}
