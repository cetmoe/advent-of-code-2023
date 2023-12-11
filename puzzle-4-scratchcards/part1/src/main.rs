fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .trim()
            .split("\n")
            .map(|line| {
                let winning_and_my_numbers = line
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .split("|")
                    .map(|input| {
                        input
                            .trim()
                            .split(" ")
                            .filter(|numbah| *numbah != "")
                            .map(|number_as_string| number_as_string.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                    })
                    .collect::<Vec<Vec<usize>>>();

                let mut count = 0;
                winning_and_my_numbers
                    .get(1)
                    .unwrap()
                    .iter()
                    .for_each(|my_number| {
                        if winning_and_my_numbers.get(0).unwrap().contains(my_number) {
                            count += 1;
                        }
                    });
                if count > 0 {
                    let x = 2_u32.pow((count - 1) as u32);
                    x
                } else {
                    0
                }
            })
            .sum::<u32>()
    )
}
