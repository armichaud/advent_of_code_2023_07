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
    JackOrJoker, 
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
            "J" => CardRank::JackOrJoker,
            "Q" => CardRank::Queen,
            "K" => CardRank::King,
            "A" => CardRank::Ace,
            _ => panic!("Invalid card rank: {}", s),
        }
    }

    fn to_ordinal(&self, wildcards_enabled: bool) -> i32 {
        match self {
            CardRank::Two => 1,
            CardRank::Three => 2,
            CardRank::Four => 3,
            CardRank::Five => 4,
            CardRank::Six => 5,
            CardRank::Seven => 6,
            CardRank::Eight => 7,
            CardRank::Nine => 8,
            CardRank::Ten => 9,
            CardRank::JackOrJoker => if wildcards_enabled { 0 } else { 10 },
            CardRank::Queen => 11,
            CardRank::King => 12,
            CardRank::Ace => 13,
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<String>, // original card labels
    cards_mapped: HashMap<String, i32>, // card labels mapped to count
    bid: i32,
    cards_sorted: Vec<String>, // cards_for_sorting, sorted by count, then rank
    hand_type: HandType,
    cards_for_sorting: Vec<String>, // cards, but with wildcard substitutes when jokers are enabled
}

impl Hand {
    fn new(cards: Vec<String>, bid: i32, wildcards_enabled: bool) -> Hand {
        let mut cards_mapped = HashMap::new();

        cards.iter().for_each(|x| {
            *cards_mapped.entry(x.to_string()).or_insert(0) += 1;
        });
        let mut cards_for_sorting = cards.clone();
        if wildcards_enabled {
            let mut max_count = 0;
            let mut max_card = String::from("A");
            for (card, count) in cards_mapped.iter() {
                if *count > max_count && card != "J" {
                    max_count = *count;
                    max_card = card.to_string();
                }
            }
            *cards_mapped.entry(max_card.clone()).or_insert(0) += max_count;
            cards_mapped.remove("J");

            cards_for_sorting.iter_mut().for_each(|x| {
                if *x == "J" {
                    *x = max_card.clone();
                }
            });
        }
        let mut hand = Hand { cards, cards_mapped, bid, cards_sorted: Vec::new(), hand_type: HandType::HighCard, cards_for_sorting };
        hand.sort_cards();
        hand.get_hand_type();
        hand
    }

    fn sort_cards(&mut self) {
        self.cards_sorted = self.cards_for_sorting.clone();
        self.cards_sorted.sort_by(|a, b| {
            let a_count = self.cards_mapped.get(a).unwrap();
            let b_count = self.cards_mapped.get(b).unwrap();
            if a_count == b_count {
                let a_rank = CardRank::from_str(a);
                let b_rank = CardRank::from_str(b);
                // Jokers can always be false here because we remove any Js from the map when wildcards are enabled
                a_rank.to_ordinal(false).cmp(&b_rank.to_ordinal(false))
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


fn get_hands(filename: &str, wildcards_enabled: bool) -> Vec<Hand> {
    read_to_string(filename).unwrap().lines().map(|line| {
        let mut line = line.split_whitespace();
        let cards = line.next().unwrap().chars().collect::<Vec<char>>().iter().map(|x| x.to_string()).collect::<Vec<String>>();
        
        let bid = line.next().unwrap().parse::<i32>().unwrap();
        Hand::new(cards, bid, wildcards_enabled)
    }).collect::<Vec<Hand>>()
}

fn sort_hands(hands: &mut Vec<Hand>, wildcards_enabled: bool) {
    hands.sort_by(|a, b| {
        if a.hand_type.to_ordinal() == b.hand_type.to_ordinal() {
            for i in 0..a.cards.len() {
                let a_rank = CardRank::from_str(&a.cards[i]);
                let b_rank = CardRank::from_str(&b.cards[i]);
                if a_rank.to_ordinal(wildcards_enabled) != b_rank.to_ordinal(wildcards_enabled) {
                    return a_rank.to_ordinal(wildcards_enabled).cmp(&b_rank.to_ordinal(wildcards_enabled));
                }
            }
            // This should never happen but otherwise Rust would require some kind of return value here
            panic!("Hands are equal: {:?}, {:?}", a, b);
        } else {
            a.hand_type.to_ordinal().cmp(&b.hand_type.to_ordinal())
        }
    });
}

fn solution(filename: &str, wildcards_enabled: bool) -> usize {
    let mut hands = get_hands(filename, wildcards_enabled);
    sort_hands(&mut hands, wildcards_enabled);
    let mut sum: usize = 0;
    for (i, hand) in hands.iter().enumerate() {
        let winnings = hand.bid as usize * (i + 1);
        sum += winnings;
    }
    sum
}

fn part_1(filename: &str) -> usize {
    solution(filename, false)
}

fn part_2(filename: &str) -> usize {
    solution(filename, true)
}

fn main() {
    println!("{}", part_1("example.txt"));
    println!("{}", part_1("input.txt"));
    println!("{}", part_2("example.txt"));
    println!("{}", part_2("input.txt"));
}
