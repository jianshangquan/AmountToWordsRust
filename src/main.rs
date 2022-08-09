use crate::AmountToWords::AmountConversion;

fn main() {
    let amt:u128 = 123456789;
    let result = AmountToWords::convert(&amt, AmountConversion::Burmese);
    print!("{}", result);
}





mod AmountToWords{

    pub enum AmountConversion{
        Burmese,
        Chinese,
        English
    }

    pub fn convert(value: &u128, conversion: AmountConversion) -> String{
        return match conversion {
            AmountConversion::Burmese => {
                BurmeseAmountConverter::convert(value)
            },
            AmountConversion::English => {
                String::from("")
            },
            AmountConversion::Chinese => {
                String::from("")
            }
        }
    }


    mod BurmeseAmountConverter{

        const unit: [&str; 8] = ["ခု", "ဆယ်", "ရာ", "ထောင်", "သောင်း", "သိန်း", "သန်း", "ကုဍ"];
        const numbers: [&str; 10] = ["၁", "၂", "၃", "၄", "၅", "၆", "၇", "၈", "၉", "၀"];
        const words: [&str; 10] = ["တစ်", "နှစ်", "သုံး", "လေး", "ငါး", "ခြှောက်", "ခုနှစ်", "ရှစ်", "ကိုး", "သုံည"];

        pub fn convert(amount: &u128) -> String{
            let amtString: String = amount.to_string();
            let firstTheinAmt: String = if amtString.len() > 5 { amtString.chars().skip(0).take(amtString.len() - 5).collect() } else {String::from("")};
            let lastTheinAmt: String = amtString.chars().skip(if amtString.len() < 6 {0} else {amtString.len() - 6}).collect();
            let mut result : String = String::from("");

            result = convertThein(lastTheinAmt);
            if firstTheinAmt.len() != 0 {
                let mut temp = convertThein(firstTheinAmt);
                temp.push_str(result.as_str());
                result = temp;
            }
            // result = result + ((lastTheinAmt.charAt(lastTheinAmt.length() - 1) != '0') ? lastTheinAmt.charAt(lastTheinAmt.length() - 1) : "") + "ကျပ်";
            // TODO: convert above java code to rust

            return result;
        }

        pub fn convertInLetter(value: &u128) -> String{
            let mut result: String = String::from(convert(value));
            for i in 0..10{
                result = result.replace(i.to_string().as_str(), words[i]);
            }
            return result;
        }

        fn convertThein(theinAmt: String) -> String{
            let mut result : String = String::from("");
            for i in 0..theinAmt.len() {
                let c = theinAmt.chars().nth(i);
                match c{
                    None => {}
                    Some(char) => {
                        result.push(char);
                        result.push_str(unit[theinAmt.len() - i - 1]);
                        if i == theinAmt.len() - 2 && char != '0'{
                            result.push_str("့");
                        }
                    }
                }
            }
            return result;
        }
    }


    mod ChineseAmountConverter{

    }



    mod EnglishAmountConverter{

    }
}








