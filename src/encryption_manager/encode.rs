use super::ArgMatches;
use crate::{
    displaylib, 
    loglib, 
    utilities,
    errorlib,
};

/// Encode string to XPManager v1.0 key style.
/// 
/// ### Example:
/// ```
/// let xpm_xpmv1 = encode::xpmv1("xpm", 9999);
/// assert_eq!(xpm_xpmv1, "0x124f08%$%0x111690%$%0x10a163");
/// ```
pub fn xpmv1(string: String, n: u16) -> String {
    const XPM_KEY_MARK: &str = "%$%";
    string
        .chars()
        .map(|i| format!("0x{:x}", i as u32 * n as u32))  
        .collect::<Vec<String>>()
        .join(XPM_KEY_MARK)
}

/// Encode string to hexadecimal.
/// 
/// ### Example:
/// ```
/// let xpm_hex = encode::hex("xpm");
/// assert_eq!(xpm_hex, "78 70 6D");
/// ```
pub fn hex(string: String) -> String {
    string
        .chars()
        .map(|i| format!("{:X}", i as u32) )
        .collect::<Vec<String>>()
        .join(" ")
}

/// Hash string as hexadecimal using simple operation.
/// 
/// ### Example:
/// ```
/// let xpm_hash = encode::hex_hash("xpm is the best!");
/// assert_eq!(xpm_hash, "155 DC 141 1CF");
/// ```
pub fn hex_hash(string: String) -> String {
    let mut hash = String::new();
    let mut num : u32 = 0;
    for i in string.chars() {
        if i == ' ' {
            hash += &format!(" {:X}", num);
            num = 0;
            continue;
        }
        num += i as u32;
    }
    if num != 0 {
        hash += &format!(" {:X}", num);
    }
    hash.trim().to_owned()
}

/// Encode string to binary.
/// 
/// ### Example:
/// ```
/// let xpm_bin = encode::bin("xpm");
/// assert_eq!(xpm_bin, "1111000 1110000 1101101");
/// ```
pub fn bin(string: String) -> String {
    string
        .chars()
        .map(|i| format!("{:b}", i as u32) )
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn main(command: &ArgMatches) {
    let mut logger = loglib::Logger::new("encode");
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
    } else if *command.get_one::<bool>("hex-hash").unwrap_or(&false) {
        _encoded_date = hex_hash(string.clone());
    } else {
        _encoded_date = hex(string.clone());
    };
    displaylib::encode::display(&_encoded_date);
    logger.info("string encoded successfully.");
}


#[cfg(test)]
mod tests {
    #[test]
    fn xpmv1() {
        let result = super::xpmv1("XPManager".to_string(), 2025);
        let expected = "0x2b818%$%0x278d0%$%0x26115%$%0x2ff49%$%0x3661e%$%0x2ff49%$%0x32ebf%$%0x31eed%$%0x385c2";
        assert_eq!(result, expected, "Encode value NOT match!!");
    }

    #[test]
    fn hex() { 
        let result = super::hex("XPManager".to_string());
        let expected = "58 50 4D 61 6E 61 67 65 72";
        assert_eq!(result, expected, "Encode value NOT match!!");
    }

    #[test]
    fn hex_hash() {
        let result = super::hex_hash("XPManager".to_string());
        let expected = "363";
        assert_eq!(result, expected, "Encode value NOT match!!");
    }

    #[test]
    fn bin() {
        let result = super::bin("XPManager".to_string());
        let expected = "1011000 1010000 1001101 1100001 1101110 1100001 1100111 1100101 1110010";
        assert_eq!(result, expected, "Encode value NOT match!!");
    }
}