use std::process::Command;

#[tauri::command]
pub async fn open_in_vscode(path: String) -> Result<(), String> {
    Command::new("code")
        .arg(&path)
        .spawn()
        .map_err(|e| format!("Failed to open VS Code: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn open_in_terminal(path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args(["-a", "Terminal", &path])
            .spawn()
            .map_err(|e| format!("Failed to open terminal: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        // Try common terminal emulators — each has different flags for
        // setting the working directory.
        let attempts: &[(&str, &[&str])] = &[
            ("x-terminal-emulator", &["--working-directory", &path]),
            ("gnome-terminal", &["--working-directory", &path]),
            ("konsole", &["--workdir", &path]),
            ("alacritty", &["--working-directory", &path]),
            ("xfce4-terminal", &["--working-directory", &path]),
            // xterm doesn't support a working-directory flag; use sh -c
            ("xterm", &["-e", &format!("cd '{}' && exec $SHELL", path)]),
        ];
        let mut launched = false;
        for (term, args) in attempts {
            if Command::new(term)
                .args(*args)
                .spawn()
                .is_ok()
            {
                launched = true;
                break;
            }
        }
        if !launched {
            return Err("No terminal emulator found".to_string());
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Try Windows Terminal first, fall back to cmd.exe.
        // Paths are quoted to handle spaces.
        Command::new("cmd")
            .args(["/c", "start", "wt", "-d", &format!("\"{}\"", path)])
            .spawn()
            .or_else(|_| {
                Command::new("cmd")
                    .args(["/c", "start", "cmd", "/k", &format!("cd /d \"{}\"", path)])
                    .spawn()
            })
            .map_err(|e| format!("Failed to open terminal: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn open_in_finder(path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open Finder: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file manager: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open Explorer: {}", e))?;
    }

    Ok(())
}
