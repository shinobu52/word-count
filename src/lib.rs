//! word-countはシンプルな文字、単語、行の出現頻度の計数機能を提供します
//! 詳しくは[`count`](fn.count.html)関数のドキュメントを見てください

#![warn(missing_docs)]
use regex::Regex;
use std::collections::HashMap;
use std::hash::Hash;
use std::io::BufRead;

/// [`count`](fn.count.html)で使うオプション
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    /// 文字ごとに頻度を数える
    Char,
    /// 単語ごとに頻度を数える
    Word,
    /// 行ごとに頻度を数える
    Line,
}

/// オプションのデフォルトは[`Word`](enum.CountOption.html#variant.Word)
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// inputから1行ずつUTF-8文字列を読み込む、頻度を数える
///
/// 頻度を数える対象はオプションによって制御される
///
/// *[`CountOption::Char`](enum.CountOption.html#variant.Char): Unicodeの1文字ごと<br>
/// *[`CountOption::Word`](enum.CountOption.html#variant.Word): 正規表現 \w+ にマッチする単語ごと<br>
/// *[`CountOption::Line`](enum.CountOption.html#variant.Line): \n または \r\n で区切られた1行ごと<br>
///
/// # Panics
///
/// 入力が UTF-8 でフォーマットされていない場合にパニックする
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new(); // HashMap<String, usize> 型

    for line in input.lines() {
        use crate::CountOption::*;

        let line = line.unwrap();
        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1
                }
            },
            Word => {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1
                }
            },
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }
    freqs
}

#[test]
fn word_count_works() {
    use std::io::Cursor;

    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("bb".to_string(), 2);
    exp.insert("cc".to_string(), 1);

    assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp)
}

#[test]
fn word_count_works2() {
    use std::io::Cursor;

    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("cc".to_string(), 1);
    exp.insert("dd".to_string(), 1);

    assert_eq!(count(Cursor::new("aa cc dd"), CountOption::Word), exp)
}

#[test]
#[should_panic]
fn word_count_do_not_contain_unknown_words() {
    use std::io::Cursor;

    count(
        Cursor::new([
            b'a', // a
            0xf0, 0x90, 0x80, // でたらめなバイト列
            0xe3, 0x81, 0x82, // あ
        ]),
        CountOption::Word,
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    // Map の複数のKey-Value ペアに対して assert をするマクロを定義しておく
    macro_rules! assert_map {
        ($expr: expr, {$($key: expr => $value:expr), *}) => {
            $(assert_eq!($expr[$key], $value));*
        };
    }

    #[test]
    fn word_count_works3() {
        let freqs = count(Cursor::new("aa cc dd"), CountOption::Word);

        assert_eq!(freqs.len(), 3);
        // 定義したマクロを使ってテストを書く
        assert_map!(freqs, {"aa" => 1, "cc" => 1, "dd" => 1});
        // 展開すると概ね以下のようなコードになる
        // ```
        // assert_eq!(freqs["aa"], 1);
        // assert_eq!(freqs["cc"], 1);
        // assert_eq!(freqs["dd"], 1);
        // ```
    }
}
