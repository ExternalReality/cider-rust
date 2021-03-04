#[macro_use]
extern crate clap;

#[macro_use]
extern crate prettytable;
use prettytable::{Table};

use clap::App;
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;
use strum_macros::EnumString;
use tokio;
use toml;
use openapi::api::{configuration}

#[derive(Serialize, Deserialize, EnumString, Debug)]
enum Provider {
    TeamCity,
    GitLab,
}

#[derive(Deserialize, Debug)]
struct ProviderConfig {
    url: String,
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("provider", Some(sc)) => handle_provider(sc),
        ("pipeline", Some(sc)) => handle_pipeline(sc),
        ("project", Some(sc)) => handle_project(sc),
        _ => {}
    }
}

fn handle_project(sc: &clap::ArgMatches<'_>) {
    match sc.subcommand() {
        ("list", Some(sc)) => handle_project_list(sc),
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
    let _: Provider = input.parse().unwrap();
    let file = File::open("cider.yaml").unwrap();
    let len = file.metadata().unwrap().len();
    let mut providers: BTreeSet<String> = BTreeSet::new();
    if len > 0 {
        providers = serde_yaml::from_reader(file).unwrap();
    }

    providers.insert(input.to_string());
    let file = File::create("cider.yaml").unwrap();
    serde_yaml::to_writer(file, &providers).unwrap();

    println!("{:?}", providers)
}

fn handle_list(_: &clap::ArgMatches<'_>) {
    let file = File::open("cider.yaml").expect("failed opening file");
    let providers: Vec<Provider> = serde_yaml::from_reader(file).unwrap();
    println!("{:?}", providers);
}

fn handle_pipeline(sc: &clap::ArgMatches<'_>) {
    match sc.subcommand() {
        ("list", Some(sc)) => handle_pipeline_list(sc),
        _ => {}
    }
}

#[tokio::main]
async fn handle_pipeline_list(_: &clap::ArgMatches<'_>) {
    let file = File::open("cider.yaml").expect("failed opening file");
    let providers: Vec<Provider> = serde_yaml::from_reader(file).unwrap();
    let cfgs = load_provider_configs(providers);

    for cfg in cfgs {
        let mut file = File::open("provider_config.toml").expect("failed opening file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let _: ProviderConfig = toml::from_str(&contents).unwrap();
        let c: Configuration = Configuration::new();
        let j = Configuration{
            bearer_access_token: Some(String::from("eyJ0eXAiOiAiVENWMiJ9.a21UTG9uRUE1NEZmdlJZNlFraFhMWHRvZXdV.NWViOGU2ZGUtODQ5NS00ZTc0LWJmNjktMmZhNDU3MDE1N2Iy")),
            ..c
    };
    let m = build_type_api::get_all_build_types(&j, None, None)
        .await
        .unwrap();
    let mut table = Table::new();
    table.add_row(row!["name", "provider", "id", "project"]);

    for bt in m.build_type.unwrap() {
        table.add_row(row![bt.name.unwrap(), "Team City", bt.id.unwrap(), bt.project_name.unwrap()]);
    }

    table.printstd();
}

#[tokio::main]
async fn handle_project_list(_: &clap::ArgMatches<'_>) {
    let mut file = File::open("provider_config.toml").expect("failed opening file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let _: ProviderConfig = toml::from_str(&contents).unwrap();
    let c: openapi::apis::configuration::Configuration =
        openapi::apis::configuration::Configuration::new();
    let j = openapi::apis::configuration::Configuration{
        bearer_access_token: Some(String::from("eyJ0eXAiOiAiVENWMiJ9.a21UTG9uRUE1NEZmdlJZNlFraFhMWHRvZXdV.NWViOGU2ZGUtODQ5NS00ZTc0LWJmNjktMmZhNDU3MDE1N2Iy")),
        ..c
    };
    let m = openapi::apis::project_api::get_all_projects(&j, None, None).await;
    println!("{:?}", m);
}

fn load_provider_configs(providers : Vec<Provider>) -> Vec<ProviderConfig> {
    let mut cfgs : Vec<ProviderConfig> = vec!();
    for p in providers {
        let mut filename = format!("./cider_config/{:?}.json", p);
        filename = filename.to_lowercase();
        let file = File::open(filename).expect("failed opening file");
        let cfg = serde_yaml::from_reader(file).unwrap();
        cfgs.push(cfg);
    } 
    cfgs
} 


