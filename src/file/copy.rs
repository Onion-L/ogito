use color_eyre::eyre::Result;
use fs_extra::dir::{copy, CopyOptions};
use std::path::Path;

pub fn create_template<P: AsRef<Path>>(source: P, destination: P) -> Result<()> {
    let source_path = source.as_ref();
    let dest_path = destination.as_ref();

    if !source_path.exists() {
        return Err(color_eyre::eyre::eyre!(
            "Source path '{:?}' does not exist.",
            source_path
        ));
    }
    if !source_path.is_dir() {
        return Err(color_eyre::eyre::eyre!(
            "Source path '{:?}' is not a directory.",
            source_path
        ));
    }

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    copy(source_path, dest_path, &options)?;

    Ok(())
}
