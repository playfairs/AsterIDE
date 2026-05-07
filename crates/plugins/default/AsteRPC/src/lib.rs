// this file is meant for rewrite, any and all comments in this file
// can be removed after the RPC plugin works as intended.

use discord_presence::Client as DiscordClient;
use discord_presence::models::Activity;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

pub mod integration;
pub use integration::EditorIntegration;

pub struct AsteRPC {
    client: Arc<Mutex<DiscordClient>>,
    app_id: String,
    start_timestamp: i64,
    connected: Arc<AtomicBool>,
}

#[derive(Debug, Clone)]
pub struct WorkStatus {
    pub project: String,
    pub file: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub language: String,
    pub is_editing: bool,
}

impl Default for WorkStatus {
    fn default() -> Self {
        Self {
            project: "No workspace".to_string(),
            file: "No file open".to_string(),
            line: None,
            column: None,
            language: "text".to_string(),
            is_editing: false,
        }
    }
}

impl AsteRPC {
    pub fn new(app_id: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = DiscordClient::new(app_id.parse()?);
        
        let start_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        Ok(Self {
            client: Arc::new(Mutex::new(client)),
            app_id: app_id.to_string(),
            start_timestamp,
            connected: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn is_discord_running() -> bool {
        // check if discord is running by trying to connect to its RPC pipe
        // discord creates a named pipe on Windows or unix socket on macOS/Linux
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::fs::FileExt;
            std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(r"\\.\pipe\discord-ipc-0")
                .is_ok()
        }
        #[cfg(target_os = "macos")]
        {
            // Official Discord clients.
            let discord = ["Discord", "Discord PTB", "Discord Canary", "DiscordDevelopment"];
            
            // Modded/Alternative clients (Equicord, Vencord, Ripcord, Powercord, LightCord).
            let other = ["Equibop", "Vesktop", "Ripcord", "Powercord", "Lightcord"];
            
            // Official Discord clients first, then modded.
            for process in &discord {
                let result = std::process::Command::new("pgrep")
                    .arg("-ix") // Case insensitivity
                    .arg(process)
                    .output();
                    
                if let Ok(output) = result {
                    if output.status.success() {
                        println!("Discord process detected: {}", process);
                        
                        // this checks for discords ipc socket (case-insensitive paths)
                        let home_dir = std::env::var("HOME").unwrap_or_default();
                        let ipc_paths = [
                            format!("{}/Library/Application Support/discord/discord-ipc-0", home_dir),
                            format!("{}/Library/Application Support/Discord/discord-ipc-0", home_dir),
                            format!("{}/Library/Application Support/discordcanary/discord-ipc-0", home_dir),
                            format!("{}/Library/Application Support/Discord Canary/discord-ipc-0", home_dir),
                            format!("{}/Library/Application Support/discordptb/discord-ipc-0", home_dir),
                            format!("{}/Library/Application Support/Discord PTB/discord-ipc-0", home_dir),
                        ];
                        
                        for path in &ipc_paths {
                            let ipc_path = std::path::Path::new(path);
                            if ipc_path.exists() {
                                println!("Discord IPC socket found at: {}", path);
                                return true;
                            }
                        }
                        
                        // if process is running but no ipc found, discord might be using a different ipc mechanism
                        // if that is the case, we will need to implement a new and better system for this
                        println!("Discord process found, but IPC socket not in standard locations - will try connecting anyway");
                        return true;
                    }
                }
                
                // partial match just in case im stupid
                let result = std::process::Command::new("pgrep")
                    .arg("-i")
                    .arg(process)
                    .output();
                    
                if let Ok(output) = result {
                    if output.status.success() {
                        println!("Discord process detected (partial): {}", process);
                        return true;
                    }
                }
            }
            
            // now we can check for modded clients
            for process in &other {
                let result = std::process::Command::new("pgrep")
                    .arg("-ix")
                    .arg(process)
                    .output();
                    
                if let Ok(output) = result {
                    if output.status.success() {
                        println!("Modded Discord client detected: {}", process);
                        return true;
                    }
                }
                
                // also partial match
                let result = std::process::Command::new("pgrep")
                    .arg("-i")
                    .arg(process)
                    .output();
                    
                if let Ok(output) = result {
                    if output.status.success() {
                        println!("Modded Discord client detected (partial): {}", process);
                        return true;
                    }
                }
            }
            
            let temp_dir = std::env::temp_dir();
            let discord_ipc = temp_dir.join("discord-ipc-0");
            if discord_ipc.exists() {
                println!("Discord IPC socket found in temp dir");
                return true;
            }
            
            false
        }
        #[cfg(target_os = "linux")]
        {
            let discord_processes = ["Discord", "discord", "DiscordPTB", "DiscordCanary"];
            
            for process in &discord_processes {
                let result = std::process::Command::new("pgrep")
                    .arg("-x")
                    .arg(process)
                    .output();
                    
                if let Ok(output) = result {
                    if output.status.success() {
                        println!("Discord detected: {}", process);
                        return true;
                    }
                }
            }
            
            // Check for Discord IPC socket invarious locations
            let ipc_paths = [
                std::env::temp_dir().join("discord-ipc-0"),
                std::env::temp_dir().join("app/com.discordapp.Discord/discord-ipc-0"),
            ];
            
            for path in &ipc_paths {
                if path.exists() {
                    println!("Discord IPC socket found at: {:?}", path);
                    return true;
                }
            }
            
            false
        }
    }

    pub fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !Self::is_discord_running() {
            return Err("Discord is not running".into());
        }

        let client = self.client.clone();
        let connected = self.connected.clone();
        
        std::thread::spawn(move || {
            let mut client = client.lock().unwrap();
            
            let connected_cb = connected.clone();
            client.on_ready(move |_ctx| {
                println!("Discord RPC connected successfully!");
                connected_cb.store(true, Ordering::SeqCst);
            });
            
            let connected_cb = connected.clone();
            client.on_error(move |_ctx| {
                eprintln!("Discord RPC error occurred");
                connected_cb.store(false, Ordering::SeqCst);
            });
            
            client.start();
            println!("Discord RPC client thread ended");
        });
        
        std::thread::sleep(Duration::from_millis(500));
        
        if self.connected.load(Ordering::SeqCst) {
            println!("Discord RPC connection established!");
            Ok(())
        } else {
            println!("Discord RPC connection in progress...");
            Ok(())
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    pub fn disconnect(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.connected.store(false, Ordering::SeqCst);
        Ok(())
    }

    pub fn update_status(&self, status: &WorkStatus) -> Result<(), Box<dyn std::error::Error>> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err("Not connected to Discord".into());
        }

        let state = if status.is_editing {
            format!("Editing {}", status.language)
        } else {
            "Viewing".to_string()
        };

        let details = if status.file != "No file open" {
            format!("{}", status.file)
        } else {
            "AsterIDE".to_string()
        };

        let large_text = format!("Project: {}", status.project);
        
        println!("Updating Discord RPC: {} - {}", state, details);
        
        let activity = Activity::new()
            .state(&state)
            .details(&details)
            .assets(|a| {
                a.large_image("asteride")
                    .large_text(&large_text)
                    .small_image(if status.is_editing { "edit" } else { "view" })
                    .small_text(if status.is_editing { "Editing" } else { "Viewing" })
            })
            .timestamps(|t| t.start(self.start_timestamp as u64));

        let mut client = self.client.lock().unwrap();
        client.set_activity(|_| activity)?;
        
        println!("Discord RPC status updated successfully!");
        Ok(())
    }

    pub fn clear_status(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = self.client.lock().unwrap();
        client.clear_activity()?;
        Ok(())
    }
}

pub fn get_language_from_extension(file: &str) -> String {
    let ext = std::path::Path::new(file)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("text");
    
    match ext.to_lowercase().as_str() {
        // web
        "js" => "JavaScript",
        "ts" => "TypeScript",
        "jsx" => "React",
        "tsx" => "TypeScript React",
        "html" => "HTML",
        "css" => "CSS",
        "scss" => "SCSS",
        "php" => "PHP",
        
        // program
        "rs" => "Rust",
        "py" => "Python",
        "go" => "Go",
        "java" => "Java",
        "c" => "C",
        "fs" => "F#",
        "cs" => "C#"
        "cpp" | "cc" | "cxx" => "C++",
        "h" | "hpp" => "C Header",
        "kt" => "Kotlin",
        "swift" => "Swift",
        "rb" => "Ruby",
        "zig" => "Zig",
        "haskell" => "Haskell",
        "asm" | "s" => "Assembly",
        
        // config / data
        "toml" => "TOML",
        "yaml" | "yml" => "YAML",
        "json" => "JSON",

        // markup
        "md" => "Markdown",
        "adoc" => "Ascii Doc",
        "org" => "I forgot what org is"
        
        // basic shell script
        "sh" | "bash" => "Shell",
        "bat" => "Batch",
        "nix" => "Nix",
        "lua" => "Lua",
        "fish" => "Fish",
        "zsh" => "Zsh",
        "ps1" => "PowerShell",

        // buildsystem files
        "dockerfile" => "Docker",
        "makefile" => "Makefile",
        "cmake" => "CMake",
        "justfile" => "Justfile",
        
        _ => "Text",
    }.to_string()
}

// yeah because why not right, we can remove this after we push this shit
// to stable, if i fucking remember anyways, honestly this will probably
// be rewritten anyways so who cares, my main concern is getting the shit to
// detect discord in the first place and then establish a connection.
// we can worry about the file accuracy and details shit later.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection() {
        assert_eq!(get_language_from_extension("test.rs"), "Rust");
        assert_eq!(get_language_from_extension("test.py"), "Python");
        assert_eq!(get_language_from_extension("test.js"), "JavaScript");
        assert_eq!(get_language_from_extension("test.txt"), "Text");
    }
}
