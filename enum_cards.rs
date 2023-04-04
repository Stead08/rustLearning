enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

enum Card {
    Joker,

    King(Suit),
    Queen(Suit),
    Jack(Suit),
    Ace(Suit),

    Pip {
        suit: Suit,
        rank: u8,
    }
}

impl Suit {
    fn print_suit(self) -> String {
        match self {
            Suit::Clubs => "Clubs".to_string(),
            Suit::Spades => "Spades".to_string(),
            Suit::Diamonds => "Diamonds".to_string(),
            Suit::Hearts => "Hearts".to_string(),
        }
    }
}


impl Card {
    fn print_card_info(self) {
        match self {
            Card::Joker => println!("Joker"),
            Card::King(suit) => println!("King [{}]", suit.print_suit()),
            Card::Queen(suit) => println!("Queen [{}]", suit.print_suit()),
            Card::Jack(suit) => println!("Jack [{}]", suit.print_suit()),
            Card::Ace(suit) => println!("Ace [{}]", suit.print_suit()),
            Card::Pip { suit, rank } => println!("{} [{}]", rank, suit.print_suit()),
        }
    }
}

fn main() {
    let king_heart = Card::King(Suit::Hearts);
    king_heart.print_card_info();
    let dia3 = Card::Pip { suit: Suit::Diamonds, rank: 3 };
    dia3.print_card_info();
}
