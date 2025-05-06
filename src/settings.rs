pub mod settings {
    use crate::data::Settings;
    use crate::util::ensure;
    use crate::APP_NAME;
    use config::{Config, File};
    use std::path::PathBuf;
    use std::sync::{OnceLock, RwLock};

    const CONFIG_FILENAME: &str = "config.toml";
    static DEFAULT_CONFIG: &str = include_str!("../resources/config.default.toml");
    static CONFIG: OnceLock<RwLock<Settings>> = OnceLock::new();
    static PATH: OnceLock<PathBuf> = OnceLock::new();

    pub fn init(path: Option<String>) {
        let path = path
            .map(|it| PathBuf::from(it))
            .map(|it| {
                ensure(it.exists(), "Config file does not exist.");
                ensure(!it.is_dir(), "Config file cannot be a directory.");

                it
            })
            .unwrap_or_else(|| find_config_path());

        PATH.set(path).expect("Failed to set config path.");
    }

    fn find_config_path() -> PathBuf {
        let local = dirs::config_dir()
            .unwrap()
            .join(APP_NAME)
            .join(CONFIG_FILENAME);
        let system = PathBuf::from("/etc").join(APP_NAME).join(CONFIG_FILENAME);

        let path = [&local, &system]
            .into_iter()
            .find(|path| path.exists())
            .unwrap_or_else(|| &local);

        (*path).clone()
    }

    pub fn get_path() -> &'static PathBuf {
        PATH.get().unwrap()
    }

    fn parse_config() -> Settings {
        let path = get_path();
        if !path.exists() {
            std::fs::create_dir_all(path.parent().unwrap())
                .expect("Failed to create folders for default configuration.");
            std::fs::write(path.clone(), DEFAULT_CONFIG)
                .expect("Failed to write default configuration.");
        }

        let config = Config::builder()
            .add_source(File::from(path.clone()))
            .build()
            .unwrap();

        config.try_deserialize::<Settings>().unwrap_or_else(|e| {
            eprintln!("Error parsing config: {}", e);
            Settings::default()
        })
    }

    fn config_instance() -> &'static RwLock<Settings> {
        CONFIG.get_or_init(|| {
            let settings = parse_config();
            RwLock::new(settings)
        })
    }

    pub fn refresh() {
        *config_instance().write().unwrap() = parse_config()
    }

    pub fn get() -> Settings {
        config_instance().read().unwrap().clone()
    }
}
