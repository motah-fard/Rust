#[derive(Debug)]

struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            // balance start at 0
            balance:0,
        }
    }
}

struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
       Bank { accounts: vec![]} 
    }
}

fn main() {
    let bank = Bank:: new();


    
    println!("{:#?}", bank);
}
