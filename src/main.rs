use blockchainlib::*;

fn main() {
    let difficulty = 0x00ffffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![],
        }],
        difficulty,
    );

    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);
}
