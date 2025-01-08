use colored::*;
use std::io::ErrorKind;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, OpenOptions};
use std::io::Result;
use std::io::{self, Write};
use std::path::Path;
use std::process::{self, Command};
use std::thread::sleep;
use std::time::Duration;
use dirs;
use std::thread;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::env;

pub fn delete_venv() -> std::io::Result<()> {
    let _ = list_venvs();
    let venv_name = prompt("Choose a VENV to delete > ");
    
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let venman_dir = home_dir.join("venman");
    let venvs_path = venman_dir.join("venvs");
    let config_path = venman_dir.join("venvs.toml");

    if !venvs_path.exists() {
        println!("{}", "No virtual environments found.".yellow());
        return Ok(());
    }

    let venv_path = venvs_path.join(&venv_name);
    if !venv_path.exists() {
        println!("{} {}", "Virtual environment".red(), format!("'{}' not found.", venv_name).red());
        return Ok(());
    }

    let mut config_content = match std::fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(_) => {
            println!("{}", "Could not read configuration file.".red());
            return Ok(());
        }
    };

    println!("{}", "Are you sure you want to delete this virtual environment?".yellow());
    println!("Environment: {}", venv_name.red());
    let confirm = prompt("Type 'yes' to confirm deletion: ");

    if confirm.to_lowercase() != "yes" {
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", "Deletion cancelled.".green());
        return Ok(());
    }

    std::fs::remove_dir_all(&venv_path)?;

    let mut config: toml::Value = match toml::from_str(&config_content) {
        Ok(cfg) => cfg,
        Err(_) => {
            println!("{}", "Invalid configuration format.".red());
            return Ok(());
        }
    };

    if let Some(table) = config.as_table_mut() {
        table.remove(&venv_name);
    }

    let updated_config = toml::to_string_pretty(&config)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    std::fs::write(&config_path, updated_config)?;
    print!("\x1B[2J\x1B[1;1H");
    println!("{} {} {}", 
        "Virtual environment".green(), 
        venv_name.bright_red(), 
        "has been successfully deleted.".green()
    );

    Ok(())
}

pub fn activate_venv() {
    let _ = list_venvs();
    let venv_name = prompt("Choose a VENV > ");
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let venman_dir = home_dir.join("venman");
    let venvs_path = venman_dir.join("venvs");
    let config_path = venman_dir.join("venvs.toml");

    if !venvs_path.exists() {
        println!("{}", "No virtual environments found.".yellow());
        return;
    }

    let config_content = match std::fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(_) => {
            println!("{}", "Could not read configuration file.".red());
            return;
        }
    };

    let config: toml::Value = match toml::from_str(&config_content) {
        Ok(cfg) => cfg,
        Err(_) => {
            println!("{}", "Invalid configuration format.".red());
            return;
        }
    };

    let venv_path = venvs_path.join(&venv_name);
    if !venv_path.exists() {
        println!("{} {}", "Virtual environment".red(), format!("'{}' not found.", venv_name).red());
        return;
    }

    #[cfg(windows)]
    let activate_path = venv_path.join("Scripts").join("activate.bat");
    #[cfg(not(windows))]
    let activate_path = venv_path.join("bin").join("activate");

    if !activate_path.exists() {
        println!("{}", "Activation script not found.".red());
        return;
    }

    if let Some(env_config) = config.get(&venv_name) {
        let description = env_config.get("description")
            .and_then(|d| d.as_str())
            .unwrap_or("No description");
        
        let packages = env_config.get("packages")
            .and_then(|p| p.as_str())
            .unwrap_or("No packages");

        println!("{}", "Activating Virtual Environment:".green());
        println!("{}: {}", "Name".bright_cyan(), venv_name.bold());
        println!("{}: {}", "Description".bright_cyan(), description);
        println!("{}: {}", "Packages".bright_cyan(), packages);
    }

    #[cfg(windows)]
    {
        Command::new("cmd")
            .arg("/C")
            .arg(activate_path.display().to_string())
            .status()
            .expect("Failed to activate virtual environment");
    }

    #[cfg(not(windows))]
    {
        Command::new("bash")
            .arg("-c")
            .arg(format!("source {}/bin/activate && exec $SHELL", venv_path.display()))
            .status()
            .expect("Failed to start shell");
    }
}

pub fn list_venv_names() -> std::io::Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let venvs_path = home_dir.join("venman").join("venvs");

    if !venvs_path.exists() {
        println!("{}", "No virtual environments found.".yellow());
        return Ok(());
    }

    println!("{}", "Available Virtual Environments:".green());
    
    for entry in std::fs::read_dir(&venvs_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let venv_name = path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown");

            println!("• {}", venv_name.bright_cyan());
        }
    }

    Ok(())
}

fn list_venvs() -> std::io::Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let venman_dir = home_dir.join("venman");
    let venvs_path = venman_dir.join("venvs");
    let config_path = venman_dir.join("venvs.toml");

    if !venvs_path.exists() {
        println!("{}", "No virtual environments found.".yellow());
        return Ok(());
    }

    let config_content = match std::fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(_) => {
            println!("{}", "Could not read configuration file.".red());
            return Ok(());
        }
    };

    let config: toml::Value = match toml::from_str(&config_content) {
        Ok(cfg) => cfg,
        Err(_) => {
            println!("{}", "Invalid configuration format.".red());
            return Ok(());
        }
    };

    println!("{}", "VenMan Virtual Environments".green().bold());
    println!("{}", "------------------------------".green());

    let mut env_found = false;

    for entry in std::fs::read_dir(&venvs_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let venv_name = path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown");

            let venv_config = config.get(venv_name);

            match venv_config {
                Some(cfg) => {
                    env_found = true;
                    let description = cfg.get("description")
                        .and_then(|d| d.as_str())
                        .unwrap_or("No description");
                    
                    let packages = cfg.get("packages")
                        .and_then(|p| p.as_str())
                        .unwrap_or("No packages");

                    println!(
                        "{} | {} 📦\n  📝 Description : {}\n  📦 Packages: {}\n",
                        venv_name.bright_cyan().bold(),
                        "Description:".yellow(),
                        description,
                        packages
                    );
                },
                None => {
                    println!(
                        "{} | {} No configuration available\n",
                        venv_name.bright_cyan().bold(),
                        "⚠️".yellow()
                    );
                }
            }
        }
    }

    if !env_found {
        println!("{}", "No virtual environments discovered.".yellow());
    }
    Ok(())
}

fn loading_animation(running: Arc<AtomicBool>, message: String, done: String) {
    let spinner = vec!["⢿", "⣻", "⣽", "⣾", "⣷", "⣯", "⣟", "⡿"];
    let mut i = 0;
    while running.load(Ordering::Relaxed) {
        print!("\r{} {}", spinner[i], message);
        std::io::stdout().flush().unwrap();
        i = (i + 1) % spinner.len();
        thread::sleep(Duration::from_millis(80));
    }
    println!("\r{:<width$}", done, width = message.len() + spinner[0].len() + 1);
    std::io::stdout().flush().unwrap();
}

fn create_env(path: &str, name: String, packages: &str) -> Result<()> {
    let running_env = Arc::new(AtomicBool::new(true));
    let running_env_clone = running_env.clone();
    let name_clone = name.clone(); 

    let spinner_thread_env = thread::spawn(move || {
        loading_animation(running_env_clone, 
            format!("Creating env \"{}\". please wait...", name_clone), 
            String::from("Done!                                ")
        );
    });

    let env_path = Path::new(path);

    create_dir_all(env_path)?;

    let venv_output = Command::new("python")
        .arg("-m")
        .arg("venv")
        .arg(env_path)
        .output()?;

    if !venv_output.status.success() {
        eprintln!("Failed to create virtual environment: {}", 
            String::from_utf8_lossy(&venv_output.stderr)
        );
        return Err(io::Error::new(
            io::ErrorKind::Other, 
            "Failed to create virtual environment"
        ));
    }
    
    running_env.store(false, Ordering::Relaxed);
    spinner_thread_env.join().unwrap();

    if !packages.is_empty() {
        let running_packages = Arc::new(AtomicBool::new(true));
        let running_packages_clone = running_packages.clone();
        let packages_clone = packages.to_string(); 

        let spinner_thread_packages = thread::spawn(move || {
            loading_animation(running_packages_clone, 
                format!("Installing packages \"{}\". please wait...", packages_clone), 
                String::from("Done!                                        ")
            );
        });

        let pip_path = if cfg!(windows) {
            env_path.join("Scripts").join("pip.exe")
        } else {
            env_path.join("bin").join("pip")
        };

        let package_list: Vec<&str> = packages.split_whitespace().collect();
        
        let pip_output = Command::new(&pip_path)
            .arg("install")
            .args(&package_list)
            .output()?;

        if !pip_output.status.success() {
            eprintln!("Failed to install packages: {}", 
                String::from_utf8_lossy(&pip_output.stderr)
            );
            return Err(io::Error::new(
                io::ErrorKind::Other, 
                "Failed to install packages"
            ));
        }
        
        running_packages.store(false, Ordering::Relaxed);
        spinner_thread_packages.join().unwrap();
    }

    print!("\x1B[2J\x1B[1;1H");
    println!("{} {} {}", 
        "Virtual environment".green(), 
        name.bright_red(), 
        "has been successfully created.".green()
    );

    Ok(())
}

fn append_to_file(path: &str, content: &str) -> Result<()> {
    let home_dir = dirs::home_dir().expect("Could not find home dir");
    
    let full_path = home_dir.join("venman").join(path);
    
    if let Some(parent_dir) = full_path.parent() {
        create_dir_all(parent_dir).unwrap_or_else(|_| {
            eprintln!("Couldn't create dir : {}", parent_dir.display());
        });
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&full_path)
        .unwrap_or_else(|e| {
            eprintln!("Erreur lors de l'ouverture du fichier : {}", e);
            panic!("Impossible d'ouvrir le fichier");
        });

    writeln!(file, "{}", content).unwrap_or_else(|e| {
        eprintln!("Erreur lors de l'écriture : {}", e);
    });

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    TopToBottom,
}

#[derive(Debug, Clone, Copy)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Env {
    name: String,
    description: String,
    packages: String,
}

impl Env {
    fn create(name: String, description: String, packages: String) -> Result<()> {
        
        let mut table = toml::Table::new();
        let mut env_table = toml::Table::new();

        env_table.insert("description".to_string(), toml::Value::String(description));
        env_table.insert("packages".to_string(), toml::Value::String(packages.clone()));

        table.insert(name.clone(), toml::Value::Table(env_table));

        let toml_string = toml::to_string(&table)
            .map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;
                        
        let home_dir = dirs::home_dir().expect("Could not find home folder");
        let venman_dir = home_dir.join("venman");
        create_dir_all(&venman_dir)?;

        let venplace = venman_dir.join("venvs");
        create_dir_all(&venplace)?;
        let venplace = venplace.join(&name);
            
        append_to_file("venvs.toml", &toml_string)?;
            
        create_env(venplace.to_str().unwrap(), name, &packages)?;
        
        Ok(())
    }
}

impl RGB {
    fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }
}

fn fade_print_multiline(
    text: &str,
    start_color: RGB,
    end_color: RGB,
    direction: Direction,
    speed: u64,
) {
    let lines: Vec<&str> = text.lines().collect();
    match direction {
        Direction::TopToBottom => {
            for (index, &line) in lines.iter().enumerate() {
                let progress = index as f32 / (lines.len() - 1) as f32;

                let r =
                    (start_color.r as f32 * (1.0 - progress) + end_color.r as f32 * progress) as u8;
                let g =
                    (start_color.g as f32 * (1.0 - progress) + end_color.g as f32 * progress) as u8;
                let b =
                    (start_color.b as f32 * (1.0 - progress) + end_color.b as f32 * progress) as u8;

                println!("{}", line.truecolor(r, g, b));
                sleep(Duration::from_millis(speed));
            }
        }
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Input error");
    input.trim().to_string()
}

fn main() -> io::Result<()> {

    loop {
        //print!("\x1B[2J\x1B[1;1H");
        let board = "\n\n┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃          VENMAN - 0.1.7          ┃
┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃ [1] Create Venv                  ┃
┠──────────────────────────────────┨
┃ [2] Enter Venv                   ┃
┠──────────────────────────────────┨
┃ [3] List Venv                    ┃
┠──────────────────────────────────┨
┃ [4] Delete Venv                  ┃
┠──────────────────────────────────┨
┃ [5] Quit                         ┃
┠━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛";

        fade_print_multiline(
            board,
            RGB::new(255, 188, 5),
            RGB::new(255, 30, 5),
            Direction::TopToBottom,
            0,
        );

        let choice = prompt("\x1b[38;2;255;30;5m╰──────────────────\x1b[0m$ ");

        match choice.as_str() {
            "1" => {
                print!("\x1B[2J\x1B[1;1H");
                let venv_name = prompt("Enter VENV name >> ");
                let venv_desc = prompt("Enter VENV description (optional) >> ");
                let packages = prompt("Enter packages (spaces separated (optional)) >> ");
                
                match Env::create(venv_name.clone(), venv_desc.clone(), packages.clone()) {
                    Ok(_) => print!(""),
                    Err(e) => eprintln!("Error while creating VENV {}", e),
                }
            }
            "2" => {
                activate_venv();
            }
            "3" => {
                let _ = list_venvs();
            }
            "4" => {
                let _ = delete_venv();
            }
            "5" => {
                process::exit(0);

            }
            _ => {
                print!("\x1B[2J\x1B[1;1H");
            }
        }
    }
}
