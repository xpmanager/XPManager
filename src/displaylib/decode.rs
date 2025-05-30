use super::Colorize;

/// Despley the decoded string.
/// 
/// ### Example:
/// ```
/// let decode = "XPManager";
/// displaylib::decode::display(decode);
/// ```
pub fn display(new: String) {
    println!(
        "{}:\n{}\n",
        "The decode".green(),
        new.green()
    )
}