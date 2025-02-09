use std::{
    env,
    fs,
    path::{Path, PathBuf},
    process::{self, Command},
};

fn main() {
    let symb = Path::new("./pub.pdb");

    if symb.exists() {
        fs::remove_file(symb).unwrap();
    }

    let version = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please provide a version, e.g., ./pub.exe 3.0.0");
        process::exit(1);
    });

    let bins = ["game-server", "sdk-server", "cfg-manager"];

    for bin in &bins {
        println!("Building {}.", bin);
        let status = Command::new("cargo")
            .args(["build", "--release", "--bin", bin])
            .status()
            .expect("Failed to start cargo build");

        if !status.success() {
            eprintln!("Failed to build {}", bin);
            process::exit(1);
        }
    }

    let target_dir = Path::new("target").join("release");

    if !target_dir.exists() {
        eprintln!("Target directory not found. Was the build successful?");
        process::exit(1);
    }

    let exe_files: Vec<PathBuf> = bins
        .iter()
        .map(|bin| target_dir.join(format!("{}.exe", bin)))
        .collect();

    for exe in &exe_files {
        if !exe.exists() {
            eprintln!("{} not found.", exe.display());
            process::exit(1);
        }
    }

    let release_dir = Path::new("release");

    if !release_dir.exists() {
        println!("Release directory not found. Creating it.");
        fs::create_dir(release_dir).expect("Failed to create release directory");
    }

    let new_dir = release_dir.join(format!("{}-Windows-X86_64", version));

    if !new_dir.exists() {
        println!("Creating directory: {}", new_dir.display());
        fs::create_dir(&new_dir).expect("Failed to create new release directory");
    }

    println!("Copying .exe files.");

    for exe in &exe_files {
        let dest = new_dir.join(exe.file_name().unwrap());
        fs::copy(exe, &dest).unwrap_or_else(|e| {
            eprintln!("Failed to copy {}: {}", exe.display(), e);
            process::exit(1);
        });
    }

    let license_file = Path::new("LICENSE");

    if license_file.exists() {
        println!("Copying LICENSE file.");
        fs::copy(license_file, new_dir.join("LICENSE")).expect("Failed to copy LICENSE file");
    } else {
        println!("LICENSE file not found.");
    }

    let cfg_dir = Path::new("_cfg");

    if cfg_dir.exists() {
        println!("Copying _cfg directory.");
        copy_dir_recursive(cfg_dir, &new_dir.join("_cfg")).expect("Failed to copy _cfg directory");
    } else {
        println!("_cfg directory not found.");
    }

    println!("All files have been copied to {}", new_dir.display());
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
    if !dest.exists() {
        fs::create_dir(dest)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        if entry_path.is_dir() {
            copy_dir_recursive(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }
    Ok(())
}
