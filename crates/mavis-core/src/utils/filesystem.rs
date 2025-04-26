// Filesystem utility functions for MAVIS

use crate::error::CoreError;
use log::{debug, info};
use std::{env, fs, path::{Path, PathBuf}};

/// Ensures all required local directories exist for MAVIS
pub fn ensure_local_dirs() -> Result<(), CoreError> {
    let local_app_data = get_local_app_data()?;
    let mavis_dir = local_app_data.join("MAVIS");
    
    // Create main directories
    let directories = [
        mavis_dir.join("config"),
        mavis_dir.join("themes"),
        mavis_dir.join("logs"),
        mavis_dir.join("cache"),
        mavis_dir.join("plugins"),
    ];
    
    for dir in &directories {
        if !dir.exists() {
            debug!("Creating directory: {:?}", dir);
            fs::create_dir_all(dir)?;
        }
    }
    
    info!("Local directories created at {:?}", mavis_dir);
    Ok(())
}

/// Gets the local app data directory path
pub fn get_local_app_data() -> Result<PathBuf, CoreError> {
    let local_app_data = env::var("LOCALAPPDATA")
        .map_err(|_| CoreError::IoError(std::io::Error::new(std::io::ErrorKind::NotFound, "LOCALAPPDATA environment variable not found")))?; // Wrap error
        
    Ok(PathBuf::from(local_app_data))
}

/// Copies a directory recursively
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), CoreError> {
    fs::create_dir_all(&dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let target = dst.as_ref().join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_all(entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target)?;
        }
    }
    
    Ok(())
}

// Removed copy_default_config function.
// This logic is now handled more specifically by ConfigLoader::copy_default_lua_files,
// which copies individual template files if they are missing, rather than the whole directory.

/// Copies default themes to user's local app data directory
pub fn copy_default_themes(install_dir: impl AsRef<Path>) -> Result<(), CoreError> {
    let local_app_data = get_local_app_data()?;
    let source_themes = install_dir.as_ref().join("assets").join("themes");
    let dest_themes = local_app_data.join("MAVIS").join("themes");
    
    // Only copy if the destination doesn't exist or is empty
    if !dest_themes.exists() || fs::read_dir(&dest_themes)?.next().is_none() {
        info!("Copying default themes from {:?} to {:?}", source_themes, dest_themes);
        copy_dir_all(source_themes, dest_themes)?;
    } else {
        debug!("Themes directory already populated, skipping default copy");
    }
    
    Ok(())
}

/// Gets the installation directory of MAVIS
pub fn get_install_dir() -> Result<PathBuf, CoreError> {
    let exe_path = env::current_exe()?;
    let install_dir = exe_path.parent()
        .ok_or_else(|| CoreError::IoError(std::io::Error::new(std::io::ErrorKind::NotFound, "Failed to determine installation directory")))?; // Wrap error
        
    Ok(install_dir.to_path_buf())
}