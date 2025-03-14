use serde_json::Error;
use crate::files::{
    config::Config,
    managed_link::ManagedLink,
    managed_link_list::ManagedLinkList
};
use crate::utils::file_utils::FileUtils;

pub struct Application {
    pub managed_link_list: ManagedLinkList,
    pub config: Config,
    file_utils: FileUtils,
}

impl Application {
    pub fn new() -> Result<Application, Error> {
        let file_utils = FileUtils::new();
        let config_res = Config::load(&file_utils);

        if let Ok(config) = config_res {
            let managed_link_list = ManagedLinkList::load(&file_utils);
            return Ok(Application {
                managed_link_list,
                config,
                file_utils,
            });
        }

        Err(config_res.err().unwrap())
    }

    pub fn display_managed_links(&self) {
        for link in &self.managed_link_list.managed_links {
            println!("{link}\n");
        }
    }

    pub fn display_config(&self) {
        println!("{}", self.config);
    }

    pub fn add_managed_link(&mut self, managed_link: ManagedLink) {
        self.managed_link_list.managed_links.push(managed_link);
        self.managed_link_list.save(&self.file_utils);
    }
}