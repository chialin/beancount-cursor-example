use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Transaction {
    date: String,
    payee: String,
    narration: String,
    postings: Vec<Posting>,
}

#[wasm_bindgen]
pub struct Posting {
    account: String,
    amount: f64,
    currency: String,
}

#[wasm_bindgen]
impl Transaction {
    #[wasm_bindgen(constructor)]
    pub fn new(date: String, payee: String, narration: String) -> Transaction {
        Transaction {
            date,
            payee,
            narration,
            postings: Vec::new(),
        }
    }

    pub fn add_posting(&mut self, account: String, amount: f64, currency: String) {
        self.postings.push(Posting { account, amount, currency });
    }
}

#[wasm_bindgen]
pub fn generate_beancount(transaction: &Transaction) -> String {
    let mut output = format!("{} * \"{}\" \"{}\"\n", 
        transaction.date,
        transaction.payee,
        transaction.narration
    );
    
    for posting in &transaction.postings {
        output.push_str(&format!("  {:<40} {:.2} {}\n", 
            posting.account,
            posting.amount,
            posting.currency
        ));
    }
    
    output
}