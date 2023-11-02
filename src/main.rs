fn main() {
    let mut numbers = [1, 2, 3, 4, 5];
    let num = 4;
    match linear_search(&mut numbers, num) {
        true => println!("{} is in the array", num),
        false => println!("{} is not in the array", num),
    };
}

// Linear Search
fn linear_search(haystack: &mut [i32], needle: i32) -> bool {
    for num in haystack {
        // Bad idea to return in for loop
        if *num == needle { return true; }
    }

    false
}
