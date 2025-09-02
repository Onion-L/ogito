use color_eyre::Result;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn update_package_json_name<P: AsRef<Path>>(path: P, new_name: &str) -> Result<()> {
    let path = path.as_ref();
    let content = fs::read_to_string(path)?;
    let mut package_json: Value = serde_json::from_str(&content)?;

    if let Some(obj) = package_json.as_object_mut() {
        obj.insert("name".to_string(), Value::String(new_name.to_string()));
    }

    let updated_content = serde_json::to_string_pretty(&package_json)?;
    fs::write(path, updated_content)?;

    Ok(())
}

/// Recursively finds and updates package.json files in a directory
pub fn update_package_json_in_dir<P: AsRef<Path>>(dir_path: P, new_name: &str) -> Result<()> {
    let dir_path = dir_path.as_ref();

    if !dir_path.exists() || !dir_path.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively search in subdirectories
            update_package_json_in_dir(&path, new_name)?;
        } else if path.file_name().and_then(|n| n.to_str()) == Some("package.json") {
            // Found package.json, update it
            update_package_json_name(&path, new_name)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_update_package_json_name() -> Result<()> {
        let temp_dir = tempdir()?;
        let package_json_path = temp_dir.path().join("package.json");

        let test_json = r#"
        {
            "name": "old-name",
            "version": "1.0.0",
            "description": "Test package"
        }
        "#;

        fs::write(&package_json_path, test_json)?;

        update_package_json_name(&package_json_path, "new-project-name")?;

        let updated_content = fs::read_to_string(&package_json_path)?;
        let package_json: Value = serde_json::from_str(&updated_content)?;

        assert_eq!(package_json["name"], "new-project-name");
        assert_eq!(package_json["version"], "1.0.0");
        assert_eq!(package_json["description"], "Test package");

        Ok(())
    }

    #[test]
    fn test_update_package_json_in_dir() -> Result<()> {
        let temp_dir = tempdir()?;
        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir)?;

        let root_package = temp_dir.path().join("package.json");
        let sub_package = sub_dir.join("package.json");

        let test_json = r#"
        {
            "name": "old-name",
            "version": "1.0.0"
        }
        "#;

        fs::write(&root_package, test_json)?;
        fs::write(&sub_package, test_json)?;

        update_package_json_in_dir(temp_dir.path(), "updated-project")?;

        let root_content = fs::read_to_string(&root_package)?;
        let sub_content = fs::read_to_string(&sub_package)?;

        let root_json: Value = serde_json::from_str(&root_content)?;
        let sub_json: Value = serde_json::from_str(&sub_content)?;

        assert_eq!(root_json["name"], "updated-project");
        assert_eq!(sub_json["name"], "updated-project");

        Ok(())
    }
}
