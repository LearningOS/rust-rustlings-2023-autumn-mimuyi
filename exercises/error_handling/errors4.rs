/*
 * @Descripttion: 
 * @version: 
 * @Author: Muyi
 * @Date: 2023-10-18 15:38:23
 * @LastEditors: Muyi
 * @LastEditTime: 2023-10-18 15:38:24
 * @E-mail: Muyi_Mi@aliyun.com
 */
// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.


#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        if value > 0 {
            Ok(PositiveNonzeroInteger(value as u64))
        } else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Err(CreationError::Negative)
        }

    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
