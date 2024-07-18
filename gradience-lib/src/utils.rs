pub fn run_command(command: &str) -> std::process::Output {
    // if os var FLATPAK_ID exists prefix command
    if let Ok(_) = std::env::var("FLATPAK_ID") {
        std::process::Command::new("flatpak-spawn")
            .arg("--host")
            .arg("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    }

}

#[derive(Debug)]
pub enum ShellVersion {
    G46,
    Unsupported
}

pub fn get_gnome_shell_version() -> ShellVersion {
    let output = run_command("gnome-shell --version");
    let version = String::from_utf8(output.stdout).unwrap();
    let version = version.split_whitespace().collect::<Vec<&str>>()[2].to_string();
    let version = version.split(".").collect::<Vec<&str>>()[0].parse::<u32>().unwrap();

    match version {
        46 => ShellVersion::G46,
        _ => ShellVersion::Unsupported,
    }
}

pub fn check_installed_extension(extension: &str) -> bool {
    let output = run_command("gnome-extensions list");
    let output = String::from_utf8(output.stdout).unwrap();
    output.contains(extension)
}

pub fn check_enabled_extension(extension: &str) -> bool {
    let output = run_command("gnome-extensions list --enabled");
    let output = String::from_utf8(output.stdout).unwrap();
    output.contains(extension) 
}

pub fn set_shell_theme(theme: &str) {
    if check_enabled_extension("user-theme@gnome-shell-extensions.gcampax.github.com") {
        run_command(&format!("gsettings set org.gnome.shell.extensions.user-theme name '{}'", theme));
    } else if check_installed_extension("user-theme@gnome-shell-extensions.gcampax.github.com") {
        run_command("gnome-extensions enable user-theme@gnome-shell-extensions.gcampax.github.com");
        run_command(&format!("gsettings set org.gnome.shell.extensions.user-theme name '{}'", theme));
    }
}

pub fn reset_shell_theme() {
    run_command("gsettings reset org.gnome.shell.extensions.user-theme name");
}