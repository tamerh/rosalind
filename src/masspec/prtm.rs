use crate::utils::Monoisotopic;
use std::fs;

pub fn solve() -> std::io::Result<()> {
  let table = Monoisotopic::default();
  let f = fs::read_to_string("inputs/rosalind_prtm.txt")?;
  println!("{}", table.peptide_mass(f.trim()));
  Ok(())
}

#[test]
fn sample() {
  let input = "SKADYEK";
  let e = 0.001;
  assert!((Monoisotopic::default().peptide_mass(input) - 821.392).abs() <= e);
}
