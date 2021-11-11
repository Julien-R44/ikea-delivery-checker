use rustbreak::{deser::Ron, FileDatabase as _FileDatabase, RustbreakError};
use std::collections::HashMap;

pub struct FileDatabase {
    db: _FileDatabase<HashMap<String, bool>, Ron>,
}

impl FileDatabase {
    pub fn new(filename: Option<&str>) -> Result<FileDatabase, Box<dyn std::error::Error>> {
        let db = _FileDatabase::<HashMap<String, bool>, Ron>::load_from_path_or_default(
            filename.unwrap_or("default.db"),
        )?;
        db.load()?;

        return Ok(FileDatabase { db });
    }

    pub fn get_key_value(&self, key: &str) -> Result<bool, RustbreakError> {
        return self.db.read(|db| {
            let key_value = db.get_key_value(key);
            if key_value.is_none() {
                return false;
            }
            return *key_value.unwrap().1;
        });
    }

    pub fn save_key_value(&self, key: &str, value: bool) -> Result<(), RustbreakError> {
        self.db.write(|db| {
            db.insert(key.into(), value);
        })?;

        self.db.save()?;
        Ok(())
    }
}
