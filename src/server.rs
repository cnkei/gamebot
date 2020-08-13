use std::error::Error;
use std::fs;

use elefren::prelude::*;
use elefren::entities::event::Event;

pub struct Server {
    token: String,
}

impl Server {
    pub fn new(token_file: &str) -> Result<Server, Box<dyn Error>> {
        let token = fs::read_to_string(token_file)?;
        Ok(Server{token: String::from(token.trim())})
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let data = Data{
            base: "https://monado.ren".into(),
            client_id: "".into(),
            client_secret: "".into(),
            redirect: "".into(),
            token: self.token.clone().into(),
        };

        let client = Mastodon::from(data);
        for status in client.get_home_timeline()?.items_iter() {
            println!("{:?}", status);
        }

        /*
        for event in client.streaming_direct()? {
            println!("{:?}", event);
        }
        */

        Ok(())
    }
}
