
mod config;
pub trait Store {
    fn new(input_path: Option<String>) -> Self;
    fn open_config(&self) -> Result<config::Configuration, Box<dyn std::error::Error>>;
    fn write_config(&self,config: config::Configuration) -> Result<(),Box<dyn std::error::Error>>;
    fn purge_config(&self) -> Result<(),std::io::Error>;
    fn get(&self, key: &str) -> Option<String>;
    fn set(&self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn remove(&self, key: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct Storage {
    pub storage_path : String
}

impl Store for Storage {
    fn new(input_path: Option<String>) -> Self {
        let mut path = format!("{}/.noted", std::env::var("HOME").unwrap());
        if input_path.is_some() {
            // set the path to input path
            path = input_path.unwrap();
        }
        // check if file exists
        if !std::path::Path::new(&path).exists() {
            // create the file
            std::fs::File::create(&path).unwrap();
        }
        let metadata = std::fs::metadata(&path).unwrap();
                if metadata.len() == 0 {
                    // write a default configuration object into it
                    let configuration = config::Configuration {
                        data: std::collections::HashMap::new()
                    };
                    let serialized = serde_json::to_string(&configuration).unwrap();
                    std::fs::write(&path, serialized).unwrap();
                }    
        Storage {
            storage_path: path.clone()
        }
    }
    fn write_config(&self,config: config::Configuration) -> Result<(),Box<dyn std::error::Error>> {
        let serialized = serde_json::to_string(&config)?;
        std::fs::write(&self.storage_path, serialized)?;
        Ok(())
    }
    fn purge_config(&self) -> Result<(),std::io::Error> {
        std::fs::remove_file(&self.storage_path)
    }
    fn open_config(&self) -> Result<config::Configuration, Box<dyn std::error::Error>> {
        let configuration = std::fs::read_to_string(&self.storage_path)?;
        let deserialized: config::Configuration = serde_json::from_str(&configuration)?;
        Ok(deserialized)
    }
    fn get(&self, key: &str) -> Option<String> {
        let configuration = self.open_config();
        match configuration {
            Ok(config) => {      
                return config.data.get(key).cloned();
            },
            Err(_) => {
                return None;
            }
        }
    }
    fn remove(&self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut configuration = self.open_config().unwrap();
        configuration.data.remove(key);
        self.write_config(configuration)   
    }
    fn set(&self, key: &str, value: &str)-> Result<(), Box<dyn std::error::Error>>{
        let mut configuration = self.open_config().unwrap();
        configuration.data.insert(key.to_string(), value.to_string());
        self.write_config(configuration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_new() {
        let temp_file = std::env::temp_dir().join("test_storage_new");
        let temp_file = temp_file.to_str().unwrap().to_string();
        let storage = Storage::new(Some(temp_file.clone()));
        assert_eq!(storage.storage_path, temp_file);
    }
    // try getting a key that doesn't exist
    #[test]
    fn test_storage_set_get() {
        let temp_file = std::env::temp_dir().join("test_storage_set_get");
        let temp_file = temp_file.to_str().unwrap().to_string();
        let storage = Storage::new(Some(temp_file));
        storage.set("test", "value").unwrap();
        assert_eq!(storage.get("test").unwrap(), "value");
        storage.purge_config().unwrap();
    }
    // Add a key then remove it
    #[test]
    fn test_storage_remove() {
        let temp_file = std::env::temp_dir().join("test_storage_remove");
        let temp_file = temp_file.to_str().unwrap().to_string();
        let storage = Storage::new(Some(temp_file));
        storage.set("test", "value").unwrap();
        storage.remove("test").unwrap();
        assert_eq!(storage.get("test"), None);
        storage.purge_config().unwrap();
    }
}
