extern crate stem;
extern crate time;

use stem::*;
use time::precise_time_s;


#[test]
fn test_stem_program() {
  let input_fasta_file_path = "assets/sampled_trnas.fa";
  let args = ["-i", input_fasta_file_path, "-o", "assets/sampled_trnas"];
  let begin = precise_time_s();
  run_command("target/release/stem", &args, "Failed to run the STEM program.");
  let elapsed_time = precise_time_s() - begin;
  println!("The elapsed time for computing the STAPMT of each pair of RNA sequences in the FASTA file \"{}\" = {} [s].", input_fasta_file_path, elapsed_time);
}