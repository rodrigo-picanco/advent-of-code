use itertools::Itertools;

use nom::IResult;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process_input(input);
    dbg!(output);
}

fn process_input(input: &str) -> usize {
    let (_, games) = parse_input(input).expect("Failed to parse input");
    let count = games.len();

    games
        .iter()
        .sorted_by(|(hand1, _), (hand2, _)| {
            // The order of cards to be cmp matters!
            let comp = hand2.game.score().cmp(&hand1.game.score());
            match comp {
                std::cmp::Ordering::Equal => {
                    for i in 0..5 {
                        let comp = card_score(hand2.cards[i]).cmp(&card_score(hand1.cards[i]));
                        if comp != std::cmp::Ordering::Equal {
                            return comp;
                        }
                    }
                    panic!("Two hands are equal")
                }
                _ => comp,
            }
        })
        .enumerate()
        .map(|(i, (_, bid))| bid * (count - i))
        .sum()
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn new(cards: HandCards) -> Self {
        use HandType::*;
        let counts = cards.iter().sorted().counts();

        match counts.values().sorted().as_slice() {
            [1, 1, 1, 1, 1] => HighCard,
            [1, 1, 1, 2] => OnePair,
            [1, 2, 2] => TwoPair,
            [1, 1, 3] => ThreeOfKind,
            [2, 3] => FullHouse,
            [1, 4] => FourOfKind,
            [5] => FiveOfKind,
            _ => panic!("Invalid hand"),
        }
    }
    fn score(&self) -> usize {
        use HandType::*;
        match self {
            FiveOfKind => 7,
            FourOfKind => 6,
            FullHouse => 5,
            ThreeOfKind => 4,
            TwoPair => 3,
            OnePair => 2,
            HighCard => 1,
        }
    }
}

fn card_score(card: char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as usize,
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Hand {
    cards: HandCards,
    game: HandType,
}

impl Hand {
    fn new(cards: HandCards) -> Self {
        let game = HandType::new(cards.to_vec());
        Self { cards, game }
    }
}

type HandCards = Vec<char>;
type Bid = usize;

fn parse_input(input: &str) -> IResult<&str, Vec<(Hand, Bid)>> {
    let (input, output) = nom::multi::many1(nom::sequence::terminated(
        nom::sequence::separated_pair(
            nom::multi::count(nom::character::complete::one_of("23456789TJQKA"), 5),
            nom::character::complete::space1,
            nom::character::complete::digit1,
        ),
        nom::branch::alt((nom::character::complete::line_ending, nom::combinator::eof)),
    ))(input)?;

    let output = output
        .iter()
        .map(|(cards, bid)| (Hand::new(cards.to_vec()), bid.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    Ok((input, output))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_game_type() {
        assert_eq!(
            HandType::new(vec!['A', 'A', 'A', 'A', 'A']),
            HandType::FiveOfKind
        );
        assert_eq!(
            HandType::new(vec!['A', 'A', 'A', 'A', 'B']),
            HandType::FourOfKind
        );
        assert_eq!(
            HandType::new(vec!['A', 'A', 'A', 'B', 'B']),
            HandType::FullHouse
        );
        assert_eq!(
            HandType::new(vec!['A', 'A', 'A', 'B', 'C']),
            HandType::ThreeOfKind
        );
        assert_eq!(
            HandType::new(vec!['A', 'A', 'B', 'B', 'C']),
            HandType::TwoPair
        );
        assert_eq!(
            HandType::new(vec!['A', 'A', 'B', 'C', 'D']),
            HandType::OnePair
        );
        assert_eq!(
            HandType::new(vec!['A', 'B', 'C', 'D', 'E']),
            HandType::HighCard
        );
    }

    #[test]
    fn test_game_score() {
        assert_eq!(HandType::FiveOfKind.score(), 7);
        assert_eq!(HandType::FourOfKind.score(), 6);
        assert_eq!(HandType::FullHouse.score(), 5);
        assert_eq!(HandType::ThreeOfKind.score(), 4);
        assert_eq!(HandType::TwoPair.score(), 3);
        assert_eq!(HandType::OnePair.score(), 2);
        assert_eq!(HandType::HighCard.score(), 1);
    }

    #[test]
    fn test_parse_input() {
        let (_, output) = parse_input(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        )
        .expect("Failed to parse input");

        assert_eq!(
            output,
            vec![
                (Hand::new(vec!['3', '2', 'T', '3', 'K']), 765),
                (Hand::new(vec!['T', '5', '5', 'J', '5']), 684),
                (Hand::new(vec!['K', 'K', '6', '7', '7']), 28),
                (Hand::new(vec!['K', 'T', 'J', 'J', 'T']), 220),
                (Hand::new(vec!['Q', 'Q', 'Q', 'J', 'A']), 483),
            ]
        )
    }

    #[test]
    fn test_process_input() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(process_input(input), 6440);
    }
}
