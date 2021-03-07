#[macro_use]
extern crate clap;

#[macro_use]
extern crate prettytable;

use prettytable::Table;

use clap::App;
use openapi::apis::{build_type_api, configuration::Configuration};
use std::collections::BTreeSet;
use std::fs::File;
use tokio;

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
    let _: ProviderType = input.parse().unwrap();
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
    let providers: Vec<ProviderType> = serde_yaml::from_reader(file).unwrap();
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
    let file = File::open("cider.yaml").expect("failed opening file");
    let providers: Vec<ProviderType> = serde_yaml::from_reader(file).unwrap();
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
    let cfg = provider::configuration::load_provider_config(ProviderType::TeamCity);
    let mut c: Configuration = Configuration::new();
    c.bearer_access_token = cfg.api_token;
    let _: provider::teamcity::TeamCity = provider::Provider::new();

    // let mut table = Table::new();
    // table.add_row(row!["name", "provider", "id"]);
    // for p in res.project.unwrap() {
    //     table.add_row(row![
    //         p.name.unwrap(),
    //         format!("{:?}", cfg.provider),
    //         p.id.unwrap(),
    //     ]);
    // }
    // table.printstd();
}
