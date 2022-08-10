mod amount_to_words;

use amount_to_words::AmountToWords;
use amount_to_words::AmountConversion;


fn main() {
    let amt:u128 = 1234567899;
    let result = AmountToWords::convertTo(&12346500000, AmountConversion::Burmese);
    print!("{}", result);
}














