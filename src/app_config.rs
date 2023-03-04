use config::{Config, File as ConfigFile};
use serde::Deserialize;
use std::{error::Error, fs::File, io::BufReader};

#[derive(Debug, Deserialize)]
pub struct MongoMember {
    pub name: String,
    pub host: String,
    pub port: u32,
}

type MongoMembers = Vec<MongoMember>;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub foo: String,
    pub bar: u32,
    pub mongo_members: MongoMembers,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        let mongo_members = match read_mongo_members_from_file() {
            Ok(members) => members,
            Err(error) => panic!("Unable to read mongo members because: {:#?}", error),
        };

        let conf = Config::builder().add_source(ConfigFile::with_name("config/default"));

        let conf = match conf.build() {
            Ok(c) => c,
            Err(error) => panic!(
                "Unable to build the application config because: {:#?}",
                error
            )
        };

        let mut conf: AppConfig = match conf.try_deserialize() {
            Ok(ac) => ac,
            Err(error) => panic!("Cannot deserialise the application config because: {:#?}", error)
        };

        conf.mongo_members = mongo_members;

        conf
    }
}

fn read_mongo_members_from_file() -> Result<MongoMembers, Box<dyn Error>> {
    let file = File::open("../config/mongo-members.json")?;
    let reader = BufReader::new(file);

    let mongo_members = serde_json::from_reader(reader)?;

    Ok(mongo_members)
}
