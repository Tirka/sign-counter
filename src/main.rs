fn main() {
    
}

// Дана последовательность чисел.
// Вычислить количество смен знака в последовательности:
enum Sign { Unknown, Positive, Negative }

fn count_sign_change(xs: &[i32]) -> i32 {
    xs
        .into_iter()
        .fold((0, Sign::Unknown), |(counter, sign), next| {
            match (sign, next) {
                (Sign::Unknown, next) if  *next >= 0 => (counter, Sign::Positive),
                (Sign::Unknown, next) if  *next <  0 => (counter, Sign::Negative),
                (Sign::Positive, next) if *next <  0 => (counter + 1, Sign::Negative),
                (Sign::Negative, next) if *next >= 0 => (counter + 1, Sign::Positive),
                (sign, _) => (counter, sign)
            }
        })
        .0
}

#[test]
fn test_count_sign_change() {
    assert_eq!(count_sign_change(&[1, -5, 3, -23, 3, -3]), 5);
    assert_eq!(count_sign_change(&[0, 0, 0, 0]), 0);
}