use rand::{seq::SliceRandom, rng};

#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}


impl Deck {
    fn new() -> Self {
        // list of 'suits' - 'hearts', 'spades', ..
        let suits = vec!["Hearts", "Spades", "Diamonds", "Clubs"];
        // list of 'values' - 'ace', 'two', 'three', ..
        let values = vec!["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];


        let mut cards = vec![];
        
        // double nested for loop 
        for suit in suits {
            for value in &values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
        
    }
    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }
    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    
    deck.shuffle();

    // Probably need to add error handling!!!!
    let cards = deck.deal(3);

    println!("Heres your hand: {:#?}", cards);
    println!("Heres your deck: {:#?}", deck);
}
