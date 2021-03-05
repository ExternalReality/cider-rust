#[macro_use]
extern crate clap;

#[macro_use]
extern crate prettytable;
use prettytable::Table;

use clap::App;
use openapi::apis::{build_type_api, configuration::Configuration};
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use strum_macros::EnumString;
use tokio;

#[derive(Serialize, Deserialize, EnumString, Debug)]
enum Provider {
    TeamCity,
    GitLab,
}

#[derive(Deserialize, Serialize, Debug)]
struct ProviderConfig {
    url: String,
    api_token: Option<String>,
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

    let mut table = Table::new();
    table.add_row(row!["name", "provider", "id", "project"]);

    for c in cfgs {
        let mut rc = Configuration::new();
        rc.bearer_access_token = c.api_token;
        let m = build_type_api::get_all_build_types(&rc, None, None)
            .await
            .unwrap();

        for bt in m.build_type.unwrap() {
            table.add_row(row![
                bt.name.unwrap(),
                "Team City",
                bt.id.unwrap(),
                bt.project_name.unwrap()
            ]);
        }
    }

    table.printstd();
}

#[tokio::main]
async fn handle_project_list(_: &clap::ArgMatches<'_>) {
    let cfg = load_provider_config(Provider::TeamCity);
    let mut c: Configuration = Configuration::new();
    c.bearer_access_token = cfg.api_token;
    let m = openapi::apis::project_api::get_all_projects(&c, None, None).await;
    println!("{:?}", m);
}

fn load_provider_configs(providers: Vec<Provider>) -> Vec<ProviderConfig> {
    let mut cfgs: Vec<ProviderConfig> = vec![];
    for p in providers {
        cfgs.push(load_provider_config(p));
    }
    cfgs
}

fn load_provider_config(provider: Provider) -> ProviderConfig {
    let mut filename = format!("./cider_config/{:?}.json", provider);
    filename = filename.to_lowercase();
    let file = File::open(filename).expect("failed opening file");
    let mut cfg: ProviderConfig = serde_yaml::from_reader(file).unwrap();
    let token_var = format!("{:?}_token", provider).to_uppercase();
    let token = env::var(token_var).expect("token env var not found");
    cfg.api_token = Some(token);
    cfg
}
