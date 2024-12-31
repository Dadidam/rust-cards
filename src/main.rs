use core::num;

use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        // List of 'suits' - 'hearts', 'spades', etc.
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];

        // List of 'values' - 'ace', 'two', etc.
        let values = [
            "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
        ];

        let mut cards = vec![];

        // Double nested loop
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        // TODO: add error handling, e.g. check for deck size
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();
    println!("Here's your deck: {:#?}", deck);

    let cards = deck.deal(3);
    println!("Here's your hand: {:#?}", cards);
}
