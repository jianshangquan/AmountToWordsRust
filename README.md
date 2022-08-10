# AmountToWordsRust
Convert amount in letter
> Note : Currently it support only burmese

```rust
  AmountToWords::convert(&1234567899, AmountConversion::Burmese);
  // Result -> တစ်သောင်းနှစ်ထောင်သုံးရာလေးဆယ့်ငါးသိန်းခြှောက်သောင်းခုနှစ်ထောင်ရှစ်ရာကိုးဆယ့်ကိုးကျပ်
```

#### Max convertable amount in different language
| Language | Max convertable |
|--|--|
| Burmese | < 10000000000000 |

### Ported languages
Java: https://github.com/jianshangquan/AmountToWordsJava \
Javascript: https://github.com/jianshangquan/AmountToWordsJavascript \
Dart: https://github.com/jianshangquan/AmountToWordsDart \
Rust: https://github.com/jianshangquan/AmountToWordsRust
