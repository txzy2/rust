pub fn arr(value: usize) {
    let arr: Vec<Vec<u32>> = vec![vec![0; value]; value];

    println!("Your array: ");

    for row in arr.iter() {
        let row_string = row
            .iter()
            .map(|&num| num.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        println!("{}", row_string);
    }
}

pub fn sum(x: u32, y: u32) -> u32 {
    return x + y;
}
