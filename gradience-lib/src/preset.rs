use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AccentsColor {
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Mode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GtkVersion {
    Gtk3,
    Gtk4,
}

fn default_version() -> String {
    "0.0.1".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,

    #[serde(default = "default_version")]
    pub version: String,

    #[serde(default)]
    pub author: Author,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub supported: Supported,
    #[serde(default)]
    pub license: License,
    pub variables: Variables,
    pub palette: Palette,
    #[serde(alias = "custom_css")]
    pub custom: Custom,
    #[serde(default)]
    pub shell: Shell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shell {
    #[serde(default = "Shell::default_bg_color")]
    pub bg_color: Variable,
    #[serde(default = "Shell::default_fg_color")]
    pub fg_color: Variable,
    #[serde(default = "Shell::default_system_bg_color")]
    pub system_bg_color: Variable,
    #[serde(default = "Shell::default_selected_bg_color")]
    pub selected_bg_color: Variable,
    #[serde(default = "Shell::default_selected_fg_color")]
    pub selected_fg_color: Variable,
    #[serde(default = "Shell::default_panel_bg_color")]
    pub panel_bg_color: Variable,
    #[serde(default = "Shell::default_osd_bg_color")]
    pub osd_bg_color: Variable,
    #[serde(default = "Shell::default_osd_fg_color")]
    pub osd_fg_color: Variable,
    #[serde(default = "Shell::default_system_fg_color")]
    pub system_fg_color: Variable,
    #[serde(default = "Shell::default_panel_fg_color")]
    pub panel_fg_color: Variable,
}

impl Default for Shell {
    fn default() -> Self {
        Shell {
            bg_color: Shell::default_bg_color(),
            fg_color: Shell::default_fg_color(),
            system_bg_color: Shell::default_system_bg_color(),
            system_fg_color: Shell::default_system_fg_color(),
            selected_bg_color: Shell::default_selected_bg_color(),
            selected_fg_color: Shell::default_selected_fg_color(),
            panel_bg_color: Shell::default_panel_bg_color(),
            panel_fg_color: Shell::default_panel_fg_color(),
            osd_bg_color: Shell::default_osd_bg_color(),
            osd_fg_color: Shell::default_osd_fg_color(),
        }
    }
}

impl Shell {
    fn default_bg_color() -> Variable {
        Variable::Mode {
            light: "#ffffff".to_string(),
            dark: "#1e1e1e".to_string(),
        }
    }

    fn default_panel_fg_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.8)".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_system_fg_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.8)".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_fg_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.8)".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_system_bg_color() -> Variable {
        Variable::Mode {
            light: "#fafafa".to_string(),
            dark: "#242424".to_string(),
        }
    }

    fn default_selected_bg_color() -> Variable {
        Variable::Mode {
            light: "#3584e4".to_string(),
            dark: "#78aeed".to_string(),
        }
    }

    fn default_selected_fg_color() -> Variable {
        Variable::Mode {
            light: "#ffffff".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_panel_bg_color() -> Variable {
        Variable::Mode {
            light: "#f2f2f2".to_string(),
            dark: "#2a2a2a".to_string(),
        }
    }

    fn default_osd_bg_color() -> Variable {
        Variable::Mode {
            light: "#f2f2f2".to_string(),
            dark: "#2a2a2a".to_string(),
        }
    }

    fn default_osd_fg_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.8)".to_string(),
            dark: "#ffffff".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    name: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    url: String,
}

impl Default for Author {
    fn default() -> Self {
        Author {
            name: "Anonymous".to_string(),
            email: "".to_string(),
            url: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Supported {
    gnome: String,
    adw: String,
    gtk: String,
    mode: Vec<String>,
    accents: Vec<String>,
}

impl Default for Supported {
    fn default() -> Self {
        Supported {
            gnome: ">46".to_string(),
            adw: ">1.5".to_string(),
            gtk: ">3.24".to_string(),
            mode: vec!["light".to_string(), "dark".to_string()],
            accents: vec![
                "blue".to_string(),
                "green".to_string(),
                "red".to_string(),
                "yellow".to_string(),
                "purple".to_string(),
                "pink".to_string(),
                "orange".to_string(),
                "slate".to_string(),
                "teal".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum License {
    GPL3,
    MIT,
    Apache2,
    BSD,
    LGPL3,
    AGPL3,
    MPL2,
    CC0,
    CCBY,
    CCBYSA,
    CCBYNC,
    CCBYNCSA,
    CCBYND,
    CCBYNCND,
}

impl Default for License {
    fn default() -> Self {
        License::GPL3
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variables {
    #[serde(default = "Variables::default_accent_color")]
    pub accent_color: Variable,
    #[serde(default = "Variables::default_accent_bg_color")]
    pub accent_bg_color: Variable,
    #[serde(default = "Variables::default_accent_fg_color")]
    pub accent_fg_color: Variable,
    #[serde(default = "Variables::default_destructive_color")]
    pub destructive_color: Variable,
    #[serde(default = "Variables::default_destructive_bg_color")]
    pub destructive_bg_color: Variable,
    #[serde(default = "Variables::default_destructive_fg_color")]
    pub destructive_fg_color: Variable,
    #[serde(default = "Variables::default_success_color")]
    pub success_color: Variable,
    #[serde(default = "Variables::default_success_bg_color")]
    pub success_bg_color: Variable,
    #[serde(default = "Variables::default_success_fg_color")]
    pub success_fg_color: Variable,
    #[serde(default = "Variables::default_warning_color")]
    pub warning_color: Variable,
    #[serde(default = "Variables::default_warning_bg_color")]
    pub warning_bg_color: Variable,
    #[serde(default = "Variables::default_warning_fg_color")]
    pub warning_fg_color: Variable,
    #[serde(default = "Variables::default_error_color")]
    pub error_color: Variable,
    #[serde(default = "Variables::default_error_bg_color")]
    pub error_bg_color: Variable,
    #[serde(default = "Variables::default_error_fg_color")]
    pub error_fg_color: Variable,
    #[serde(default = "Variables::default_window_bg_color")]
    pub window_bg_color: Variable,
    #[serde(default = "Variables::default_window_fg_color")]
    pub window_fg_color: Variable,
    #[serde(default = "Variables::default_view_bg_color")]
    pub view_bg_color: Variable,
    #[serde(default = "Variables::default_view_fg_color")]
    pub view_fg_color: Variable,
    #[serde(default = "Variables::default_headerbar_bg_color")]
    pub headerbar_bg_color: Variable,
    #[serde(default = "Variables::default_headerbar_fg_color")]
    pub headerbar_fg_color: Variable,
    #[serde(default = "Variables::default_headerbar_border_color")]
    pub headerbar_border_color: Variable,
    #[serde(default = "Variables::default_headerbar_backdrop_color")]
    pub headerbar_backdrop_color: Variable,
    #[serde(default = "Variables::default_headerbar_shade_color")]
    pub headerbar_shade_color: Variable,
    #[serde(default = "Variables::default_headerbar_darker_shade_color")]
    pub headerbar_darker_shade_color: Variable,
    #[serde(default = "Variables::default_card_bg_color")]
    pub card_bg_color: Variable,
    #[serde(default = "Variables::default_card_fg_color")]
    pub card_fg_color: Variable,
    #[serde(default = "Variables::default_card_shade_color")]
    pub card_shade_color: Variable,
    #[serde(default = "Variables::default_dialog_bg_color")]
    pub dialog_bg_color: Variable,
    #[serde(default = "Variables::default_dialog_fg_color")]
    pub dialog_fg_color: Variable,
    #[serde(default = "Variables::default_popover_bg_color")]
    pub popover_bg_color: Variable,
    #[serde(default = "Variables::default_popover_fg_color")]
    pub popover_fg_color: Variable,
    #[serde(default = "Variables::default_popover_shade_color")]
    pub popover_shade_color: Variable,
    #[serde(default = "Variables::default_shade_color")]
    pub shade_color: Variable,
    #[serde(default = "Variables::default_scrollbar_outline_color")]
    pub scrollbar_outline_color: Variable,
    #[serde(default = "Variables::default_thumbnail_bg_color")]
    pub thumbnail_bg_color: Variable,
    #[serde(default = "Variables::default_thumbnail_fg_color")]
    pub thumbnail_fg_color: Variable,
    #[serde(default = "Variables::default_sidebar_bg_color")]
    pub sidebar_bg_color: Variable,
    #[serde(default = "Variables::default_sidebar_fg_color")]
    pub sidebar_fg_color: Variable,
    #[serde(default = "Variables::default_sidebar_backdrop_color")]
    pub sidebar_backdrop_color: Variable,
    #[serde(default = "Variables::default_sidebar_shade_color")]
    pub sidebar_shade_color: Variable,
    #[serde(default = "Variables::default_secondary_sidebar_bg_color")]
    pub secondary_sidebar_bg_color: Variable,
    #[serde(default = "Variables::default_secondary_sidebar_fg_color")]
    pub secondary_sidebar_fg_color: Variable,
    #[serde(default = "Variables::default_secondary_sidebar_backdrop_color")]
    pub secondary_sidebar_backdrop_color: Variable,
    #[serde(default = "Variables::default_secondary_sidebar_shade_color")]
    pub secondary_sidebar_shade_color: Variable,
}

#[derive(Clone, Debug)]
pub enum VariablesName {
    AccentColor,
    AccentBgColor,
    AccentFgColor,
    DestructiveColor,
    DestructiveBgColor,
    DestructiveFgColor,
    SuccessColor,
    SuccessBgColor,
    SuccessFgColor,
    WarningColor,
    WarningBgColor,
    WarningFgColor,
    ErrorColor,
    ErrorBgColor,
    ErrorFgColor,
    WindowBgColor,
    WindowFgColor,
    ViewBgColor,
    ViewFgColor,
    HeaderbarBgColor,
    HeaderbarFgColor,
    HeaderbarBorderColor,
    HeaderbarBackdropColor,
    HeaderbarShadeColor,
    HeaderbarDarkerShadeColor,
    CardBgColor,
    CardFgColor,
    CardShadeColor,
    DialogBgColor,
    DialogFgColor,
    PopoverBgColor,
    PopoverFgColor,
    PopoverShadeColor,
    ShadeColor,
    ScrollbarOutlineColor,
    ThumbnailBgColor,
    ThumbnailFgColor,
    SidebarBgColor,
    SidebarFgColor,
    SidebarBackdropColor,
    SidebarShadeColor,
    SecondarySidebarBgColor,
    SecondarySidebarFgColor,
    SecondarySidebarBackdropColor,
    SecondarySidebarShadeColor,
}

impl VariablesName {
    pub fn to_slug(&self) -> String {
        match self {
            VariablesName::AccentColor => "accent_color".to_string(),
            VariablesName::AccentBgColor => "accent_bg_color".to_string(),
            VariablesName::AccentFgColor => "accent_fg_color".to_string(),
            VariablesName::DestructiveColor => "destructive_color".to_string(),
            VariablesName::DestructiveBgColor => "destructive_bg_color".to_string(),
            VariablesName::DestructiveFgColor => "destructive_fg_color".to_string(),
            VariablesName::SuccessColor => "success_color".to_string(),
            VariablesName::SuccessBgColor => "success_bg_color".to_string(),
            VariablesName::SuccessFgColor => "success_fg_color".to_string(),
            VariablesName::WarningColor => "warning_color".to_string(),
            VariablesName::WarningBgColor => "warning_bg_color".to_string(),
            VariablesName::WarningFgColor => "warning_fg_color".to_string(),
            VariablesName::ErrorColor => "error_color".to_string(),
            VariablesName::ErrorBgColor => "error_bg_color".to_string(),
            VariablesName::ErrorFgColor => "error_fg_color".to_string(),
            VariablesName::WindowBgColor => "window_bg_color".to_string(),
            VariablesName::WindowFgColor => "window_fg_color".to_string(),
            VariablesName::ViewBgColor => "view_bg_color".to_string(),
            VariablesName::ViewFgColor => "view_fg_color".to_string(),
            VariablesName::HeaderbarBgColor => "headerbar_bg_color".to_string(),
            VariablesName::HeaderbarFgColor => "headerbar_fg_color".to_string(),
            VariablesName::HeaderbarBorderColor => "headerbar_border_color".to_string(),
            VariablesName::HeaderbarBackdropColor => "headerbar_backdrop_color".to_string(),
            VariablesName::HeaderbarShadeColor => "headerbar_shade_color".to_string(),
            VariablesName::HeaderbarDarkerShadeColor => "headerbar_darker_shade_color".to_string(),
            VariablesName::CardBgColor => "card_bg_color".to_string(),
            VariablesName::CardFgColor => "card_fg_color".to_string(),
            VariablesName::CardShadeColor => "card_shade_color".to_string(),
            VariablesName::DialogBgColor => "dialog_bg_color".to_string(),
            VariablesName::DialogFgColor => "dialog_fg_color".to_string(),
            VariablesName::PopoverBgColor => "popover_bg_color".to_string(),
            VariablesName::PopoverFgColor => "popover_fg_color".to_string(),
            VariablesName::PopoverShadeColor => "popover_shade_color".to_string(),
            VariablesName::ShadeColor => "shade_color".to_string(),
            VariablesName::ScrollbarOutlineColor => "scrollbar_outline_color".to_string(),
            VariablesName::ThumbnailBgColor => "thumbnail_bg_color".to_string(),
            VariablesName::ThumbnailFgColor => "thumbnail_fg_color".to_string(),
            VariablesName::SidebarBgColor => "sidebar_bg_color".to_string(),
            VariablesName::SidebarFgColor => "sidebar_fg_color".to_string(),
            VariablesName::SidebarBackdropColor => "sidebar_backdrop_color".to_string(),
            VariablesName::SidebarShadeColor => "sidebar_shade_color".to_string(),
            VariablesName::SecondarySidebarBgColor => "secondary_sidebar_bg_color".to_string(),
            VariablesName::SecondarySidebarFgColor => "secondary_sidebar_fg_color".to_string(),
            VariablesName::SecondarySidebarBackdropColor => {
                "secondary_sidebar_backdrop_color".to_string()
            }
            VariablesName::SecondarySidebarShadeColor => {
                "secondary_sidebar_shade_color".to_string()
            }
        }
    }
}
pub struct VariablesIterator<'a> {
    variables: &'a Variables,
    name: VariablesName,
}

impl<'a> std::iter::Iterator for VariablesIterator<'a> {
    type Item = (VariablesName, &'a Variable);

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some((self.name.clone(), self.variables.get(&self.name)));
        self.name = match self.name {
            VariablesName::AccentColor => VariablesName::AccentBgColor,
            VariablesName::AccentBgColor => VariablesName::AccentFgColor,
            VariablesName::AccentFgColor => VariablesName::DestructiveColor,
            VariablesName::DestructiveColor => VariablesName::DestructiveBgColor,
            VariablesName::DestructiveBgColor => VariablesName::DestructiveFgColor,
            VariablesName::DestructiveFgColor => VariablesName::SuccessColor,
            VariablesName::SuccessColor => VariablesName::SuccessBgColor,
            VariablesName::SuccessBgColor => VariablesName::SuccessFgColor,
            VariablesName::SuccessFgColor => VariablesName::WarningColor,
            VariablesName::WarningColor => VariablesName::WarningBgColor,
            VariablesName::WarningBgColor => VariablesName::WarningFgColor,
            VariablesName::WarningFgColor => VariablesName::ErrorColor,
            VariablesName::ErrorColor => VariablesName::ErrorBgColor,
            VariablesName::ErrorBgColor => VariablesName::ErrorFgColor,
            VariablesName::ErrorFgColor => VariablesName::WindowBgColor,
            VariablesName::WindowBgColor => VariablesName::WindowFgColor,
            VariablesName::WindowFgColor => VariablesName::ViewBgColor,
            VariablesName::ViewBgColor => VariablesName::ViewFgColor,
            VariablesName::ViewFgColor => VariablesName::HeaderbarBgColor,
            VariablesName::HeaderbarBgColor => VariablesName::HeaderbarFgColor,
            VariablesName::HeaderbarFgColor => VariablesName::HeaderbarBorderColor,
            VariablesName::HeaderbarBorderColor => VariablesName::HeaderbarBackdropColor,
            VariablesName::HeaderbarBackdropColor => VariablesName::HeaderbarShadeColor,
            VariablesName::HeaderbarShadeColor => VariablesName::HeaderbarDarkerShadeColor,
            VariablesName::HeaderbarDarkerShadeColor => VariablesName::CardBgColor,
            VariablesName::CardBgColor => VariablesName::CardFgColor,
            VariablesName::CardFgColor => VariablesName::CardShadeColor,
            VariablesName::CardShadeColor => VariablesName::DialogBgColor,
            VariablesName::DialogBgColor => VariablesName::DialogFgColor,
            VariablesName::DialogFgColor => VariablesName::PopoverBgColor,
            VariablesName::PopoverBgColor => VariablesName::PopoverFgColor,
            VariablesName::PopoverFgColor => VariablesName::PopoverShadeColor,
            VariablesName::PopoverShadeColor => VariablesName::ShadeColor,
            VariablesName::ShadeColor => VariablesName::ScrollbarOutlineColor,
            VariablesName::ScrollbarOutlineColor => VariablesName::ThumbnailBgColor,
            VariablesName::ThumbnailBgColor => VariablesName::ThumbnailFgColor,
            VariablesName::ThumbnailFgColor => VariablesName::SidebarBgColor,
            VariablesName::SidebarBgColor => VariablesName::SidebarFgColor,
            VariablesName::SidebarFgColor => VariablesName::SidebarBackdropColor,
            VariablesName::SidebarBackdropColor => VariablesName::SidebarShadeColor,
            VariablesName::SidebarShadeColor => VariablesName::SecondarySidebarBgColor,
            VariablesName::SecondarySidebarBgColor => VariablesName::SecondarySidebarFgColor,
            VariablesName::SecondarySidebarFgColor => VariablesName::SecondarySidebarBackdropColor,
            VariablesName::SecondarySidebarBackdropColor => {
                VariablesName::SecondarySidebarShadeColor
            }
            VariablesName::SecondarySidebarShadeColor => return None,
        };
        result
    }
}

impl Variables {
    pub fn get(&self, name: &VariablesName) -> &Variable {
        match name {
            VariablesName::AccentColor => &self.accent_color,
            VariablesName::AccentBgColor => &self.accent_bg_color,
            VariablesName::AccentFgColor => &self.accent_fg_color,
            VariablesName::DestructiveColor => &self.destructive_color,
            VariablesName::DestructiveBgColor => &self.destructive_bg_color,
            VariablesName::DestructiveFgColor => &self.destructive_fg_color,

            VariablesName::SuccessColor => &self.success_color,
            VariablesName::SuccessBgColor => &self.success_bg_color,
            VariablesName::SuccessFgColor => &self.success_fg_color,

            VariablesName::WarningColor => &self.warning_color,
            VariablesName::WarningBgColor => &self.warning_bg_color,
            VariablesName::WarningFgColor => &self.warning_fg_color,

            VariablesName::ErrorColor => &self.error_color,
            VariablesName::ErrorBgColor => &self.error_bg_color,
            VariablesName::ErrorFgColor => &self.error_fg_color,

            VariablesName::WindowBgColor => &self.window_bg_color,
            VariablesName::WindowFgColor => &self.window_fg_color,

            VariablesName::ViewBgColor => &self.view_bg_color,
            VariablesName::ViewFgColor => &self.view_fg_color,

            VariablesName::HeaderbarBgColor => &self.headerbar_bg_color,
            VariablesName::HeaderbarFgColor => &self.headerbar_fg_color,
            VariablesName::HeaderbarBorderColor => &self.headerbar_border_color,
            VariablesName::HeaderbarBackdropColor => &self.headerbar_backdrop_color,
            VariablesName::HeaderbarShadeColor => &self.headerbar_shade_color,
            VariablesName::HeaderbarDarkerShadeColor => &self.headerbar_darker_shade_color,

            VariablesName::CardBgColor => &self.card_bg_color,
            VariablesName::CardFgColor => &self.card_fg_color,
            VariablesName::CardShadeColor => &self.card_shade_color,

            VariablesName::DialogBgColor => &self.dialog_bg_color,
            VariablesName::DialogFgColor => &self.dialog_fg_color,

            VariablesName::PopoverBgColor => &self.popover_bg_color,
            VariablesName::PopoverFgColor => &self.popover_fg_color,
            VariablesName::PopoverShadeColor => &self.popover_shade_color,

            VariablesName::ShadeColor => &self.shade_color,
            VariablesName::ScrollbarOutlineColor => &self.scrollbar_outline_color,

            VariablesName::ThumbnailBgColor => &self.thumbnail_bg_color,
            VariablesName::ThumbnailFgColor => &self.thumbnail_fg_color,

            VariablesName::SidebarBgColor => &self.sidebar_bg_color,
            VariablesName::SidebarFgColor => &self.sidebar_fg_color,
            VariablesName::SidebarBackdropColor => &self.sidebar_backdrop_color,
            VariablesName::SidebarShadeColor => &self.sidebar_shade_color,

            VariablesName::SecondarySidebarBgColor => &self.secondary_sidebar_bg_color,
            VariablesName::SecondarySidebarFgColor => &self.secondary_sidebar_fg_color,
            VariablesName::SecondarySidebarBackdropColor => &self.secondary_sidebar_backdrop_color,
            VariablesName::SecondarySidebarShadeColor => &self.secondary_sidebar_shade_color,
        }
    }

    pub fn iter(&self) -> VariablesIterator {
        VariablesIterator {
            variables: self,
            name: VariablesName::AccentColor,
        }
    }

    fn default_accent_color() -> Variable {
        Variable::Mode {
            light: "#3584e4".to_string(),
            dark: "#78aeed".to_string(),
        }
    }

    fn default_accent_bg_color() -> Variable {
        Variable::Mode {
            light: "#3584e4".to_string(),
            dark: "#3584e4".to_string(),
        }
    }

    fn default_accent_fg_color() -> Variable {
        Variable::Mode {
            light: "#ffffff".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_destructive_color() -> Variable {
        Variable::Mode {
            light: "#c01c28".to_string(),
            dark: "#ff7b63".to_string(),
        }
    }

    fn default_destructive_bg_color() -> Variable {
        Variable::Mode {
            light: "#e01b24".to_string(),
            dark: "#c01c28".to_string(),
        }
    }

    fn default_destructive_fg_color() -> Variable {
        Variable::Mode {
            light: "#ffffff".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_success_color() -> Variable {
        Variable::Mode {
            light: "#26a269".to_string(),
            dark: "#8ff0a4".to_string(),
        }
    }

    fn default_success_bg_color() -> Variable {
        Variable::Mode {
            light: "#2ec27e".to_string(),
            dark: "#26a269".to_string(),
        }
    }

    fn default_success_fg_color() -> Variable {
        Variable::Mode {
            light: "#ffffff".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_warning_color() -> Variable {
        Variable::Mode {
            light: "#ae7b03".to_string(),
            dark: "#f8e45c".to_string(),
        }
    }

    fn default_warning_bg_color() -> Variable {
        Variable::Mode {
            light: "#e5a50a".to_string(),
            dark: "#cd9309".to_string(),
        }
    }

    fn default_warning_fg_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.8)".to_string(),
            dark: "rgba(0, 0, 0, 0.8)".to_string(),
        }
    }

    fn default_error_color() -> Variable {
        Variable::Mode {
            light: "#c01c28".to_string(),
            dark: "#ff7b63".to_string(),
        }
    }

    fn default_error_bg_color() -> Variable {
        Variable::Mode {
            light: "#e01b24".to_string(),
            dark: "#c01c28".to_string(),
        }
    }

    fn default_error_fg_color() -> Variable {
        Variable::Mode {
            light: "#ffffff".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_window_bg_color() -> Variable {
        Variable::Mode {
            light: "#fafafa".to_string(),
            dark: "#242424".to_string(),
        }
    }

    fn default_window_fg_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.8)".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_view_bg_color() -> Variable {
        Variable::Mode {
            light: "#ffffff".to_string(),
            dark: "#1e1e1e".to_string(),
        }
    }

    fn default_view_fg_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.8)".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_headerbar_bg_color() -> Variable {
        Variable::Mode {
            light: "#ebebeb".to_string(),
            dark: "#303030".to_string(),
        }
    }

    fn default_headerbar_fg_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.8)".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_headerbar_border_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.8)".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_headerbar_backdrop_color() -> Variable {
        Variable::Mode {
            light: "@window_bg_color".to_string(),
            dark: "@window_bg_color".to_string(),
        }
    }

    fn default_headerbar_shade_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.07)".to_string(),
            dark: "rgba(0, 0, 0, 0.36)".to_string(),
        }
    }

    fn default_headerbar_darker_shade_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.12)".to_string(),
            dark: "rgba(0, 0, 0, 0.9)".to_string(),
        }
    }

    fn default_card_bg_color() -> Variable {
        Variable::Mode {
            light: "#ffffff".to_string(),
            dark: "rgba(255, 255, 255, 0.08)".to_string(),
        }
    }

    fn default_card_fg_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.8)".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_card_shade_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.07)".to_string(),
            dark: "rgba(0, 0, 0, 0.36)".to_string(),
        }
    }

    fn default_dialog_bg_color() -> Variable {
        Variable::Mode {
            light: "#fafafa".to_string(),
            dark: "#383838".to_string(),
        }
    }

    fn default_dialog_fg_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.8)".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_popover_bg_color() -> Variable {
        Variable::Mode {
            light: "#ffffff".to_string(),
            dark: "#383838".to_string(),
        }
    }

    fn default_popover_fg_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.8)".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_popover_shade_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.07)".to_string(),
            dark: "rgba(0, 0, 0, 0.36)".to_string(),
        }
    }

    fn default_shade_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.07)".to_string(),
            dark: "rgba(0, 0, 0, 0.36)".to_string(),
        }
    }

    fn default_scrollbar_outline_color() -> Variable {
        Variable::Mode {
            light: "#ffffff".to_string(),
            dark: "rgba(0, 0, 0, 0.5)".to_string(),
        }
    }

    fn default_thumbnail_bg_color() -> Variable {
        Variable::Mode {
            light: "#ffffff".to_string(),
            dark: "#383838".to_string(),
        }
    }

    fn default_thumbnail_fg_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.8)".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_sidebar_bg_color() -> Variable {
        Variable::Mode {
            light: "#ebebeb".to_string(),
            dark: "#303030".to_string(),
        }
    }

    fn default_sidebar_fg_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.8)".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_sidebar_backdrop_color() -> Variable {
        Variable::Mode {
            light: "#f2f2f2".to_string(),
            dark: "#2a2a2a".to_string(),
        }
    }

    fn default_sidebar_shade_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.07)".to_string(),
            dark: "rgba(0, 0, 0, 0.36)".to_string(),
        }
    }

    fn default_secondary_sidebar_bg_color() -> Variable {
        Variable::Mode {
            light: "#f3f3f3".to_string(),
            dark: "#2a2a2a".to_string(),
        }
    }

    fn default_secondary_sidebar_fg_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.8)".to_string(),
            dark: "#ffffff".to_string(),
        }
    }

    fn default_secondary_sidebar_backdrop_color() -> Variable {
        Variable::Mode {
            light: "#f6f6f6".to_string(),
            dark: "#272727".to_string(),
        }
    }

    fn default_secondary_sidebar_shade_color() -> Variable {
        Variable::Mode {
            light: "rgba(0, 0, 0, 0.07)".to_string(),
            dark: "rgba(0, 0, 0, 0.36)".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Variable {
    Single(String),
    Mode {
        light: String,
        dark: String,
    },
    Accent {
        #[serde(default)]
        blue: String,
        #[serde(default)]
        green: String,
        #[serde(default)]
        red: String,
        #[serde(default)]
        yellow: String,
        #[serde(default)]
        purple: String,
        #[serde(default)]
        pink: String,
        #[serde(default)]
        orange: String,
        #[serde(default)]
        slate: String,
        #[serde(default)]
        teal: String,
        default: String,
    },
    ModeAccent {
        light: HashMap<String, String>,
        dark: HashMap<String, String>,
    },
}

impl Variable {
    pub fn get(&self, mode: &str, accent: &str) -> String {
        match self {
            Variable::Single(s) => s.to_string(),
            Variable::Mode { light, dark } => {
                if mode == "light" {
                    light.to_string()
                } else {
                    dark.to_string()
                }
            }
            Variable::Accent {
                blue,
                green,
                red,
                yellow,
                purple,
                pink,
                orange,
                slate,
                teal,
                default,
            } => match accent {
                "blue" => {
                    if blue.is_empty() {
                        default.to_string()
                    } else {
                        blue.to_string()
                    }
                }
                "green" => {
                    if green.is_empty() {
                        default.to_string()
                    } else {
                        green.to_string()
                    }
                }
                "red" => {
                    if red.is_empty() {
                        default.to_string()
                    } else {
                        red.to_string()
                    }
                }
                "yellow" => {
                    if yellow.is_empty() {
                        default.to_string()
                    } else {
                        yellow.to_string()
                    }
                }
                "purple" => {
                    if purple.is_empty() {
                        default.to_string()
                    } else {
                        purple.to_string()
                    }
                }
                "pink" => {
                    if pink.is_empty() {
                        default.to_string()
                    } else {
                        pink.to_string()
                    }
                }
                "orange" => {
                    if orange.is_empty() {
                        default.to_string()
                    } else {
                        orange.to_string()
                    }
                }
                "slate" => {
                    if slate.is_empty() {
                        default.to_string()
                    } else {
                        slate.to_string()
                    }
                }
                "teal" => {
                    if teal.is_empty() {
                        default.to_string()
                    } else {
                        teal.to_string()
                    }
                }
                _ => default.to_string(),
            },
            Variable::ModeAccent { light, dark } => {
                if mode == "light" {
                    if light.contains_key(accent) {
                        light[accent].to_string()
                    } else {
                        light["default"].to_string()
                    }
                } else {
                    if dark.contains_key(accent) {
                        dark[accent].to_string()
                    } else {
                        dark["default"].to_string()
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
    #[serde(alias = "blue_")]
    pub blue: HashMap<String, String>,
    #[serde(alias = "green_")]
    pub green: HashMap<String, String>,
    #[serde(alias = "yellow_")]
    pub yellow: HashMap<String, String>,
    #[serde(alias = "orange_")]
    pub orange: HashMap<String, String>,
    #[serde(alias = "red_")]
    pub red: HashMap<String, String>,
    #[serde(alias = "purple_")]
    pub purple: HashMap<String, String>,
    #[serde(alias = "brown_")]
    pub brown: HashMap<String, String>,
    #[serde(alias = "light_")]
    pub light: HashMap<String, String>,
    #[serde(alias = "dark_")]
    pub dark: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Custom {
    #[serde(default)]
    pub gtk4: String,
    #[serde(default)]
    pub gtk3: String,
    #[serde(default)]
    pub shell: String,
}

impl Preset {
    pub fn from_str(s: &str) -> Preset {
        let p: Preset = serde_json::from_str(s).unwrap();
        p
    }

    pub fn to_str(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_file(file: &str) -> Preset {
        let s = std::fs::read_to_string(file).unwrap();
        Preset::from_str(&s)
    }

    pub fn to_file(&self, file: &str) {
        let s = self.to_str();
        std::fs::write(file, s).unwrap();
    }

    pub fn to_css(&self, mode: Mode, accent: AccentsColor, gtk: GtkVersion) -> String {
        let mut css = match gtk {
            GtkVersion::Gtk3 => self.custom.gtk3.clone(),
            GtkVersion::Gtk4 => self.custom.gtk4.clone(),
        };

        let mode = match mode {
            Mode::Light => "light",
            Mode::Dark => "dark",
        };

        let accent = match accent {
            AccentsColor::Blue => "blue",
            AccentsColor::Green => "green",
            AccentsColor::Yellow => "yellow",
            AccentsColor::Orange => "orange",
            AccentsColor::Red => "red",
            AccentsColor::Purple => "purple",
            AccentsColor::Pink => "pink",
            AccentsColor::Slate => "slate",
            AccentsColor::Teal => "teal",
        };

        css += &format!(
            "/* Preset: {}@{} {}/{} */\n",
            self.name, self.version, mode, accent
        );

        for (name, variable) in self.variables.iter() {
            let value = variable.get(mode, accent);
            css += &format!("@define-color {} {};\n", name.to_slug(), value);
        }

        for (name, value) in self.palette.blue.iter() {
            css += &format!("@define-color blue_{} {};\n", name, value);
        }

        for (name, value) in self.palette.green.iter() {
            css += &format!("@define-color green_{} {};\n", name, value);
        }

        for (name, value) in self.palette.yellow.iter() {
            css += &format!("@define-color yellow_{} {};\n", name, value);
        }

        for (name, value) in self.palette.orange.iter() {
            css += &format!("@define-color orange_{} {};\n", name, value);
        }

        for (name, value) in self.palette.red.iter() {
            css += &format!("@define-color red_{} {};\n", name, value);
        }

        for (name, value) in self.palette.purple.iter() {
            css += &format!("@define-color purple_{} {};\n", name, value);
        }

        for (name, value) in self.palette.brown.iter() {
            css += &format!("@define-color brown_{} {};\n", name, value);
        }

        for (name, value) in self.palette.light.iter() {
            css += &format!("@define-color light_{} {};\n", name, value);
        }

        for (name, value) in self.palette.dark.iter() {
            css += &format!("@define-color dark_{} {};\n", name, value);
        }

        css = css.replace("@mode", mode);
        css = css.replace("@accent", accent);
        css
    }

    pub fn render_template(&self, template: String, mode: Mode, accent: AccentsColor) -> String {
        let reg = Handlebars::new();

        let mode = match mode {
            Mode::Light => "light",
            Mode::Dark => "dark",
        };

        let accent = match accent {
            AccentsColor::Blue => "blue",
            AccentsColor::Green => "green",
            AccentsColor::Yellow => "yellow",
            AccentsColor::Orange => "orange",
            AccentsColor::Red => "red",
            AccentsColor::Purple => "purple",
            AccentsColor::Pink => "pink",
            AccentsColor::Slate => "slate",
            AccentsColor::Teal => "teal",
        };

        let mut data = BTreeMap::new();
        data.insert(
            "bg_color".to_string(),
            self.shell.bg_color.get(mode, accent),
        );
        data.insert(
            "fg_color".to_string(),
            self.shell.fg_color.get(mode, accent),
        );
        data.insert(
            "system_bg_color".to_string(),
            self.shell.system_bg_color.get(mode, accent),
        );
        data.insert(
            "system_fg_color".to_string(),
            self.shell.system_fg_color.get(mode, accent),
        );
        data.insert(
            "selected_bg_color".to_string(),
            self.shell.selected_bg_color.get(mode, accent),
        );
        data.insert(
            "selected_fg_color".to_string(),
            self.shell.selected_fg_color.get(mode, accent),
        );
        data.insert(
            "panel_bg_color".to_string(),
            self.shell.panel_bg_color.get(mode, accent),
        );
        data.insert(
            "panel_fg_color".to_string(),
            self.shell.panel_fg_color.get(mode, accent),
        );
        data.insert(
            "osd_bg_color".to_string(),
            self.shell.osd_bg_color.get(mode, accent),
        );
        data.insert(
            "osd_fg_color".to_string(),
            self.shell.osd_fg_color.get(mode, accent),
        );

        data.insert("name".to_string(), self.name.clone());
        data.insert("version".to_string(), self.version.clone());

        // insert all variables
        for (name, variable) in self.variables.iter() {
            data.insert(name.to_slug(), variable.get(mode, accent));
        }

        // insert all palette colors
        for (name, value) in self.palette.blue.iter() {
            data.insert(format!("blue_{}", name), value.to_string());
        }

        for (name, value) in self.palette.green.iter() {
            data.insert(format!("green_{}", name), value.to_string());
        }

        for (name, value) in self.palette.yellow.iter() {
            data.insert(format!("yellow_{}", name), value.to_string());
        }

        for (name, value) in self.palette.orange.iter() {
            data.insert(format!("orange_{}", name), value.to_string());
        }

        for (name, value) in self.palette.red.iter() {
            data.insert(format!("red_{}", name), value.to_string());
        }

        for (name, value) in self.palette.purple.iter() {
            data.insert(format!("purple_{}", name), value.to_string());
        }

        for (name, value) in self.palette.brown.iter() {
            data.insert(format!("brown_{}", name), value.to_string());
        }

        for (name, value) in self.palette.light.iter() {
            data.insert(format!("light_{}", name), value.to_string());
        }

        for (name, value) in self.palette.dark.iter() {
            data.insert(format!("dark_{}", name), value.to_string());
        }

        data.insert("custom_css".to_string(), self.custom.shell.clone());
        data.insert("mode".to_string(), mode.to_string());
        data.insert("accent".to_string(), accent.to_string());

        let mut result = reg.render_template(&template, &data).unwrap();

        result = result.replace("@mode", mode);
        result = result.replace("@accent", &accent.to_string());
        result
    }
}

pub struct ApplyBuilder {
    preset: Preset,
    gtk3_path: String,
    gtk4_path: String,
    mode: Mode,
    accent: AccentsColor,
}

impl ApplyBuilder {
    pub fn new(preset: Preset) -> ApplyBuilder {
        ApplyBuilder {
            preset,
            gtk3_path: "".to_string(),
            gtk4_path: "".to_string(),
            mode: Mode::Light,
            accent: AccentsColor::Blue,
        }
    }

    pub fn gtk3_path(mut self, path: &str) -> ApplyBuilder {
        self.gtk3_path = path.to_string();
        self
    }

    pub fn gtk4_path(mut self, path: &str) -> ApplyBuilder {
        self.gtk4_path = path.to_string();
        self
    }

    pub fn mode(mut self, mode: Mode) -> ApplyBuilder {
        self.mode = mode;
        self
    }

    pub fn accent(mut self, accent: AccentsColor) -> ApplyBuilder {
        self.accent = accent;
        self
    }

    pub fn apply(&self) {
        // check if the paths exists, if not try to make directories
        if !self.gtk3_path.is_empty() {
            let path = std::path::Path::new(&self.gtk3_path);
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent).unwrap();
                }
            }
        }

        if !self.gtk4_path.is_empty() {
            let path = std::path::Path::new(&self.gtk4_path);
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent).unwrap();
                }
            }
        }

        let css = self.preset.to_css(self.mode, self.accent, GtkVersion::Gtk3);
        std::fs::write(&self.gtk3_path, css).unwrap();
        let css = self.preset.to_css(self.mode, self.accent, GtkVersion::Gtk4);
        std::fs::write(&self.gtk4_path, css).unwrap();
    }
}
