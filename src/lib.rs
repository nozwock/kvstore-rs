pub mod kv_store {

    use std::collections::HashMap;
    use std::fs;

    pub const KVSTORE_DELIMITER: &str = "->";

    // A simple UTF-8 kv database.
    pub struct Database {
        map: HashMap<String, String>,
        path: String,
    }

    impl Database {
        // new(path: String) associated fn returns a Database instance populated with
        // the entries from the database file.
        pub fn new(path: &str) -> std::io::Result<Database> {
            let mut kvstore = Database {
                map: HashMap::new(),
                path: path.to_string(),
            };
            // umm, currently does not create the db file if not available...hehe
            let contents = match fs::read_to_string(&kvstore.path) {
                Ok(contents) => contents,
                Err(_) => {
                    let _ = fs::File::create(path).unwrap();
                    return Ok(kvstore);
                }
            };

            for line in contents.lines() {
                let chunk = line
                    .split(KVSTORE_DELIMITER)
                    .map(|s| s.trim())
                    .collect::<Vec<_>>();
                // populating the database
                kvstore
                    .map
                    .insert(chunk[0].to_string(), chunk[1].to_string()); // ignore the Option<>
            }
            Ok(kvstore)
        }

        // insert() method to insert a kv pair in the database
        pub fn insert(&mut self, key: String, value: String) {
            self.map.insert(key, value);
        }

        // Flushes the database data to the disk file
        // we're still taking ownership here intentionally,
        // throw the db away after contentss has been written to disk file.
        pub fn flush(self) -> std::io::Result<()> {
            //   ^^^^-----here
            let mut contents = String::new();
            for (key, value) in &self.map {
                // let chunk = format!("{}{KVSTORE_DELIMITER}{}\n", key, value);
                // contents.push_str(&chunk);

                // trying not to use a String to make it more efficient
                contents.push_str(key);
                contents.push_str(KVSTORE_DELIMITER);
                contents.push_str(value);
                contents.push('\n');
            }
            fs::write(self.path, contents)
        }

        // to fetch the value for the given key from the database
        pub fn get(&self, key: String) -> Option<String> {
            match self.map.get(&key) {
                Some(value) => Some(value.to_owned()),
                None => None,
            }
        }
    }
}
