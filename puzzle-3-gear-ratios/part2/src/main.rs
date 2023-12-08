fn main() {
    let input = include_str!("../input.txt").trim();

    let matrix = input
        .split("\n")
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut output = Vec::<usize>::new();

    let gear_ratios = input.split("\n").enumerate().for_each(|(row_index, line)| {
        line.chars().enumerate().for_each(|(column_index, c)| {
            if c == '*' {
                let mut collection = Vec::<String>::new();
                for i in (row_index - 1)..=(row_index + 1) {
                    let mut it = (column_index - 1)..=(column_index + 1);
                    while let Some(j) = it.next() {
                        if let Some(row) = matrix.get(i) {
                            if let Some(item) = row.get(j) {
                                if item.is_numeric() {
                                    let mut left: String = (&row[..j])
                                        .iter()
                                        .rev()
                                        .take_while(|cl| cl.is_numeric())
                                        .collect::<String>()
                                        .chars()
                                        .rev()
                                        .collect();
                                    let right: String = (&row[j..])
                                        .iter()
                                        .take_while(|cr| cr.is_numeric())
                                        .collect();

                                    left.push_str(right.as_str());
                                    collection.push(left.clone());

                                    it.nth(right.len() - 1);
                                }
                            }
                        }
                    }
                }
                if collection.len() == 2 {
                    output.push(
                        collection
                            .iter()
                            .map(|i| i.parse::<usize>().unwrap())
                            .reduce(|a, b| a * b)
                            .unwrap(),
                    )
                }
            }
        })
    });

    println!("{}", output.iter().sum::<usize>());
}
