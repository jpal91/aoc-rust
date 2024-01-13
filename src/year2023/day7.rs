#![allow(unused)]
use std::cmp::Ordering;

use crate::{get_puzzle, time_it};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Number(u32),
    Joker
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Hand(Card, Card, Card, Card, Card);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HandType {
    FiveOK,
    FourOK,
    FullHouse,
    ThreeOK,
    TwoPair,
    Pair,
    HC
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PlayerHand {
    hand: Hand,
    htype: HandType,
    bid: u32
}

impl Card {
    fn value(&self) -> u32 {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::Number(num) => *num,
            Card::Joker => 0
        }
    }
}

// impl PartialEq for Card {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Self::Joker, Self::Joker) => true,
//             (Self::Joker, _) => true,
//             (_, Self::Joker) => true,
//             _ => self.value() == other.value()
//         }
//     }
// }

// impl Eq for Card {}

impl Hand {
    fn from_string(input: &str, part1: bool) -> Self {
        let mut cards: Vec<Card> = vec![];

        for card in input.chars() {

            let c = match card {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' if part1 => Card::Jack,
                'J' => Card::Joker,
                'T' => Card::Number(10),
                num => Card::Number(num.to_string().parse::<u32>().unwrap())
            };
            cards.push(c)
        };

        Hand(*&cards[0], *&cards[1], *&cards[2], *&cards[3], *&cards[4])
    }

    fn sorted(&self) -> (Card, Card, Card, Card, Card) {
        let mut vals = vec![self.0, self.1, self.2, self.3, self.4];
        vals.sort_by_key(|k| -1 * k.value() as i32);
        (*&vals[0], *&vals[1], *&vals[2], *&vals[3], *&vals[4])
    }


}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (a, b) if a.0.value() != b.0.value() => self.0.value().cmp(&other.0.value()),
            (a, b) if a.1.value() != b.1.value() => self.1.value().cmp(&other.1.value()),
            (a, b) if a.2.value() != b.2.value() => self.2.value().cmp(&other.2.value()),
            (a, b) if a.3.value() != b.3.value() => self.3.value().cmp(&other.3.value()),
            _ => self.4.value().cmp(&other.4.value()),
        }
    }
}

impl HandType {
    fn from_hand(h: Hand) -> Self {
        let hand = h.sorted();
        match (hand.0, hand.1, hand.2, hand.3, hand.4) {
            (a, b, c, d , e) 
                if a == b && b == c && c == d && d == e => HandType::FiveOK,
            
            (a, b, c, d, e) 
                if (b == c && c == d) && (a == b || d == e) => HandType::FourOK,

            (a, b, c, d, e) 
                if a == b && (b == c || c == d) && d == e => HandType::FullHouse,
            
            (a, b, c, d, e) 
                if (a == b && b == c) || (b == c && c == d) || (c == d && d == e) => HandType::ThreeOK,
            
            (a, b, c, d, e) 
                if (a == b && (c == d || d == e)) || (d == e && (b == c || c == d)) => HandType::TwoPair,
            (a, b, c, d, e) 
                if a == b || b == c || c == d || d == e => HandType::Pair,
            //     let mut hand_vec: Vec<Card> = vec![a, b, c, d, e];
            //     hand_vec.dedup();
                
            //     if hand_vec.len() == 4 {
            //         HandType::Pair
            //     } else {
            //         HandType::TwoPair
            //     }
            // }

            _ => HandType::HC
        }
    }

    fn value(&self) -> u32 {
        match self {
            HandType::FiveOK => 700,
            HandType::FourOK => 600,
            HandType::FullHouse => 500,
            HandType::ThreeOK => 400,
            HandType::TwoPair => 300,
            HandType::Pair => 200,
            _ => 100
        }
    }

    fn check_dedup(sli: &mut [Card], count: usize) -> bool {
        let mut dd = sli.to_vec();
        dd.dedup();
        dd.len() == count
    }
}

impl PlayerHand {
    fn from_string(input: &str) -> Self {
        let input: Vec<&str> = input.split(" ").take(2).collect();
        let (hand, bid) = (Hand::from_string(*&input[0], false), *&input[1].parse::<u32>().unwrap());
        let htype = HandType::from_hand(hand);

        Self {
            hand,
            htype,
            bid
        }
        
    }
    
    fn value(&self) -> u32 {
        if let Some(jokers) = self.no_jokers() {
            let mut val = self.htype.value();
            
            match (jokers, self.htype) {
                (1, HandType::FourOK) => val += 100,
                (2 | 3, HandType::FullHouse) => val += 200,
                (2, HandType::TwoPair) => val += 300,
                (1 , HandType::ThreeOK | HandType::TwoPair | HandType::Pair) => val += 200,
                (1, HandType::HC)  => val += 100,
                _ => {}
            }
            val
        } else {
            self.htype.value()
        }
        // self.htype.value()
    }

    fn no_jokers(&self) -> Option<u32> {
        let res: u32 = self.hand_to_vec()
            .into_iter()
            .filter(|x| *x == Card::Joker)
            .count() as u32;
        
        if res > 0 {
            Some(res)
        } else {
            None
        }
    }

    fn hand_to_vec(&self) -> Vec<Card> {
        vec![self.hand.0, self.hand.1, self.hand.2, self.hand.3, self.hand.4]
    }
}

impl PartialOrd for PlayerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlayerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.value(), self.hand).cmp(&(other.value(), other.hand))
    }
}

fn parse(input: &str, part1: bool) -> Vec<(Hand, u32, HandType)> {
    input
        .lines()
        .map(|l| {
            let mut cb = l.split(" ");
            let (cards, bid): (Hand, u32) = (Hand::from_string(cb.next().unwrap(), part1), cb.next().unwrap().parse().unwrap());
            (cards, bid, HandType::from_hand(cards))
        })
        .collect()
}

fn parse_pt2(input: &str) -> Vec<PlayerHand> {
    input
        .lines()
        .map(|l| {
            PlayerHand::from_string(l)
        })
        .collect()
}


fn solution_pt1(input: &str) -> u32 {
    let mut puzzle = parse(input, true);
    puzzle.sort_by_key(|k| (k.2.value(), k.0));

    puzzle
        .iter()
        .enumerate()
        .map(|(i, v)| v.1 * (i + 1) as u32)
        .sum()
}

fn solution_pt2(input: &str) -> u32 {
    let mut puzzle = parse_pt2(input);
    puzzle.sort();

    // for p in puzzle.iter() {
    //     println!("{:?}", p)
    // };

    puzzle
        .iter()
        .enumerate()
        .map(|(i, p)| p.bid * (i + 1) as u32)
        .sum()
}

pub fn main() {
    let puzzle = get_puzzle("23", "7");

    time_it!("Part 1", solution_pt1(&puzzle));
    time_it!("Part 2", solution_pt2(&puzzle));
}


#[cfg(test)]
mod tests {
    use super::*;
    const TEST: &'static str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483    
";


    #[test]
    fn test_hand () {
        let hand = Hand::from_string("TQAKJ", true);
        assert_eq!(hand, Hand(Card::Ace, Card::King, Card::Queen, Card::Jack, Card::Number(10)))
    }

    #[test]
    fn test_hand_value() {
        let hand = Hand(Card::Ace,Card::Ace,Card::Ace,Card::Ace,Card::Ace);
        assert_eq!(HandType::FiveOK, HandType::from_hand(hand));

        let hand = Hand(Card::Ace,Card::Ace,Card::Ace,Card::Ace,Card::Queen);
        assert_eq!(HandType::FourOK, HandType::from_hand(hand));

        let hand = Hand(Card::Ace,Card::Ace,Card::King,Card::King,Card::King);
        assert_eq!(HandType::FullHouse, HandType::from_hand(hand));
        
        let hand = Hand(Card::Ace,Card::King,Card::King,Card::King,Card::Queen);
        assert_eq!(HandType::ThreeOK, HandType::from_hand(hand));

        let hand = Hand(Card::Ace,Card::King,Card::King,Card::Number(10),Card::Number(10));
        assert_eq!(HandType::TwoPair, HandType::from_hand(hand));

        let hand = Hand(Card::Ace,Card::King,Card::King,Card::Queen,Card::Number(7));
        assert_eq!(HandType::Pair, HandType::from_hand(hand));

        let hand = Hand(Card::Ace,Card::King,Card::Queen,Card::Jack,Card::Number(3));
        assert_eq!(HandType::HC, HandType::from_hand(hand));
    }

    #[test]
    fn test_parse() {
        let parse = parse(TEST, true);

        assert_eq!(&parse[0], &(
            Hand(Card::King, Card::Number(10), Card::Number(3), Card::Number(3), Card::Number(2)),
            765,
            HandType::Pair
        ));

        assert_eq!(&parse[1], &(
            Hand(Card::Jack, Card::Number(10), Card::Number(5), Card::Number(5), Card::Number(5)),
            684,
            HandType::ThreeOK
        ))
    }

    #[test]
    fn test_solution1() {
        let res = solution_pt1(TEST);
        assert_eq!(res, 6440)
    }

    #[test]
    fn test_peq() {
        assert_ne!(Card::Number(3), Card::Number(4))
    }

    #[test]
    fn test_player_hand() {
        let mut hands = parse_pt2(TEST);
        hands.sort();

        for h in hands.iter() {
            println!("{:?}, {}", h, h.value())
        }
    }

    #[test]
    fn test_part2() {
        let res = solution_pt2(TEST);

        assert_eq!(res, 5905)
    }

    #[test]
    fn test_dedup() {
        let mut test = [Card::King, Card::Number(10), Card::Number(10), Card::Joker, Card::Joker];
        let mut deduped = test.to_vec();
        deduped.dedup();

        println!("{:?}", deduped)
    }
}