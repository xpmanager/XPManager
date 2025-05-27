use crate::dblib;
use super::{
    terminal,
    Table,
    Style,
    Modify,
    Rows,
    Alignment,
    Columns,
    Width
};

/// Display logs in a table format.
/// 
/// ### Example:
/// ```
/// let logs = vec![
///     dblib::log::LogInfoForamt {
///         id: 32,
///         log: "create pass123".to_string(),
///         create_at: "2025-05-24 15:59:12".to_string()
///     }
/// ];
/// displaylib::log::display(logs);
/// ```
pub fn display(logs: Vec<dblib::log::LogInfoForamt>) {
    let (width, _) = terminal::size().unwrap_or((80, 0));
    let max_col_width = (width as f32 * 0.7 ) as usize;
    println!(
        "{}",
        Table::new(logs)
            .with(Style::rounded())
            .with( // First Row: Set the alignment to center.
                Modify::new(
                    Rows::single(0)
                ).with(Alignment::center())
            )
            .with( // Logs Column: Set the max based on terminal width.
                Modify::new(
                    Columns::single(1)
                ).with(
                    Width::wrap(max_col_width)
                )
            )
    );
}