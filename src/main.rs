#[macro_use]
extern crate clap;

#[macro_use]
extern crate prettytable;
use prettytable::Table;

use clap::App;
use openapi::apis::{build_type_api, configuration::Configuration};
use std::collections::BTreeSet;
use std::error::Error;
use std::fs::File;
use tokio;

mod model;
mod provider;

pub use provider::{Provider, ProviderType};

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
    let provider: ProviderType = match input.parse() {
        Ok(provider) => provider,
        Err(_) => {
            println!("{} is not a supported provider", input);
            std::process::exit(1);
        }
    };  
    let mut providers = load_enabled_providers().unwrap();
    providers.push(provider);
    write_enabled_providers(&providers).unwrap();
}

fn handle_list(_: &clap::ArgMatches<'_>) {
    let providers = load_enabled_providers().unwrap();
    let mut table = Table::new();
    table.add_row(row!["name"]);
    for p in providers {
        table.add_row(row![format!("{:?}", p)]);
    }
    table.printstd();
}

fn handle_pipeline(sc: &clap::ArgMatches<'_>) {
    match sc.subcommand() {
        ("list", Some(sc)) => handle_pipeline_list(sc),
        _ => {}
    }
}

#[tokio::main]
async fn handle_pipeline_list(_: &clap::ArgMatches<'_>) {
    let providers = load_enabled_providers().unwrap();
    let cfgs = provider::configuration::load_provider_configs(providers);

    let mut table = Table::new();
    table.add_row(row!["name", "provider", "id", "project"]);

    for c in cfgs {
        let mut rc = Configuration::new();
        rc.bearer_access_token = c.api_token;
        let m = build_type_api::get_all_build_types(&rc, None, None)
            .await
            .unwrap();

        let pipelines = m.build_type.unwrap();

        for p in pipelines {
            table.add_row(row![
                p.name.unwrap(),
                format!("{:?}", c.provider),
                p.id.unwrap(),
                p.project_name.unwrap()
            ]);
        }
    }
    table.printstd();
}

#[tokio::main]
async fn handle_project_list(_: &clap::ArgMatches<'_>) {
    let cfg = provider::configuration::load_provider_config(ProviderType::TeamCity).unwrap();
    let mut c: Configuration = Configuration::new();
    c.bearer_access_token = cfg.api_token;
    let provider: provider::teamcity::TeamCity = provider::Provider::new();
    let res = provider.projects().await;

    let mut table = Table::new();
    table.add_row(row!["name", "provider"]);
    for p in res {
        table.add_row(row![p.name, format!("{:?}", cfg.provider)]);
    }
    table.printstd();
}

fn load_enabled_providers() -> Result<Vec<ProviderType>, Box<dyn Error>> {
    let file = File::open("cider.yaml")?;
    let len = file.metadata().unwrap().len();
    let input: BTreeSet<String>;
    let mut providers: Vec<ProviderType> = Vec::new();
    if len > 0 {
        input = serde_yaml::from_reader(file).unwrap();
        for i in input {
            providers.push(i.parse()?);
        }
    } else {
        providers = Vec::new();
    }
    Ok(providers)
}

fn write_enabled_providers(providers: &Vec<ProviderType>) -> Result<(), Box<dyn Error>> {
    let file = File::create("cider.yaml")?;
    let mut provider_set: BTreeSet<String> = BTreeSet::new();
    for p in providers {
        provider_set.insert(format!("{:?}", p));
    }
    serde_yaml::to_writer(file, &provider_set).unwrap();
    Ok(())
}
