use clap::{Parser, Subcommand, ValueEnum};
use gradience_lib::preset::{AccentsColor as GAccentsColor, Mode as GMode};
use gradience_lib::preset::{ApplyBuilder, Preset};
use gradience_lib::shell::Shell;
use gradience_lib::store::Store;
use include_dir::{include_dir, Dir};
extern crate shellexpand;

static SHELL_SOURCE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../shell");
const DEFAULT_STORE_PATH: &str = "./store";

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Light,
    Dark,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum AccentsColor {
    Blue,
    Teal,
    Green,
    Yellow,
    Orange,
    Red,
    Pink,
    Purple,
    Slate,
}

impl From<Mode> for GMode {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Light => GMode::Light,
            Mode::Dark => GMode::Dark,
        }
    }
}

impl From<AccentsColor> for GAccentsColor {
    fn from(accent: AccentsColor) -> Self {
        match accent {
            AccentsColor::Blue => GAccentsColor::Blue,
            AccentsColor::Teal => GAccentsColor::Teal,
            AccentsColor::Green => GAccentsColor::Green,
            AccentsColor::Yellow => GAccentsColor::Yellow,
            AccentsColor::Orange => GAccentsColor::Orange,
            AccentsColor::Red => GAccentsColor::Red,
            AccentsColor::Pink => GAccentsColor::Pink,
            AccentsColor::Purple => GAccentsColor::Purple,
            AccentsColor::Slate => GAccentsColor::Slate,
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, name="Gradience CLI φοῖνιξ", about="Change the look of GNOME, with ease", long_about=None)]
struct Args {
    /// The mode of the theme
    #[arg(short, long)]
    mode: Option<Mode>,

    /// The accent color of the theme
    #[arg(short, long)]
    accent: Option<AccentsColor>,

    /// The path to the store where presets are stored
    #[arg(short, long)]
    store: Option<String>,

    /// The path to the shell source directory which contains templates for the shell theme
    #[arg(long)]
    shell_source: Option<String>,

    /// The name of the preset to apply, it's not the filename
    #[arg(short, long)]
    preset: Option<String>,

    #[command(subcommand)]
    command: Commands,

    #[arg(long, hide = true)]
    markdown_help: bool,

    #[arg(long)]
    gtk3_path: Option<String>,

    #[arg(long)]
    gtk4_path: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Apply the theme to the shell, it will create a new theme with a shell theme and GTK theme
    Shell {
        /// The path to the temporary directory where the theme will be created and build
        #[arg(long)]
        temp_dir: Option<String>,
        /// The path to the directory where the theme will be stored
        #[arg(long)]
        theme_dir: Option<String>,

        #[arg(short, long)]
        gsettings: bool,
    },
    /// Apply the theme to the GTK theme
    Gtk,
    /// Manage the store of presets
    Store {
        #[command(subcommand)]
        command: StoreCommands,
    },
}

#[derive(Subcommand, Debug)]
enum StoreCommands {
    /// Add a new preset to the store
    Add {
        /// The path to the preset file
        path: String,
    },
    /// Remove a preset from the store
    Remove {
        /// The name of the preset to remove
        name: String,
    },
    /// List the online presets
    OnlineList,
    /// List the local presets
    LocalList,
    /// Download a preset from the online store
    Download {
        /// The name of the preset to download
        name: String,
    },
}

fn main() {
    let args: Args = Args::parse();

    if args.markdown_help {
        clap_markdown::print_help_markdown::<Args>();
        return;
    }

    let mut store = Store::new(match args.store {
        Some(path) => path,
        None => DEFAULT_STORE_PATH.to_string(),
    });
    store.load();

    // copy everything from the included shell source to the current directory
    SHELL_SOURCE
        .extract(format!(
            "{}/shell",
            std::env::temp_dir().to_str().unwrap().to_string()
        ))
        .unwrap();

    match &args.command {
        Commands::Shell {
            temp_dir,
            theme_dir,
            gsettings,
        } => {
            Shell::new(
                match args.shell_source {
                    Some(path) => path,
                    None => format!(
                        "{}/shell",
                        std::env::temp_dir().to_str().unwrap().to_string()
                    ),
                },
                store
                    .get_preset(args.preset.expect("please provide --preset argument"))
                    .unwrap_or_else(|| {
                        panic!(
                            "Unable to find this preset, use the name of the preset, not the filename"
                        );
                    })
                    .clone(),
            )
            .apply(
                match temp_dir {
                    Some(dir) => dir.to_string(),
                    None => std::env::temp_dir().to_str().unwrap().to_string(),
                },
                match theme_dir {
                    Some(dir) => dir.to_string(),
                    None => shellexpand::tilde("~/.themes").to_string(),
                },
                args.mode.unwrap_or(Mode::Light).into(),
                args.accent.unwrap_or(AccentsColor::Blue).into(),
                gradience_lib::shell::ThemeName::Default,
                *gsettings,
            )
            .unwrap();
        }
        Commands::Gtk => {
            ApplyBuilder::new(
                store
                    .get_preset(args.preset.expect("please provide --preset argument"))
                    .unwrap_or_else(|| {
                        panic!("Unable to find this preset, use the name of the preset, not the filename");
                    })
                    .clone(),
            )
            .mode(args.mode.unwrap_or(Mode::Light).into())
            .accent(args.accent.unwrap_or(AccentsColor::Blue).into())
            .gtk3_path(
                shellexpand::tilde(args.gtk3_path.unwrap_or("~/.config/gtk-3.0/gtk.css".to_string()).as_str())
                    .to_string()
                    .as_str(),
            )
            .gtk4_path(
                shellexpand::tilde(args.gtk4_path.unwrap_or("~/.config/gtk-4.0/gtk.css".to_string()).as_str())
                                    .to_string()
                                    .as_str(),
            )
            .apply();
        }
        Commands::Store { command } => match command {
            StoreCommands::Add { path } => {
                let preset: Preset = Preset::from_file(path);
                store.add_preset(preset);
                store.save_presets();
            }
            StoreCommands::Remove { name } => {
                store.remove_preset(name.to_string());
            }
            StoreCommands::OnlineList => {
                for p in store.list_online_presets() {
                    println!("- {}", p);
                }
            }
            StoreCommands::Download { name } => {
                match store.download_online_preset(name.to_string()) {
                    Ok(preset) => store.add_preset(preset),
                    Err(e) => println!("Error downloading {}: {}", name, e),
                }
                store.save_presets();
            }
            StoreCommands::LocalList => {
                for p in store.list_local_presets() {
                    println!("- {}", p);
                }
            }
        },
    }
}
