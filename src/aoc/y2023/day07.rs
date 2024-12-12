// Copyright (c) 2024 Lexi Robinson
//
// Licensed under the EUPL, Version 1.2
//
// You may not use this work except in compliance with the Licence.
// You should have received a copy of the Licence along with this work. If not, see:
// <https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12>.
// See the Licence for the specific language governing permissions and limitations under the Licence.

use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use itertools::Itertools;

use crate::AoCError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = AoCError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Joker),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err(AoCError::new(format!("Unknown card {value}"))),
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ace => 'A',
                Self::King => 'K',
                Self::Queen => 'Q',
                Self::Joker => 'J',
                Self::Ten => 'T',
                Self::Nine => '9',
                Self::Eight => '8',
                Self::Seven => '7',
                Self::Six => '6',
                Self::Five => '5',
                Self::Four => '4',
                Self::Three => '3',
                Self::Two => '2',
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn new_from_cards(cards: &[Card; 5]) -> Self {
        let mut map: HashMap<Card, u32> = HashMap::with_capacity(5);
        for card in cards.iter() {
            match map.get_mut(card) {
                Some(v) => *v += 1,
                None => {
                    map.insert(*card, 1);
                }
            }
        }
        if map.len() > 1 {
            let num_jokers = map.remove(&Card::Joker);
            if let Some(num_jokers) = num_jokers {
                let wat = map
                    .values_mut()
                    .sorted_by_key(|value| **value)
                    .next_back()
                    .unwrap();
                *wat += num_jokers;
            }
        }

        match map.len() {
            5 => Self::HighCard,
            4 => Self::OnePair,
            3 | 2 => {
                let mut values: Vec<_> = map.values().collect();
                values.sort();
                values.reverse();
                match values.as_slice() {
                    [2, 2, 1] => Self::TwoPair,
                    [3, 1, 1] => Self::ThreeOfAKind,
                    [3, 2] => Self::FullHouse,
                    [4, 1] => Self::FourOfAKind,
                    _ => unreachable!("unexpected map values {values:?}"),
                }
            }
            1 => Self::FiveOfAKind,
            _ => unreachable!("Unexpected map length!"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Hand {
    kind: HandType,
    cards: [Card; 5],
    bid: u32,
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ", self.kind)?;
        self.cards.iter().try_for_each(|c| c.fmt(f))?;
        write!(f, " {}", self.bid)
    }
}

impl FromStr for Hand {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or(AoCError::new("malformed hand"))?;

        let cards: Vec<Card> = cards.chars().map(|c| c.try_into()).try_collect()?;
        let cards: [Card; 5] = cards
            .try_into()
            .map_err(|_| AoCError::new("Too many cards in hand"))?;

        Ok(Hand {
            kind: HandType::new_from_cards(&cards),
            cards,
            bid: bid.parse().map_err(AoCError::new_from_parseerror)?,
        })
    }
}

pub fn main(data: crate::DataIn) -> crate::AoCResult<String> {
    let mut ret = 0;
    let mut hands: Vec<Hand> = data.map(|line| line.parse()).try_collect().unwrap();
    hands.sort();
    for (rank, hand) in hands.iter().enumerate() {
        println!("{hand}");
        ret += (rank as u32 + 1) * hand.bid;
    }
    Ok(ret.to_string())
}

inventory::submit!(crate::AoCDay::mew("2023", "7", main));
