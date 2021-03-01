#[macro_use]
extern crate clap;

use clap::App;
use serde_derive::{Serialize, Deserialize};
use strum_macros::EnumString;
use std::fs::File;
use std::io::{Read, Write};
use tokio;

#[derive(Serialize, Deserialize, EnumString, Debug)]
enum Provider {
    TeamCity,
}

#[derive(Deserialize, Debug)]
struct ProviderConfig {
    name: Provider,
    url: String
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("provider", Some(sc)) => handle_provider(sc),
        ("pipeline", Some(sc)) => handle_pipeline(sc),
        _ => {}
    }
}

fn handle_provider(sc: &clap::ArgMatches<'_>) {
    match sc.subcommand() {
        ("enable", Some(sc)) => handle_enable(sc),
        ("list", Some(sc)) => handle_list(sc),
        _ => {}
    }
}

fn handle_enable(sc: &clap::ArgMatches<'_>) {
    let input = sc.value_of("provider").unwrap();
    let provider: Provider = input.parse().unwrap();
    let enabled_providers = vec![provider];
    let toml = toml::to_string(&enabled_providers).unwrap();
    let mut file = File::create("cider.dat").expect("create failed");
    let _ = file.write_all(toml.as_bytes());
}

fn handle_list(_: &clap::ArgMatches<'_>) {
   let mut file = File::open("cider.dat").expect("failed opening file");
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   println!("{}", contents);
}

fn handle_pipeline(sc: &clap::ArgMatches<'_>) {
    match sc.subcommand() {
        ("list", Some(sc)) => handle_pipeline_list(sc),
        _ => {}
    }
}

#[tokio::main]
async fn handle_pipeline_list(_: &clap::ArgMatches<'_>) {
    let mut file = File::open("provider_config.toml").expect("failed opening file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let _ : ProviderConfig = toml::from_str(&contents).unwrap();
    let c : openapi::apis::configuration::Configuration = openapi::apis::configuration::Configuration::new();
    let j = openapi::apis::configuration::Configuration{
        bearer_access_token: Some(String::from("eyJ0eXAiOiAiVENWMiJ9.a21UTG9uRUE1NEZmdlJZNlFraFhMWHRvZXdV.NWViOGU2ZGUtODQ5NS00ZTc0LWJmNjktMmZhNDU3MDE1N2Iy")),
        ..c
    };
    let m = openapi::apis::build_type_api::get_all_build_types(&j, None, None).await;
    println!("{:?}", m);
}