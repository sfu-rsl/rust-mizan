use std::io::Cursor;
mod bite;
pub fn main() {
    println!("hello, from main!");

    big();
    little();
}

fn big() {
    use {bite::BigEndian, bite::BiteReadExpandedExt};

    let mut reader = Cursor::new(vec![2, 5, 3, 0]);
    assert_eq!(517, reader.read_u16::<BigEndian>().unwrap());
    assert_eq!(768, reader.read_u16::<BigEndian>().unwrap());

    let mut reader = Cursor::new(vec![2, 5, 3, 0]);
    println!(
        "verify: [big] 517 == {}",
        reader.read_u16::<BigEndian>().unwrap()
    );
    println!(
        "verify: [big] 768 == {}",
        reader.read_u16::<BigEndian>().unwrap()
    );
}

fn little() {
    use bite::BiteReadExt;

    let mut reader = Cursor::new(vec![5, 2, 0, 3]);
    assert_eq!(517, reader.read_u16().unwrap());
    assert_eq!(768, reader.read_u16().unwrap());

    let mut reader = Cursor::new(vec![5, 2, 0, 3]);
    println!("verify: [little] 517 == {}", reader.read_u16().unwrap());
    println!("verify: [little] 768 == {}", reader.read_u16().unwrap());
}
