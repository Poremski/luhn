use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(help = "A digit sequence", parse(from_str))]
    digit_sequence: String,
}

fn main() {
    let args = Opts::from_args();
    let digit_sequence: String = args.digit_sequence;

    if !digit_sequence.chars().all(char::is_numeric) {
        eprintln!("Error: The digit sequence needs to be numeric");
        std::process::exit(1);
    }

    let valid_digit_sequence: bool = luhn::valid(&digit_sequence);
    let next_digit: char = luhn::checksum(digit_sequence.as_bytes()) as char;

    println!("Sequence:\t{}", digit_sequence);
    println!("Valid sequence:\t{}", valid_digit_sequence);
    println!("Next digit:\t{}", next_digit);
}
