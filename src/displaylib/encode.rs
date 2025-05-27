use super::Colorize;

/// Despley the encoded string.
/// 
/// ### Example:
/// ```
/// let encode = "1111000 1110000 1101101";
/// displaylib::encode::display(encode);
/// ```
pub fn display(new: &str) {
    println!(
        "{}:\n{}\n",
        "The encode".green(),
        new.green()
    )
}