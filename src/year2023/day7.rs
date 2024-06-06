use crate::{debug_output, get_puzzle, time_it};
use std::cmp::Reverse;
use std::sync::{Mutex, OnceLock};
use std::{collections::HashMap, ops::Deref};

#[derive(Eq, Debug, Clone, PartialEq)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Num(u32),
    Joker,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
enum HandType {
    FiveOK = 7,
    FourOK,
    FullHouse,
    ThreeOK,
    TwoPair,
    Pair,
    High,
}

#[derive(Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
struct Hand(Vec<Card>);

#[derive(Debug)]
struct Player {
    hand: Hand,
    bid: u32,
}

impl Hand {
    fn new(input: &str, part2: bool) -> Self {
        Self(input.chars().map(|c| Card::new(c, part2)).collect())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.deref().cmp(self.deref())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Deref for Card {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        use Card::*;

        match self {
            Ace => &14,
            King => &13,
            Queen => &12,
            Jack => &11,
            Ten => &10,
            Joker => &0,
            Num(n) => n,
        }
    }
}

impl Deref for Hand {
    type Target = [Card];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl Card {
    fn new(card: char, part2: bool) -> Self {
        use Card::*;

        match card {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' if part2 => Joker,
            'J' => Jack,
            'T' => Ten,
            a => Num(a.to_string().parse().unwrap()),
        }
    }
}

impl From<i32> for Card {
    fn from(value: i32) -> Self {
        use Card::*;

        match value {
            14 => Ace,
            13 => King,
            12 => Queen,
            11 => Jack,
            10 => Ten,
            0 => Joker,
            any => Num(any as u32),
        }
    }
}

impl HandType {
    fn new(mut hand: impl Iterator<Item = i32>) -> Self {
        use HandType::*;
        if let Some(card) = hand.next() {
            match card {
                5 => return FiveOK,
                4 => return FourOK,
                3 => {
                    if let Some(next) = hand.next() {
                        match next {
                            2 => return FullHouse,
                            1 => return ThreeOK,
                            _ => unreachable!(),
                        }
                    } else {
                        return ThreeOK;
                    }
                }
                2 => {
                    if let Some(next) = hand.next() {
                        match next {
                            2 => return TwoPair,
                            1 => return Pair,
                            n => unreachable!("{}", n),
                        }
                    } else {
                        return Pair;
                    }
                }
                1 => return High,
                _ => unreachable!(),
            }
        }

        High
    }
}

fn counter(hand: &Hand) -> HandType {
    let mut cards: HashMap<i32, i32> = HashMap::new();
    hand.iter().for_each(|c| {
        cards.entry(**c as i32).and_modify(|e| *e += 1).or_insert(1);
    });

    let cards = match cards.remove(&0) {
        Some(5) => vec![5],
        Some(joker) => {
            let mut cards: Vec<i32> = cards.values().copied().collect();
            cards.sort_by_key(|k| -k);
            cards[0] = (cards[0] + joker).min(5);
            cards
        }
        None => {
            let mut cards: Vec<i32> = cards.values().copied().collect();
            cards.sort_by_key(|k| -k);
            cards
        }
    };

    HandType::new(cards.into_iter())
    // hand_type
}

fn parse(input: &str, part2: bool) -> Vec<Player> {
    input
        .lines()
        .map(|line| {
            let mut player = line.split(' ');
            let mut hand = Hand::new(player.next().unwrap(), part2);
            let bid = player.next().unwrap().trim().parse().unwrap();
            Player { hand, bid }
        })
        .collect()
}

fn solution(input: &str, part2: bool) -> u32 {
    let players = parse(input, part2);
    let mut hands = players
        .into_iter()
        .map(|p| (p.bid, counter(&p.hand), p.hand.clone()))
        .collect::<Vec<_>>();
    hands.sort_by_key(|a| Reverse((a.1.clone(), a.2.clone())));

    hands
        .into_iter()
        .map(|h| h.0)
        .enumerate()
        .fold(0, |acc, (i, bid)| acc + (bid * (i as u32 + 1)))
}

pub fn main() {
    let puzzle = get_puzzle("23", "7");

    time_it!("Part 1", solution(&puzzle, false));
    time_it!("Part 2", solution(&puzzle, true));
}

#[cfg(test)]
const TEST_ONE: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        let res = solution(TEST_ONE, false);

        assert_eq!(res, 6440)
    }

    #[test]
    fn test_two() {
        let res = solution(TEST_ONE, true);

        assert_eq!(res, 5905)
    }
}
