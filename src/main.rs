fn main() {
    let amt:i32 = 123456789;
    let result = convert(&amt, AmountConversion::Burmese);
    print!("{}", result);
}


enum AmountConversion{
    Burmese,
    Chinese,
    English
}

fn convert(value: &i32, conversion: AmountConversion) -> &str{
    match conversion{
        AmountConversion::Burmese => {
            return BurmeseAmountConverter::convert(value);
        },
        AmountConversion::English => {
            return "";
        },
        AmountConversion::Chinese => {  
            return "";
        }
    }
}




mod BurmeseAmountConverter{
    pub fn convert(value: &i32) -> &str{
        return "တစ်နှစ်သုံးလေးငါးခြောက်";
    }
}


mod ChineseAmountConverter{

}



mod EnglishAmountConverter{
    
}

