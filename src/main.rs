use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    let digit_sequence: String = args.digit_sequence;
    let valid_digit_sequence: bool = luhn::valid(&digit_sequence);
    let next_digit: char = luhn::checksum(digit_sequence.as_bytes()) as char;

    println!("Sequence:\t{}", digit_sequence);
    println!("Valid sequence:\t{}", valid_digit_sequence);
    println!("Next digit:\t{}", next_digit);
}

#[derive(StructOpt)]
struct Cli {
    digit_sequence: String,
}
