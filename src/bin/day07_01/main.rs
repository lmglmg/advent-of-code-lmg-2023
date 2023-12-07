fn main() {
    let mut hands_and_bids: Vec<(Hand, i64)> = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .map(|s| {
            let (hand, bid) = s.split_once(' ').unwrap();
            (Hand::from(hand), bid.parse().unwrap())
        })
        .collect();

    hands_and_bids.sort_by_key(|(h, _)| *h);

    let total_score = hands_and_bids
        .iter()
        .rev()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as i64 * *bid )
        .sum::<i64>();

    println!("{}", total_score);
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
struct Card(u8);

#[derive(Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
enum HandStrength {
    FiveOfAKind = 0,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Hand {
    card_counts: [(Card, u8); 5],
    original_order: [Card; 5],
}

impl Card {
    const MAX_STRENGTH: Self = Self::from_char('A');

    const fn from_char(c: char) -> Self {
        match c {
            'J' => Self(0),
            '2'..='9' => Self(c as u8 - b'2' + 1u8),
            'T' => Self(9),
            'Q' => Self(10),
            'K' => Self(11),
            'A' => Self(12),
            _ => panic!(),
        }
    }

    pub fn new(value: u8) -> Self {
        assert!(value <= Self::MAX_STRENGTH.0);
        Self(value)
    }
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        Self::from_char(c)
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        assert!(s.len() == 5);

        let mut sorted_cards: [Card; 5] = [Card::new(0); 5];
        for (i, c) in s.chars().enumerate() {
            sorted_cards[i] = Card::from(c);
        }

        let mut card_counts = [(Card::new(0), 0); Card::MAX_STRENGTH.0 as usize + 1];

        for i in 0..=Card::MAX_STRENGTH.0 {
            card_counts[i as usize].0 = Card::new(i);
        }

        let mut joker_count = 0;

        for card in sorted_cards.iter() {
            if card.0 == 0 {
                joker_count += 1;
            } else {
                card_counts[card.0 as usize].1 += 1;
            }
        }

        card_counts.sort_by(|(_, a_count), (_, b_count)| b_count.cmp(a_count));

        card_counts[0].1 += joker_count;

        Self { card_counts: card_counts[0..5].try_into().unwrap(), original_order: sorted_cards }
    }
}

impl Hand {
    fn strength(&self) -> HandStrength {
        match self.card_counts {
            [(_, 5), ..         ] => HandStrength::FiveOfAKind,
            [(_, 4), ..         ] => HandStrength::FourOfAKind,
            [(_, 3), (_, 2), .. ] => HandStrength::FullHouse,
            [(_, 3), ..         ] => HandStrength::ThreeOfAKind,
            [(_, 2), (_, 2), .. ] => HandStrength::TwoPairs,
            [(_, 2), ..         ] => HandStrength::OnePair,
            [(_, 1), ..         ] => HandStrength::HighCard,
            _ => unreachable!(),
        }
    }

    fn compare_strength(&self, other: &Self) -> std::cmp::Ordering {
        let hand_strength =  self.strength().cmp(&other.strength());
        if hand_strength != std::cmp::Ordering::Equal {
            hand_strength
        } else {
            other.original_order.cmp(&self.original_order)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.compare_strength(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.compare_strength(other)
    }
}
