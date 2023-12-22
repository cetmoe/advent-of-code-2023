fn main() {
    let input = include_str!("../input.txt").trim();

    let mut cards: Vec<u32> = Vec::new();

    input.split("\n").for_each(|_| cards.push(1));

    input.split("\n").enumerate().for_each(|(i, card)| {
        let numbers_in_card = card
            .split(":")
            .nth(1)
            .unwrap()
            .split("|")
            .map(|segment| {
                segment
                    .split(" ")
                    .filter(|f| *f != "")
                    .map(|s| s.trim().parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        numbers_in_card
            .get(0)
            .unwrap()
            .iter()
            .fold(0, |mut acc, n| {
                if numbers_in_card.get(1).unwrap().contains(n) {
                    acc += 1;

                    let multiplier = *cards.clone().get(i).unwrap();

                    if let Some(changing_card) = cards.get_mut(i + acc) {
                        *changing_card += multiplier * 1;
                    }
                }
                acc
            });
    });

    println!("{:?}", cards);
    println!(" {:?}", cards.iter().sum::<u32>());
}
