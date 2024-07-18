use crate::preset::Preset;

pub struct Store {
    pub base_path: String,
    presets: Vec<Preset>
}

impl Store {
    pub fn new(base_path: String) -> Store {
        if !std::path::Path::new(&base_path).exists() {
            std::fs::create_dir_all(&base_path).unwrap();
        }

        Store {
            base_path,
            presets: Vec::new()
        }
    }

    pub fn load(&mut self) {
        let paths = std::fs::read_dir(&self.base_path).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            let preset = Preset::from_file(&path.to_str().unwrap());
            self.presets.push(preset);
        }
    }

    pub fn add_preset(&mut self, preset: Preset) {
        self.presets.push(preset);
    }

    pub fn remove_preset(&mut self, name: String) {
        self.presets.retain(|p| p.name != name);

        let path = format!("{}/{}.json", self.base_path, name);
        std::fs::remove_file(path).unwrap();
    }

    pub fn get_preset(&self, name: String) -> Option<&Preset> {
        self.presets.iter().find(|p| p.name == name)
    }

    pub fn save_presets(&self) {
        for preset in &self.presets {
            let path = format!("{}/{}.json", self.base_path, preset.name);
            preset.to_file(&path);

        }
    }
}