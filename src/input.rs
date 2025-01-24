use std::collections::HashMap;

use anyhow::anyhow;

#[derive(Debug)]
pub struct Item {
    pub key: String,
    pub link: String,
}

impl TryFrom<&str> for Item {
    type Error = anyhow::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = s.split(',').collect();
        if split.len() != 2 {
            return Err(anyhow!("There should be 2 elements per line"));
        }
        let key = String::from(split[0]);
        let link = String::from(split[1]);
        Ok(Item { key, link })
    }
}

#[derive(Debug)]
pub struct Items {
    values: HashMap<String, String>,
}

impl TryFrom<&str> for Items {
    type Error = anyhow::Error;
    fn try_from(path: &str) -> Result<Self, Self::Error> {
        let mut values = HashMap::new();
        for line in std::fs::read_to_string(path)?.lines() {
            if line.is_empty() {
                continue;
            }
            let item = Item::try_from(line)?;
            values.insert(item.key, item.link);
        }
        Ok(Items { values })
    }
}

impl Items {
    pub fn get_link(&self, s: &str) -> Option<String> {
        self.values.get(s).cloned()
    }

    pub fn get_names(&self) -> String {
        self.values.keys().fold(String::new(), |acc, x| {
            if !acc.is_empty() {
                format!("{acc}\n{}", x)
            } else {
                String::from(x)
            }
        })
    }
}
