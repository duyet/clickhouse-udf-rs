use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{collections::HashMap, env};
use tera::{Context, Tera};

#[derive(Serialize, Deserialize, Debug)]
struct UdfConfig {
    udf_name: String,
    usages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Bin {
    name: String,
    bin: String,
    udf_name: String,
    usages: Vec<String>,
}

#[derive(Serialize)]
struct Project {
    name: String,
    bins: Vec<Bin>,
}

const IGNORED: [&str; 2] = ["shared", "string"];

struct ToClickHouseFunctionName;

impl tera::Filter for ToClickHouseFunctionName {
    fn filter(
        &self,
        value: &tera::Value,
        _: &HashMap<String, tera::Value>,
    ) -> tera::Result<tera::Value> {
        match value {
            tera::Value::String(bin) => Ok(tera::Value::String(to_clickhouse_udf_name(bin))),
            _ => Err("Expected a string".into()),
        }
    }
}

fn to_clickhouse_udf_name(bin: &str) -> String {
    let name = bin.trim_end_matches("-chunk-header");
    change_case::camel_case(name)
}

fn get_bin_config(member: String) -> Result<HashMap<String, UdfConfig>> {
    let path = format!("{}/udf_config.toml", member);

    // Read toml
    let content = std::fs::read_to_string(&path)?;

    let config = toml::from_str::<HashMap<String, UdfConfig>>(&content)?;

    Ok(config)
}

fn get_bins(member: String) -> Result<Vec<Bin>> {
    let child_path = format!("{}/Cargo.toml", member);

    let mut manifest = cargo_toml::Manifest::from_path(Path::new(&child_path))?;
    manifest.complete_from_path(Path::new(&child_path))?;

    let bins = manifest
        .bin
        .into_iter()
        .map(|bin| {
            let config = get_bin_config(member.clone()).unwrap_or_default();

            // Extract bin name once to avoid repeated cloning
            let bin_name = bin.name.unwrap_or_default();
            let bin_path = bin.path.unwrap_or_default();

            Bin {
                name: bin_name.clone(),
                bin: bin_path,
                udf_name: config
                    .get(&bin_name)
                    .map(|c| c.udf_name.clone())
                    .unwrap_or_else(|| to_clickhouse_udf_name(&bin_name)),
                usages: config
                    .get(&bin_name)
                    .map(|c| c.usages.clone())
                    .unwrap_or_default(),
            }
        })
        .collect::<Vec<_>>();

    Ok(bins)
}

/// Get a list of projects from the current workspace
fn get_projects() -> Result<Vec<Project>> {
    let mut manifest = cargo_toml::Manifest::from_path(Path::new("Cargo.toml"))?;
    manifest.complete_from_path(Path::new("Cargo.toml"))?;

    match manifest.workspace {
        Some(ref workspace) => Ok(workspace
            .clone()
            .members
            .into_iter()
            .filter(|member| !IGNORED.contains(&member.as_str()))
            .map(|ref member| Project {
                name: member.to_string(),
                bins: get_bins(member.to_string()).unwrap_or_default(),
            })
            .collect::<Vec<_>>()),
        None => Err(anyhow!("No workspace found")),
    }
}

fn get_tera_context() -> Result<Context> {
    let mut context = Context::new();

    // TODO: Binding the latest version from Cargo.toml
    let version = std::env::var("RELEASE_VERSION").unwrap_or_else(|_| "<version>".to_string());
    // v0.1.0 -> 0.1.0
    let version = version.trim_start_matches('v');
    context.insert("version", &version);

    // Binding a list of projects
    let projects = get_projects()?;
    context.insert("projects", &projects);

    Ok(context)
}

/// Usage: cargo run --bin readme-generator -- README.tpl
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let root = args.get(1).expect("Missing template file");

    // Create a new Tera instance and add a template from a string
    let mut tera =
        Tera::new(&format!("{}/**/*.tpl", root)).expect("Could not create Tera instance");
    tera.register_filter("to_clickhouse_function", ToClickHouseFunctionName);

    // Prepare the context with some data
    let context = get_tera_context()?;

    // List template
    let names: Vec<_> = tera.get_template_names().collect();
    match names.contains(&"README.tpl") {
        true => {
            let rendered = tera
                .render("README.tpl", &context)
                .expect("Could not render README");
            println!("{}", rendered);
        }
        false => println!("No README.tpl found"),
    }

    println!("Done");
    Ok(())
}
