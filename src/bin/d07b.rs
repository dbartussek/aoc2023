use aoc2023::read_input_lines_skip_empty;
use counter::Counter;
use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
enum Card {
    Joker,
    Number(u8),
    Ten,
    Queen,
    King,
    Ace,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
enum HandValue {
    Single,
    Pair,
    TwoPair,
    Three,
    House,
    Four,
    Five,
}

impl Card {
    pub fn from_char(c: char) -> Self {
        match c {
            '0'..='9' => Self::Number(c as u8 - ('0' as u8)),
            'T' => Self::Ten,
            'J' => Self::Joker,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let lines = read_input_lines_skip_empty(7);

    let mut hands = lines
        .iter()
        .map(|l| {
            let mut l = l.trim().split(' ');
            let hand = l.next().unwrap().chars().map(Card::from_char).collect_vec();
            let bet = l.next().unwrap().parse::<u64>().unwrap();

            (hand, bet)
        })
        .collect_vec();

    hands.sort_by_cached_key(|(hand, _)| {
        let counter: Counter<_> = hand.iter().copied().collect();
        let mut common = counter.most_common();

        let jokers = common
            .iter()
            .copied()
            .find(|(card, _)| *card == Card::Joker)
            .map(|(_, amount)| amount)
            .unwrap_or_default();
        if jokers == 5 {
            return (HandValue::Five, hand.clone());
        }

        common.retain(|(card, _)| *card != Card::Joker);

        let value = match common[0].1 + jokers {
            1 => HandValue::Single,
            4 => HandValue::Four,
            5 => HandValue::Five,

            2 => {
                if common[1].1 == 2 {
                    HandValue::TwoPair
                } else {
                    HandValue::Pair
                }
            },
            3 => {
                if common[1].1 == 2 {
                    HandValue::House
                } else {
                    HandValue::Three
                }
            },
            _ => unimplemented!(),
        };
        (value, hand.clone())
    });

    let total: u64 = hands
        .iter()
        .enumerate()
        .map(|(index, (_, bet))| (index as u64 + 1) * bet)
        .sum();
    println!("{total}");
}
