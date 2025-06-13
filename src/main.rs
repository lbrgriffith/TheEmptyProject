use std::fs::File;
use std::path::{Path, PathBuf, Component};
use std::time::SystemTime;
use anyhow::{Context, Result};
use chrono::{Datelike, Local, NaiveDateTime, TimeZone};
use clap::{Arg, Command};

fn main() -> Result<()> {
    let matches = Command::new("empty")
        .version("1.0.0")
        .about("A Rust implementation of the Linux touch command")
        .long_about("
Creates empty files or updates timestamps of existing files.

EXAMPLES:
    empty file.txt                    # Create empty file or update timestamp
    empty file1.txt file2.txt         # Create/update multiple files
    empty -c file.txt                 # Don't create if doesn't exist
    empty -a file.txt                 # Update only access time
    empty -m file.txt                 # Update only modification time
    empty -r ref.txt file.txt         # Copy timestamps from ref.txt
    empty -t 202501011200 file.txt    # Set specific time (YYMMDDhhmm)
    empty -t 20250101120030 file.txt  # Set time with seconds
        ")
        .arg(
            Arg::new("no-create")
                .short('c')
                .long("no-create")
                .help("Do not create files if they don't exist")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("access-time")
                .short('a')
                .long("access-time")
                .help("Change only the access time")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("modification-time")
                .short('m')
                .long("modification-time")
                .help("Change only the modification time")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("reference")
                .short('r')
                .long("reference")
                .value_name("FILE")
                .help("Use this file's times instead of current time"),
        )
        .arg(
            Arg::new("time")
                .short('t')
                .long("time")
                .value_name("STAMP")
                .help("Use [[CC]YY]MMDDhhmm[.ss] instead of current time"),
        )
        .arg(
            Arg::new("FILES")
                .help("Files to touch")
                .required(true)
                .num_args(1..),
        )
        .get_matches();

    let no_create = matches.get_flag("no-create");
    let access_only = matches.get_flag("access-time");
    let modify_only = matches.get_flag("modification-time");
    let reference_file = matches.get_one::<String>("reference");
    let time_stamp = matches.get_one::<String>("time");
    let files: Vec<_> = matches.get_many::<String>("FILES").unwrap().collect();

    // Security: Limit number of files to prevent resource exhaustion
    if files.len() > 1000 {
        eprintln!("Error: Too many files specified (max 1000)");
        std::process::exit(1);
    }

    for file_path in files {
        if let Err(e) = validate_and_touch_file(
            file_path,
            no_create,
            access_only,
            modify_only,
            reference_file,
            time_stamp,
        ) {
            eprintln!("Error processing file: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn validate_and_touch_file(
    file_path: &str,
    no_create: bool,
    access_only: bool,
    modify_only: bool,
    reference_file: Option<&String>,
    time_stamp: Option<&String>,
) -> Result<()> {
    // Security: Validate file path to prevent path traversal
    let safe_path = validate_file_path(file_path)?;
    
    // Security: Check if path is a symlink before operations
    if safe_path.is_symlink() {
        return Err(anyhow::anyhow!("Refusing to operate on symbolic links"));
    }

    touch_file(&safe_path, no_create, access_only, modify_only, reference_file, time_stamp)
}

fn validate_file_path(file_path: &str) -> Result<PathBuf> {
    // Security: Limit filename length to prevent DoS and filesystem issues
    if file_path.len() > 255 {
        return Err(anyhow::anyhow!("Filename too long"));
    }
    
    // Security: Check for null bytes and invalid characters
    if file_path.contains('\0') || file_path.contains('\n') || file_path.contains('\r') {
        return Err(anyhow::anyhow!("Invalid characters in filename"));
    }
    
    let path = PathBuf::from(file_path);
    
    // Security: Reject absolute paths to prevent system file access
    if path.is_absolute() {
        return Err(anyhow::anyhow!("Absolute paths are not allowed"));
    }
    
    // Security: Check for path traversal attempts
    for component in path.components() {
        match component {
            Component::ParentDir => {
                return Err(anyhow::anyhow!("Path traversal attempts are not allowed"));
            }
            Component::CurDir => {
                // Allow current directory references
                continue;
            }
            Component::Normal(name) => {
                // Security: Validate individual path components
                let name_str = name.to_string_lossy();
                if name_str.len() > 255 {
                    return Err(anyhow::anyhow!("Path component too long"));
                }
                continue;
            }
            _ => {
                return Err(anyhow::anyhow!("Invalid path component"));
            }
        }
    }
    
    // Security: Ensure path doesn't resolve outside current directory
    let canonical_current = std::env::current_dir()?;
    let target_path = canonical_current.join(&path);
    
    if !target_path.starts_with(&canonical_current) {
        return Err(anyhow::anyhow!("Path resolves outside current directory"));
    }
    
    Ok(path)
}

fn touch_file(
    path: &Path,
    no_create: bool,
    access_only: bool,
    modify_only: bool,
    reference_file: Option<&String>,
    time_stamp: Option<&String>,
) -> Result<()> {
    if !path.exists() {
        if no_create {
            return Ok(());
        }
        File::create(path)
            .with_context(|| "Failed to create file")?;
    }

    if reference_file.is_some() || time_stamp.is_some() || access_only || modify_only {
        let target_time = if let Some(ref_file) = reference_file {
            get_reference_time(ref_file)?
        } else if let Some(stamp) = time_stamp {
            parse_timestamp(stamp)?
        } else {
            SystemTime::now()
        };

        set_file_times(path, target_time, access_only, modify_only)?;
    }

    Ok(())
}

fn get_reference_time(reference_file: &str) -> Result<SystemTime> {
    // Security: Validate reference file path as well
    let safe_ref_path = validate_file_path(reference_file)?;
    
    let metadata = std::fs::metadata(&safe_ref_path)
        .with_context(|| "Failed to read reference file")?;
    
    Ok(metadata.modified().unwrap_or(SystemTime::now()))
}

fn parse_timestamp(timestamp: &str) -> Result<SystemTime> {
    // Security: Validate timestamp input to prevent injection
    if !timestamp.chars().all(|c| c.is_ascii_digit()) {
        return Err(anyhow::anyhow!("Invalid timestamp format - only digits allowed"));
    }
    
    // Security: Limit timestamp length to prevent DoS
    if timestamp.len() > 14 {
        return Err(anyhow::anyhow!("Timestamp too long"));
    }
    
    let now = Local::now();
    let current_year = now.year();
    let current_century = current_year / 100;

    let parsed_time = if timestamp.len() == 12 {
        NaiveDateTime::parse_from_str(timestamp, "%Y%m%d%H%M")
    } else if timestamp.len() == 10 {
        let with_century = format!("{}{}", current_century, timestamp);
        NaiveDateTime::parse_from_str(&with_century, "%Y%m%d%H%M")
    } else if timestamp.len() == 8 {
        let with_year = format!("{}{}", current_year, timestamp);
        NaiveDateTime::parse_from_str(&with_year, "%Y%m%d%H%M")
    } else {
        return Err(anyhow::anyhow!("Invalid timestamp format"));
    };

    let naive_dt = parsed_time.with_context(|| "Failed to parse timestamp")?;
    let local_dt = Local.from_local_datetime(&naive_dt).single()
        .ok_or_else(|| anyhow::anyhow!("Invalid local datetime"))?;
    
    Ok(local_dt.into())
}

fn set_file_times(
    path: &Path,
    time: SystemTime,
    access_only: bool,
    modify_only: bool,
) -> Result<()> {
    let metadata = path.metadata()?;
    
    // Cross-platform way to get current times
    let current_atime = metadata.accessed().unwrap_or(SystemTime::now());
    let current_mtime = metadata.modified().unwrap_or(SystemTime::now());

    let new_atime = if modify_only { current_atime } else { time };
    let new_mtime = if access_only { current_mtime } else { time };

    filetime::set_file_times(path, 
        filetime::FileTime::from_system_time(new_atime),
        filetime::FileTime::from_system_time(new_mtime)
    )?;

    Ok(())
}
