#[cfg(target_arch = "wasm32")]
mod platform {
    fn storage() -> Option<web_sys::Storage> {
        web_sys::window()?.local_storage().ok()?
    }

    pub fn load(key: &str) -> Option<String> {
        storage()?.get_item(key).ok()?
    }

    pub fn save(key: &str, value: &str) {
        if let Some(storage) = storage() {
            let _ = storage.set_item(key, value);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    use std::path::PathBuf;

    #[cfg(target_os = "android")]
    fn data_dir() -> PathBuf {
        PathBuf::from("/data/data/com.brenoworker.educabrasil/files")
    }

    #[cfg(target_os = "ios")]
    fn data_dir() -> PathBuf {
        std::env::var("HOME")
            .map(|home| PathBuf::from(home).join("Documents"))
            .unwrap_or_else(|_| std::env::temp_dir())
    }

    #[cfg(target_os = "windows")]
    fn data_dir() -> PathBuf {
        std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir())
            .join("EducaBrasil")
    }

    #[cfg(target_os = "macos")]
    fn data_dir() -> PathBuf {
        std::env::var("HOME")
            .map(|home| PathBuf::from(home).join("Library/Application Support/EducaBrasil"))
            .unwrap_or_else(|_| std::env::temp_dir().join("EducaBrasil"))
    }

    #[cfg(not(any(
        target_os = "android",
        target_os = "ios",
        target_os = "windows",
        target_os = "macos"
    )))]
    fn data_dir() -> PathBuf {
        std::env::var("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|_| std::env::var("HOME").map(|home| PathBuf::from(home).join(".local/share")))
            .unwrap_or_else(|_| std::env::temp_dir())
            .join("educa-brasil")
    }

    fn path(key: &str) -> PathBuf {
        data_dir().join(format!("{key}.json"))
    }

    pub fn load(key: &str) -> Option<String> {
        std::fs::read_to_string(path(key)).ok()
    }

    pub fn save(key: &str, value: &str) {
        let target = path(key);
        if let Some(parent) = target.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(target, value);
    }
}

pub use platform::{load, save};
