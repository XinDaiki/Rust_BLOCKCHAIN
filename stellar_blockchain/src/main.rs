use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
}

//  hash function (NOT real crypto)
fn calculate_hash(index: u32, timestamp: u128, data: &str, previous_hash: &str) -> String {
    format!("{}{}{}{}", index, timestamp, data, previous_hash)
}

// ⛏️ Create new block
fn create_block(index: u32, data: String, previous_hash: String) -> Block {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let hash = calculate_hash(index, timestamp, &data, &previous_hash);

    Block {
        index,
        timestamp,
        data,
        previous_hash,
        hash,
    }
}

// Blockchain structure
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    //new blockchain with genesis block
    fn new() -> Self {
        let genesis = create_block(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            chain: vec![genesis],
        }
    }

    // Add new block
    fn add_block(&mut self, data: String) {
        let last_block = self.chain.last().unwrap();

        let new_block = create_block(
            last_block.index + 1,
            data,
            last_block.hash.clone(),
        );

        self.chain.push(new_block);
    }

    // Display blockchain
    fn print_chain(&self) {
        for block in &self.chain {
            println!("{:#?}", block);
        }
    }
}

// 🚀 Main function
fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("First transaction".to_string());
    blockchain.add_block("Second transaction".to_string());

    blockchain.print_chain();
}