use mpl_token_metadata::state::{Creator, Edition, Key};
use solana_sdk::pubkey::Pubkey;
use std::fs::File;

fn main() {
    write();

    read();
}

fn write() {
    let creator = Creator {
        address: Pubkey::new_unique(),
        verified: false,
        share: 50,
    };
    let edition = Edition {
        key: Key::EditionV1,
        parent: Pubkey::new_unique(),
        edition: 1,
    };

    let e = File::create("edition.json").unwrap();
    let c = File::create("creator.json").unwrap();

    serde_json::to_writer(c, &creator).unwrap();
    serde_json::to_writer(e, &edition).unwrap();
}

fn read() {
    let c = File::open("creator.json").unwrap();
    let e = File::open("edition.json").unwrap();

    let creator: Creator = serde_json::from_reader(c).unwrap();
    let edition: Edition = serde_json::from_reader(e).unwrap();

    println!("{:?}", creator);
    println!("{:?}", edition);
}
