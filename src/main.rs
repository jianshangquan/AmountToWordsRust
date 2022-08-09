fn main() {
    let amt:u32 = 123456789;
    let result = convert(&amt, AmountConversion::Burmese);
    print!("{}", result);
}


enum AmountConversion{
    Burmese,
    Chinese,
    English
}

fn convert(value: &u32, conversion: AmountConversion) -> &str{
    return match conversion {
        AmountConversion::Burmese => {
            BurmeseAmountConverter::convert(value)
        },
        AmountConversion::English => {
            ""
        },
        AmountConversion::Chinese => {
            ""
        }
    }
}




mod BurmeseAmountConverter{
    pub fn convert(value: &u32) -> &str{
        return "တစ်နှစ်သုံးလေးငါးခြောက်";
    }
}


mod ChineseAmountConverter{

}



mod EnglishAmountConverter{
    
}

