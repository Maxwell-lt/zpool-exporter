#[macro_use] extern crate rocket;
use std::process::Command;
use std::str::from_utf8;
use anyhow::Result;
use rocket::{State, Config};
use structopt::StructOpt;
use options::Options;

mod options;

fn get_property(dataset: &str, property: &str) -> Result<u64> {
  Ok(
    from_utf8(
      &Command::new("zfs")
        .arg("get")
        .arg("-H") // No headers
        .arg("-o") // Get only the value
        .arg("value")
        .arg("-p") // Exact size in bytes
        .arg(property)
        .arg(dataset)
        .output()?
        .stdout
    )?
    .trim()
    .parse::<u64>()?
  )
}

#[get("/metrics")]
fn metrics(parameters: &State<options::Options>) -> Result<String, rocket::response::Debug<anyhow::Error>> {
    let mut output = String::new();
    output.push_str("# HELP zpool_property zpool property\n");
    output.push_str("# TYPE zpool_property gauge\n");

    for dataset in &parameters.datasets {
        for property in &parameters.properties {
            let value = get_property(&dataset, &property)?;
            output.push_str(&format!("zpool_property{{dataset=\"{}\",property=\"{}\"}} {}\n", &dataset, &property, value));
        }
    }

    Ok(output)

    
}

#[launch]
fn rocket() -> _ {
    let parameters: Options = Options::from_args();
    
    let config = Config::figment()
        .merge(("port", parameters.port));

    rocket::custom(config).manage(parameters).mount("/", routes![metrics])
}
