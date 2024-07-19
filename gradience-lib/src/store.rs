use crate::preset::Preset;
#[cfg(feature = "online")]
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use serde::Deserialize;

const GH_API_URL: &str =
    "https://api.github.com/repos/t-dantiau/community/git/trees/main?recursive=1";

pub struct Store {
    pub base_path: String,
    presets: Vec<Preset>,
}

#[derive(Deserialize, Debug)]
pub struct GhApiUrlResponse {
    pub tree: Vec<Tree>,
    pub url: String,
    pub sha: String,
    pub truncated: bool,
}

#[derive(Deserialize, Debug)]
pub struct Tree {
    pub path: String,
    pub mode: String,

    #[serde(rename(deserialize = "type"))]
    pub type_: String,
    pub sha: String,
    pub size: u32,
    pub url: String,
}

impl Store {
    pub fn new(base_path: String) -> Store {
        if !std::path::Path::new(&base_path).exists() {
            std::fs::create_dir_all(&base_path).unwrap();
        }

        Store {
            base_path,
            presets: Vec::new(),
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

    #[cfg(feature = "online")]
    fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
        headers
    }

    #[cfg(feature = "online")]
    pub fn list_online_presets(&self) -> Vec<String> {
        let client = reqwest::blocking::Client::new();
        let res: GhApiUrlResponse = client
            .get(GH_API_URL)
            .headers(Store::construct_headers())
            .send()
            .unwrap()
            .json()
            .unwrap();
        println!("Online presets:");

        let mut online_presets = Vec::new();

        for tree in res.tree {
            let url = format!(
                "https://github.com/t-dantiau/Community/raw/main/{}",
                tree.path
            );
            online_presets.push(url);
        }
        online_presets
    }

    #[cfg(feature = "online")]
    pub fn download_online_preset(&self, name: String) -> Result<Preset, reqwest::Error> {
        let client = reqwest::blocking::Client::new();

        let url = format!(
            "https://github.com/t-dantiau/Community/raw/main/{}.json",
            name
        );

        let mut resp = client
            .get(&url)
            .headers(Store::construct_headers())
            .send()
            .unwrap();
        let content: Preset = resp.json().unwrap();
        Ok(content)
    }

    pub fn list_local_presets(&self) -> Vec<String> {
        let mut local_presets = Vec::new();
        for preset in &self.presets {
            local_presets.push(format!("{}/{}.json", self.base_path, preset.name));
        }
        local_presets
    }
}
