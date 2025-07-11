use anyhow::*;

mod utils;

use utils::rename_files_with_prefix;

fn main() -> Result<()> {
    rename_files_with_prefix(r#"C:\repos\another-css-sprite\images\classes"#, r"^\d+$", "class_{}")?;

    Ok(())
}