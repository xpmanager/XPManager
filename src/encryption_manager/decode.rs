use super::ArgMatches;
use crate::{
    displaylib, 
    loglib, 
    utilities,
    errorlib,
};

/// Decode XPManager v1.0 key style to string.
/// 
/// ### Example: 
/// ```
/// let xpm = decode::xpmv1("0x124f08%$%0x111690%$%0x10a163", 9999);
/// assert_eq!(xpm, "xpm");
/// ```
pub fn xpmv1(string: String, n: u16) -> String {
    const XPM_KEY_MARK: &str = "%$%";
    string
        .split(XPM_KEY_MARK)
        .map(|s| {
            let letters = u32::from_str_radix(
                &format!("{}", s.replace("0x", "")),
                16 
            ).unwrap() / n as u32;
            char::from_u32(letters).unwrap()
        })
        .collect::<String>()
}

/// Decode hexadecimal to string.
/// 
/// ### Example:
/// ```
/// let xpm = decode::hex("78 70 6D");
/// assert_eq!(xpm, "xpm");
/// ```
pub fn hex(string: String) -> String {
    string
        .split(" ")
        .map(|i| {
            char::from_u32(
                u32::from_str_radix(i, 16).unwrap()
            ).unwrap()
        })
        .collect::<String>()
}

/// Decode binary string.
/// 
/// ### Example:
/// ```
/// let xpm = decode::bin("1111000 1110000 1101101");
/// assert_eq!(xpm, "xpm");
/// ```
pub fn bin(string: String) -> String {
    string
        .split(" ")
        .map(|i| {
            char::from_u32(
                u32::from_str_radix(i, 2).unwrap()
            ).unwrap()
        })
        .collect::<String>()
}

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("decode");
    let string = utilities::input("Enter the string: ");
    logger.start();
    let mut _encoded_date = String::new();
    if *command.get_one::<bool>("xpmv1").unwrap_or(&false) {
        let constant_str = utilities::input("Enter the constant number: ");
        logger.start();
        if let Ok(constant) = constant_str.parse::<u16>() {
            if constant > 9999u16 || constant < 1000u16 {
                logger.error(
                    "constant must be in range (1000 <= x <= 9999)!",
                    errorlib::ExitErrorCode::Input
                )
            }
            _encoded_date = xpmv1(string.clone(), constant);
        } else {
            logger.error(
                "constant must be string!",
                errorlib::ExitErrorCode::Input
            )
        }
    } else if *command.get_one::<bool>("bin").unwrap_or(&false) {
        _encoded_date = bin(string.clone());
    } else {
        _encoded_date = hex(string.clone());
    };
    displaylib::encode::display(&_encoded_date);
    logger.info("string decoded successfully.");
}


#[cfg(test)]
mod tests {
    #[test]
    fn xpmv1() {
        let result = super::xpmv1(
            "0x2b818%$%0x278d0%$%0x26115%$%0x2ff49%$%0x3661e%$%0x2ff49%$%0x32ebf%$%0x31eed%$%0x385c2".to_string(), 
            2025
        );
        let expected = "XPManager";
        assert_eq!(result, expected, "Decoded value NOT match!!");
    }

    #[test]
    fn hex() { 
        let result = super::hex(
            "58 50 4D 61 6E 61 67 65 72".to_string()
        );
        let expected = "XPManager";
        assert_eq!(result, expected, "Decoded value NOT match!!");
    }

    #[test]
    fn bin() {
        let result = super::bin(
            "1011000 1010000 1001101 1100001 1101110 1100001 1100111 1100101 1110010".to_string()
        );
        let expected = "XPManager";
        assert_eq!(result, expected, "Decoded value NOT match!!");
    }
}