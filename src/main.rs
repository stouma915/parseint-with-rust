use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("{}", parse_int("256".to_string()).unwrap());
}

fn parse_int(string: String) -> Option<i32> {
    if string.is_empty() {
        return None;
    }

    let is_negative: bool = string.starts_with("-");
    let unsigned_string: &String = &string.replacen("+", "", 1).replacen("-", "", 1);

    return if unsigned_string.graphemes(true).count() == 1 {
        let number: i32 = match unsigned_string.as_str() {
            "0" => 0,
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            _ => return None,
        };
        let signed_number: i32 = if is_negative { -number } else { number };

        Some(signed_number)
    } else {
        let mut number: i32 = 0;
        let mut index: u32 = 0;
        let mut error: bool = false;

        for char in unsigned_string.chars().rev() {
            let num: i32 = parse_int(char.to_string()).unwrap_or_else(|| {
                error = true;
                0
            });
            if error {
                break;
            }

            number += num * 10_i32.pow(index);
            index += 1;
        }

        let signed_number: i32 = if is_negative { -number } else { number };

        if error {
            None
        } else {
            Some(signed_number)
        }
    };
}
