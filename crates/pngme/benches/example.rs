use std::path::PathBuf;

use pngme_lib;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench()]
fn decode() -> Result<(), pngme_lib::Error> {
    let path = PathBuf::from("../dice.png");
    let _ = pngme_lib::decode(&path, "RuSt")?;
    Ok(())
}

#[divan::bench(args = ["This is a secret message", ""])]
fn encode(msg: &str) -> Result<(), pngme_lib::Error> {
    let path = PathBuf::from("../dice.png");
    pngme_lib::encode(&path, "RuSt", msg.to_string())?;
    Ok(())
}

#[divan::bench()]
fn encode_large_string() -> Result<(), pngme_lib::Error> {
    let path = PathBuf::from("../dice.png");
    let message = String::from_utf8(vec![b'X'; 1000000]).unwrap();
    pngme_lib::encode(&path, "RuSt", message)?;
    Ok(())
}

#[divan::bench()]
fn print_chunks() -> Result<(), pngme_lib::Error> {
    let path = PathBuf::from("../dice.png");
    pngme_lib::print_chunks(&path)?;
    Ok(())
}

#[divan::bench()]
fn end_to_end() -> Result<(), pngme_lib::Error> {
    let path = PathBuf::from("../dice.png");
    let chunk_type = "RuSt".to_string();
    let msg = "TEST".to_string();
    pngme_lib::encode(&path, &chunk_type, msg)?;
    assert_eq!(pngme_lib::decode(&path, &chunk_type)?, "TEST");
    Ok(())
}
