#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}


fn main() {
    // list of 'suits' - 'hearts', 'spades', ..
    let suits = vec!["Hearts", "Spades", "Diamonds", "Clubs"];
    // list of 'values' - 'ace', 'two', 'three', ..
    let values = vec!["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];

    let cards = vec![];
    // double nested for loop 
    for suit in suits { 
        for value in values {
            let card = format!("{} of {}", value, suit)
            cards.push(card);
        }
    }
    let deck : Deck = Deck { cards: vec![] };
    println!("Here is your deck: {:?}", deck);
}
