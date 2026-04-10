use std::process::Command;
use crate::settings::SettingsState;

#[tauri::command]
pub async fn open_in_editor(state: tauri::State<'_, SettingsState>, path: String) -> Result<(), String> {
    let editor = {
        let settings = state.0.lock().unwrap();
        settings.editor_command.clone()
    };

    #[cfg(target_os = "macos")]
    {
        if editor.is_empty() {
            Command::new("open")
                .args(["-a", "Visual Studio Code", &path])
                .spawn()
                .map_err(|e| format!("Failed to open editor: {}", e))?;
        } else {
            Command::new(&editor)
                .arg(&path)
                .spawn()
                .map_err(|e| format!("Failed to open editor '{}': {}", editor, e))?;
        }
    }

    #[cfg(target_os = "linux")]
    {
        let cmd = if editor.is_empty() { "code".to_string() } else { editor };
        Command::new(&cmd)
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open editor '{}': {}", cmd, e))?;
    }

    #[cfg(target_os = "windows")]
    {
        if editor.is_empty() {
            Command::new("cmd")
                .args(["/c", "code", &path])
                .spawn()
                .map_err(|e| format!("Failed to open editor: {}", e))?;
        } else {
            Command::new("cmd")
                .args(["/c", &editor, &path])
                .spawn()
                .map_err(|e| format!("Failed to open editor '{}': {}", editor, e))?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn open_in_terminal(state: tauri::State<'_, SettingsState>, path: String) -> Result<(), String> {
    let terminal = {
        let settings = state.0.lock().unwrap();
        settings.terminal_emulator.clone()
    };

    #[cfg(target_os = "macos")]
    {
        if terminal.is_empty() {
            Command::new("open")
                .args(["-a", "Terminal", &path])
                .spawn()
                .map_err(|e| format!("Failed to open terminal: {}", e))?;
        } else {
            Command::new(&terminal)
                .arg(&path)
                .spawn()
                .map_err(|e| format!("Failed to open terminal '{}': {}", terminal, e))?;
        }
    }

    #[cfg(target_os = "linux")]
    {
        if !terminal.is_empty() {
            let attempts: &[&[&str]] = &[
                &["--working-directory", &path],
                &["--workdir", &path],
                &["-e", &format!("cd '{}' && exec $SHELL", path)],
            ];
            let mut launched = false;
            for args in attempts {
                if Command::new(&terminal).args(*args).spawn().is_ok() {
                    launched = true;
                    break;
                }
            }
            if !launched {
                return Err(format!("Failed to open terminal '{}'", terminal));
            }
        } else {
            let attempts: &[(&str, &[&str])] = &[
                ("x-terminal-emulator", &["--working-directory", &path]),
                ("gnome-terminal", &["--working-directory", &path]),
                ("konsole", &["--workdir", &path]),
                ("alacritty", &["--working-directory", &path]),
                ("xfce4-terminal", &["--working-directory", &path]),
                ("xterm", &["-e", &format!("cd '{}' && exec $SHELL", path)]),
            ];
            let mut launched = false;
            for (term, args) in attempts {
                if Command::new(term).args(*args).spawn().is_ok() {
                    launched = true;
                    break;
                }
            }
            if !launched {
                return Err("No terminal emulator found".to_string());
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
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
