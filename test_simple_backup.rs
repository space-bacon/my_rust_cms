use std::process::Command;
use std::path::Path;
use std::fs;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use uuid::Uuid;

#[derive(Debug)]
struct SimpleBackupResult {
    pub id: String,
    pub filename: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub backup_type: String,
    pub checksum: String,
    pub description: Option<String>,
}

fn create_media_backup(description: &str) -> Result<SimpleBackupResult, Box<dyn std::error::Error>> {
    let timestamp = Utc::now();
    let backup_id = Uuid::new_v4().to_string();
    let filename = format!("media_backup_{}_{}.tar.gz", 
        &backup_id[..8], 
        timestamp.format("%Y%m%d_%H%M%S")
    );
    
    // Ensure backup directory exists
    let backup_dir = "./backups";
    fs::create_dir_all(backup_dir)?;
    
    let backup_path = Path::new(backup_dir).join(&filename);
    
    println!("🔄 Starting media backup...");
    println!("   Target file: {}", backup_path.display());

    // Check if media directories exist
    let media_paths = vec!["uploads", "backend/uploads"];
    let existing_paths: Vec<&str> = media_paths.iter()
        .filter(|path| Path::new(path).exists())
        .copied()
        .collect();

    if existing_paths.is_empty() {
        println!("⚠️  No media directories found, creating empty archive");
        // Create empty tar file
        let output = Command::new("tar")
            .arg("-czf")
            .arg(&backup_path)
            .arg("-T")
            .arg("/dev/null")
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to create empty archive: {}", stderr).into());
        }
    } else {
        println!("📁 Found media directories: {:?}", existing_paths);
        
        // Create tar.gz archive
        let mut tar_cmd = Command::new("tar");
        tar_cmd.arg("-czf")
            .arg(&backup_path)
            .arg("-C")
            .arg(".");

        for path in existing_paths {
            tar_cmd.arg(path);
        }

        let output = tar_cmd.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("tar failed: {}", stderr).into());
        }
    }

    // Calculate file size and checksum
    let metadata = fs::metadata(&backup_path)?;
    let file_contents = fs::read(&backup_path)?;
    let checksum = format!("sha256:{}", hex::encode(Sha256::digest(&file_contents)));

    println!("✅ Media backup completed successfully!");
    println!("   Size: {} bytes", metadata.len());
    println!("   Checksum: {}", checksum);

    Ok(SimpleBackupResult {
        id: backup_id,
        filename,
        size: metadata.len(),
        created_at: timestamp,
        backup_type: "media".to_string(),
        checksum,
        description: Some(description.to_string()),
    })
}

fn create_database_backup(description: &str) -> Result<SimpleBackupResult, Box<dyn std::error::Error>> {
    let timestamp = Utc::now();
    let backup_id = Uuid::new_v4().to_string();
    let filename = format!("db_backup_{}_{}.pgdump", 
        &backup_id[..8], 
        timestamp.format("%Y%m%d_%H%M%S")
    );
    
    // Ensure backup directory exists
    let backup_dir = "./backups";
    fs::create_dir_all(backup_dir)?;
    
    let backup_path = Path::new(backup_dir).join(&filename);
    
    println!("🔄 Starting database backup...");
    println!("   Target file: {}", backup_path.display());

    // Database connection parameters
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://myrustcms:password@localhost:5432/my_rust_cms".to_string()
    });
    
    // Parse database URL
    let url_parts: Vec<&str> = database_url.split("://").collect();
    if url_parts.len() != 2 {
        return Err("Invalid database URL format".into());
    }
    
    let connection_part = url_parts[1];
    let parts: Vec<&str> = connection_part.split('@').collect();
    if parts.len() != 2 {
        return Err("Invalid database URL format".into());
    }
    
    let auth_parts: Vec<&str> = parts[0].split(':').collect();
    let host_parts: Vec<&str> = parts[1].split('/').collect();
    
    if auth_parts.len() != 2 || host_parts.len() != 2 {
        return Err("Invalid database URL format".into());
    }
    
    let username = auth_parts[0];
    let password = auth_parts[1];
    let host_port: Vec<&str> = host_parts[0].split(':').collect();
    let host = host_port[0];
    let port = host_port.get(1).unwrap_or(&"5432");
    let database = host_parts[1];

    println!("   Connecting to: {}@{}:{}/{}", username, host, port, database);

    // Check if pg_dump is available
    if Command::new("pg_dump").arg("--version").output().is_err() {
        return Err("pg_dump not found. Please install PostgreSQL client tools.".into());
    }

    // Execute pg_dump with fallback options
    let output = Command::new("pg_dump")
        .arg("--host").arg(host)
        .arg("--port").arg(port)
        .arg("--username").arg(username)
        .arg("--dbname").arg(database)
        .arg("--no-password")
        .arg("--format=custom")
        .arg("--compress=9")
        .arg("--file").arg(&backup_path)
        .env("PGPASSWORD", password)
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                // If custom format fails, try plain SQL format
                println!("⚠️  Custom format failed, trying plain SQL format...");
                let fallback_output = Command::new("pg_dump")
                    .arg("--host").arg(host)
                    .arg("--port").arg(port)
                    .arg("--username").arg(username)
                    .arg("--dbname").arg(database)
                    .arg("--no-password")
                    .arg("--clean")
                    .arg("--create")
                    .arg("--file").arg(&backup_path)
                    .env("PGPASSWORD", password)
                    .output()?;

                if !fallback_output.status.success() {
                    let fallback_stderr = String::from_utf8_lossy(&fallback_output.stderr);
                    return Err(format!("pg_dump failed: {}\nFallback also failed: {}", stderr, fallback_stderr).into());
                }
            }
        }
        Err(e) => {
            return Err(format!("Failed to execute pg_dump: {}", e).into());
        }
    }

    // Calculate file size and checksum
    let metadata = fs::metadata(&backup_path)?;
    let file_contents = fs::read(&backup_path)?;
    let checksum = format!("sha256:{}", hex::encode(Sha256::digest(&file_contents)));

    println!("✅ Database backup completed successfully!");
    println!("   Size: {} bytes", metadata.len());
    println!("   Checksum: {}", checksum);

    Ok(SimpleBackupResult {
        id: backup_id,
        filename,
        size: metadata.len(),
        created_at: timestamp,
        backup_type: "database".to_string(),
        checksum,
        description: Some(description.to_string()),
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <backup_type> [description]", args[0]);
        eprintln!("backup_type: database or media");
        std::process::exit(1);
    }
    
    let backup_type = &args[1];
    let description = args.get(2).map(|s| s.as_str()).unwrap_or("Test backup");
    
    println!("🧪 Testing {} backup functionality...", backup_type);
    
    let result = match backup_type.as_str() {
        "database" => create_database_backup(description),
        "media" => create_media_backup(description),
        _ => {
            eprintln!("❌ Invalid backup type. Use 'database' or 'media'");
            std::process::exit(1);
        }
    };
    
    match result {
        Ok(backup_info) => {
            println!("\n🎉 Backup test completed successfully!");
            println!("   ID: {}", backup_info.id);
            println!("   Filename: {}", backup_info.filename);
            println!("   Size: {} bytes", backup_info.size);
            println!("   Type: {}", backup_info.backup_type);
            println!("   Created: {}", backup_info.created_at);
            if let Some(desc) = backup_info.description {
                println!("   Description: {}", desc);
            }
        }
        Err(e) => {
            eprintln!("❌ Backup test failed: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
