use super::Colorize;

/// Display the key.
/// 
/// ### Example:
/// ```
/// let key = "My super key!".to_string();
/// displaylib::key::display(key);
/// ```
pub fn display(key: String) {
    println!(
        "\n{} {}\n",
        "Your Key:".blue(),
        key.green()
    );
}