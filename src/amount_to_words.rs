pub mod AmountToWords{
    use crate::AmountConversion;

    pub fn convertTo(value: &u128, conversion: AmountConversion) -> String{
        return match conversion {
            AmountConversion::Burmese => {
                BurmeseAmountConverter::convertInLetter(value)
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
        const maxConvertableAmt: u128 = 10000000000000;

        fn convert(amount: &u128) -> String{
            if(checkValidAmount(amount)) {
                panic!("Convert burmese amount must be lower than maxConvertableAmt");
            }

            let amt_string: String = amount.to_string();
            let first_thein_amt: String = if amt_string.len() > 5 { amt_string.chars().skip(0).take(amt_string.len() - 5).collect() } else {String::from("")};
            let last_thein_amt: String = amt_string.chars().skip(if amt_string.len() < 6 {0} else { amt_string.len() - 6}).collect();
            let mut result : String = String::from("");

            result = convertThein(last_thein_amt);
            if first_thein_amt.len() != 0 {
                let mut temp = convertThein(first_thein_amt);
                // let len = last_thein_amt.len();
                // if len == 0 {temp.push_str("သိန်း")};
                temp.push_str(result.as_str());
                result = temp;
            }

            let last_digit: char = amt_string.chars().nth(amt_string.len() - 1).unwrap();
            if last_digit != '0' {
                result.push(last_digit)
            }

            result.push_str("ကျပ်");

            return result;
        }

        pub fn convertInLetter(value: &u128) -> String{
            let mut result: String = String::from(convert(value));
            for i in 1..10{
                result = result.replace(i.to_string().as_str(), words[i - 1]);
            }
            return result;
        }

        fn convertThein(thein_amt: String) -> String{
            let mut result : String = String::from("");
            for i in 0..thein_amt.len() - 1 {
                let c = thein_amt.chars().nth(i).unwrap() as char;
                if '0' != c {
                    result.push(c);
                    result.push_str(unit[thein_amt.len() - i - 1]);
                    if i == thein_amt.len() - 2 && thein_amt.chars().nth(i + 1).unwrap() as char != '0'{
                        result.push_str("့");
                    }
                }
            }
            return result;
        }

        fn checkValidAmount(value: &u128) -> bool{
            return value >= &maxConvertableAmt;
        }
    }


    mod ChineseAmountConverter{

    }



    mod EnglishAmountConverter{

    }
}
pub enum AmountConversion{
    Burmese,
    Chinese,
    English
}