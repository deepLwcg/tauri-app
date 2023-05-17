use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::config::{Configs, Site};

lazy_static! {
    pub static ref CONFIG: Configs  = Configs::read_config();
}



impl Configs {
    fn read_config() -> Configs {
        let yaml_str = include_str!("./config.yaml");
        let config: Configs = serde_yaml::from_str(yaml_str)
            .expect("config.yaml read failed!");
        config
    }
}

impl CONFIG {
    pub fn get() -> Configs {
        CONFIG.clone()
    }

    pub fn sites() -> Vec<Site> {
        let config = &CONFIG;
        let sites = &config.sites;
        let mut new_sites: Vec<Site> = vec![];
        for x in sites {
            new_sites.push(x.clone());
        }
        new_sites
    }

    pub fn sites_map() -> HashMap<u32, Site> {
        let config = &CONFIG;
        let sites = &config.sites;
        let mut result: HashMap<u32, Site> = HashMap::new();
        for x in sites {
            let site: Site = x.clone();
            result.insert(site.id, site);
        }
        result
    }
}



