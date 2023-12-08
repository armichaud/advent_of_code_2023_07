use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn to_ordinal(&self) -> i32 {
        match self {
            HandType::HighCard => 0,
            HandType::Pair => 1,
            HandType::TwoPair => 2,
            HandType::ThreeOfAKind => 3,
            HandType::FullHouse => 4,
            HandType::FourOfAKind => 5,
            HandType::FiveOfAKind => 6,
        }
    }
}

enum CardRank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack, 
    Queen,
    King,
    Ace,
}

impl CardRank {
    fn from_str(s: &str) -> CardRank {
        match s {
            "2" => CardRank::Two,
            "3" => CardRank::Three,
            "4" => CardRank::Four,
            "5" => CardRank::Five,
            "6" => CardRank::Six,
            "7" => CardRank::Seven,
            "8" => CardRank::Eight,
            "9" => CardRank::Nine,
            "T" => CardRank::Ten,
            "J" => CardRank::Jack,
            "Q" => CardRank::Queen,
            "K" => CardRank::King,
            "A" => CardRank::Ace,
            _ => panic!("Invalid card rank"),
        }
    }

    fn to_ordinal(&self) -> i32 {
        match self {
            CardRank::Two => 0,
            CardRank::Three => 1,
            CardRank::Four => 2,
            CardRank::Five => 3,
            CardRank::Six => 4,
            CardRank::Seven => 5,
            CardRank::Eight => 6,
            CardRank::Nine => 7,
            CardRank::Ten => 8,
            CardRank::Jack => 9,
            CardRank::Queen => 10,
            CardRank::King => 11,
            CardRank::Ace => 12,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<String>,
    cards_mapped: HashMap<String, i32>,
    bid: i32,
    cards_sorted: Vec<String>,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: Vec<String>, bid: i32) -> Hand {
        let mut cards_mapped = HashMap::new();

        cards.iter().for_each(|x| {
            *cards_mapped.entry(x.to_string()).or_insert(0) += 1;
        });
        let mut hand = Hand { cards, cards_mapped, bid, cards_sorted: Vec::new(), hand_type: HandType::HighCard };
        hand.sort_cards();
        hand.get_hand_type();
        hand
    }

    fn sort_cards(&mut self) {
        // sort first by how many times the card appears, then by the card rank
        self.cards_sorted = self.cards.clone();
        self.cards_sorted.sort_by(|a, b| {
            let a_count = self.cards_mapped.get(a).unwrap();
            let b_count = self.cards_mapped.get(b).unwrap();
            if a_count == b_count {
                let a_rank = CardRank::from_str(a);
                let b_rank = CardRank::from_str(b);
                a_rank.to_ordinal().cmp(&b_rank.to_ordinal())
            } else {
                a_count.cmp(b_count)
            }
        });
        self.cards_sorted.reverse();
    }

    fn get_hand_type(&mut self) {
        self.hand_type = if self.is_five_of_a_kind() {
                HandType::FiveOfAKind
            } else if self.is_four_of_a_kind() {
                HandType::FourOfAKind
            } else if self.is_full_house() {
                HandType::FullHouse
            } else if self.is_three_of_a_kind() {
                HandType::ThreeOfAKind
            } else if self.is_two_pair() {
                HandType::TwoPair
            } else if self.is_pair() {
                HandType::Pair
            } else {
                HandType::HighCard
            }
    }

    fn is_five_of_a_kind(&self) -> bool {
        self.cards_sorted.iter().all(|x| x.to_string() == self.cards_sorted[0])
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.cards_sorted[0..4].iter().all(|x| x.to_string() == self.cards_sorted[0])
    }

    fn is_full_house(&self) -> bool {
        self.cards_sorted[0..3].iter().all(|x| x.to_string() == self.cards_sorted[0]) && self.cards_sorted[3..5].iter().all(|x| x.to_string() == self.cards_sorted[3])
    }

    fn is_three_of_a_kind(&self) -> bool {
        self.cards_sorted[0..3].iter().all(|x| x.to_string() == self.cards_sorted[0])
    }

    fn is_two_pair(&self) -> bool {
        self.cards_sorted[0..2].iter().all(|x| x.to_string() == self.cards_sorted[0]) && self.cards_sorted[2..4].iter().all(|x| x.to_string() == self.cards_sorted[2])
    }

    fn is_pair(&self) -> bool {
        self.cards_sorted[0..2].iter().all(|x| x.to_string() == self.cards_sorted[0])
    }
}


fn get_hands(filename: &str) -> Vec<Hand> {
    read_to_string(filename).unwrap().lines().map(|line| {
        let mut line = line.split_whitespace();
        let cards = line.next().unwrap().chars().collect::<Vec<char>>().iter().map(|x| x.to_string()).collect::<Vec<String>>();
        
        let bid = line.next().unwrap().parse::<i32>().unwrap();
        Hand::new(cards, bid)
    }).collect::<Vec<Hand>>()
}

fn sort_hands(hands: &mut Vec<Hand>) {
    hands.sort_by(|a, b| {
        if a.hand_type.to_ordinal() == b.hand_type.to_ordinal() {
            for i in 0..a.cards.len() {
                let a_rank = CardRank::from_str(&a.cards[i]);
                let b_rank = CardRank::from_str(&b.cards[i]);
                if a_rank.to_ordinal() != b_rank.to_ordinal() {
                    return a_rank.to_ordinal().cmp(&b_rank.to_ordinal());
                }
            }
            // This should never happen
            return a.bid.cmp(&b.bid);
        } else {
            a.hand_type.to_ordinal().cmp(&b.hand_type.to_ordinal())
        }
    });
}

fn part_1(filename: &str) -> usize {
    let mut hands = get_hands(filename);
    sort_hands(&mut hands);
    let mut sum: usize = 0;
    for (i, hand) in hands.iter().enumerate() {
        let winnings = hand.bid as usize * (i + 1);
        // println!("Hand: {:?}, Winnings: {}, i: {}", hand, winnings, i);
        sum += winnings;
    }
    sum
}

fn main() {
    assert_eq!(part_1("example.txt"), 6440);
    println!("Part 1 Solution: {}", part_1("input.txt"));
    // assert_eq!(part_2("example.txt"), 0);
    // println!("Part 2 Solution: {}", part_1("input.txt"));
}
