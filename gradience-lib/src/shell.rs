use crate::preset::{AccentsColor, ApplyBuilder, Mode, Preset};
use crate::utils::{get_gnome_shell_version, run_command, ShellVersion};
use grass::from_path;
use walkdir::WalkDir;

pub struct Shell {
    pub version: ShellVersion,
    pub source_dir: String,
    pub preset: Preset,
}

pub enum ThemeName {
    Default,
    Custom { name: String },
}

impl Shell {
    pub fn new(source_dir: String, preset: Preset) -> Shell {
        let version = get_gnome_shell_version();
        Shell {
            version,
            source_dir,
            preset,
        }
    }

    fn apply_gtk(&self, mode: Mode, accent: AccentsColor, theme_dir: String) {
        if !std::path::Path::new(&format!("{}/gtk-4.0", theme_dir)).exists() {
            std::fs::create_dir_all(&format!("{}/gtk-4.0", theme_dir)).unwrap();
        }
        if !std::path::Path::new(&format!("{}/gtk-3.0", theme_dir)).exists() {
            std::fs::create_dir_all(&format!("{}/gtk-3.0", theme_dir)).unwrap();
        }

        ApplyBuilder::new(self.preset.clone())
            .accent(accent)
            .mode(mode)
            .gtk3_path(format!("{}/gtk-3.0/gtk.css", theme_dir).as_str())
            .gtk4_path(format!("{}/gtk-4.0/gtk.css", theme_dir).as_str())
            .apply();
    }

    pub fn apply(
        &self,
        target_dir: String,
        themes_dir: String,
        mode: Mode,
        accent: AccentsColor,
        theme_name: ThemeName,
    ) -> Result<(), std::io::Error> {
        let version = match self.version {
            ShellVersion::G46 => "46",
            ShellVersion::Unsupported => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unsupported shell version",
                ))
            }
        };

        let source_path = format!("{}/{}", self.source_dir, version);
        let target_path = match theme_name {
            ThemeName::Default => format!(
                "{}/{}-{:?}-{:?}",
                target_dir, self.preset.name, mode, accent
            ),
            ThemeName::Custom { ref name } => name.to_string(),
        };
        let theme_dir = match theme_name {
            ThemeName::Default => format!(
                "{}/{}-{:?}-{:?}",
                themes_dir, self.preset.name, mode, accent
            ),
            ThemeName::Custom { ref name } => name.to_string(),
        };

        if !std::path::Path::new(&target_path).exists() {
            std::fs::create_dir_all(&target_path).unwrap();
        }

        if !std::path::Path::new(&theme_dir).exists() {
            std::fs::create_dir_all(&theme_dir).unwrap();
        }

        std::fs::create_dir_all(&format!("{}/gnome-shell", theme_dir)).unwrap();

        let output = run_command(&format!("cp -r {}/* {}", source_path, target_path));

        if !output.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to copy files",
            ));
        }

        for entry in WalkDir::new(&target_path) {
            let entry = entry?;
            if entry.path().extension().unwrap_or_default() == "template" {
                let template = std::fs::read_to_string(entry.path())?;
                let rendered = self.preset.render_template(template, mode, accent);
                std::fs::write(entry.path().with_extension("scss"), rendered)?;
                std::fs::remove_file(entry.path().with_extension("template"))?;
            }
        }

        let css = from_path(
            &format!("{}/gnome-shell.scss", target_path),
            &grass::Options::default(),
        )
        .unwrap();
        std::fs::write(format!("{}/gnome-shell/gnome-shell.css", theme_dir), css)?;
        self.apply_gtk(mode, accent, theme_dir);

        return Ok(());
    }
}
