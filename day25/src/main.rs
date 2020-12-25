fn find_loop(pk: i64, subject: i64, div: i64) -> usize {
    let mut value = 1;
    for ls in 1.. {
        value *= subject;
        value = value % div;
        if value == pk {
            return ls;
        }
    }
    0
}

fn find_enc(loop_size: usize, subject: i64, div: i64) -> i64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject;
        value = value % div;
    }
    value
}

fn main() {
    // real input
    let card_pk = 5290733;
    let door_pk = 15231938;
    // Example input
    //let card_pk = 5764801; 
    //let door_pk = 17807724;
    let card_loop = find_loop(card_pk, 7, 20201227);
    let door_loop = find_loop(door_pk, 7, 20201227);
    println!("Card loop: {}, Door loop: {}", card_loop, door_loop);
    let card_enc = find_enc(card_loop, door_pk, 20201227);
    let door_enc = find_enc(door_loop, card_pk, 20201227);
    println!("Card enc: {}, Door enc: {}", card_enc, door_enc);
}
