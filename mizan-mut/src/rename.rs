use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Result};
use ide::{AnalysisHost, FilePosition, TextSize};
use ide_db::base_db::FileId;
use load_cargo::{load_workspace_at, LoadCargoConfig, ProcMacroServerChoice};
use project_model::CargoConfig;
use vfs::Vfs;

pub fn rename_symbol(crate_root: &str, file: &str, offset: u32, new_name: &str) -> Result<()> {
    let (db, vfs, _) = load_workspace(crate_root)?;
    let host = AnalysisHost::with_database(db);
    let analysis = host.analysis();

    // Find the file
    let file_path = Path::new(crate_root).join(file);
    let file_id = find_file_id(&vfs, &file_path)?;

    let file_position = FilePosition {
        file_id,
        offset: TextSize::from(offset),
    };

    println!("Renaming symbol at {}:{} to '{}'", file, offset, new_name);

    let source_change = match analysis.rename(file_position, new_name) {
        Ok(Ok(change)) => change,
        Ok(Err(e)) => bail!("Rename error: {}", e),
        Err(e) => bail!("Rename error: {}", e),
    };
    apply_changes(&vfs, source_change)?;

    Ok(())
}

fn load_workspace(
    crate_root: &str,
) -> Result<(
    ide_db::RootDatabase,
    Vfs,
    Option<proc_macro_api::ProcMacroClient>,
)> {
    let cargo_config = CargoConfig::default();

    let load_config = LoadCargoConfig {
        load_out_dirs_from_check: false,
        with_proc_macro_server: ProcMacroServerChoice::None,
        prefill_caches: false,
    };

    load_workspace_at(Path::new(crate_root), &cargo_config, &load_config, &|_| {})
}

fn find_file_id(vfs: &Vfs, file_path: &Path) -> Result<FileId> {
    // Normalize the path
    let normalized_path = file_path
        .canonicalize()
        .unwrap_or_else(|_| file_path.to_path_buf());

    // Search through VFS for matching file
    for (file_id, vfs_path) in vfs.iter() {
        if let Some(path) = vfs_path.as_path() {
            let vfs_absolute = std::env::current_dir()
                .ok()
                .and_then(|cwd| cwd.join(path.as_ref() as &Path).canonicalize().ok());

            if let Some(vfs_abs) = vfs_absolute {
                if vfs_abs == normalized_path {
                    return Ok(file_id);
                }
            }
        }
    }

    bail!("File '{}' not found in the project", file_path.display())
}

fn apply_changes(vfs: &Vfs, source_change: ide_db::source_change::SourceChange) -> Result<()> {
    if source_change.source_file_edits.is_empty() {
        println!("No changes needed!");
        return Ok(());
    }

    println!("\nChanges to apply:");

    // Apply source file edits
    for (file_id, (edit, _)) in &source_change.source_file_edits {
        let path = vfs
            .file_path(*file_id)
            .as_path()
            .ok_or_else(|| anyhow!("No path for file"))?;

        let abs_path: PathBuf = std::env::current_dir()?.join(path.as_ref() as &Path);
        println!("\nFile: {}", abs_path.display());

        // Read file
        let mut content = fs::read_to_string(&abs_path)?;

        // Apply edits
        edit.apply(&mut content);

        // Write file
        fs::write(&abs_path, content)?;
        println!("  Applied {} edits", edit.len());
    }

    println!("\nAll changes applied successfully!");

    Ok(())
}
