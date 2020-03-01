use std::fs;

fn transcribe(ch: char) -> char {
    match ch {
        'T' => 'U',
        _ => ch,
    }
}

pub fn solve() -> std::io::Result<()> {
    let input = fs::read_to_string("inputs/rosalind_rna.txt")?;
    let output = input.chars().map(transcribe).collect::<String>();
    println!("{}", output);
    Ok(())
}

#[test]
fn sample() {
    let input = "GATGGAACTTGACTACGTAAATT".to_string();
    let output = input.chars().map(transcribe).collect::<String>();
    assert_eq!(&output, "GAUGGAACUUGACUACGUAAAUU");
}
