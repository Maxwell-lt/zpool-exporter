use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use prometheus_exporter_base::{render_prometheus, PrometheusMetric, MetricType};
use std::process::Command;
use std::str::from_utf8;
use clap::{Arg, App, crate_name, crate_authors, crate_version};
mod exporter_error;
use exporter_error::ExporterError;
mod options;
use options::Options;

fn get_property(dataset: &str, property: &str) -> Result<u64, ExporterError> {
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

#[tokio::main]
async fn main() {
  let matches = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!("\n"))
    .arg(Arg::with_name("port")
      .short("p")
      .long("port")
      .required(false)
      .help("Sets the port the exporter uses")
      .default_value("9101")
      .takes_value(true))
    .arg(Arg::with_name("properties")
      .short("P")
      .long("properties")
      .help("Which properties of the zpool should be retrieved")
      .required(false)
      .default_value("used,available")
      .use_delimiter(true)
      .takes_value(true))
    .arg(Arg::with_name("datasets")
      .short("d")
      .long("datasets")
      .help("Which datasets should be polled for properties")
      .required(true)
      .use_delimiter(true)
      .takes_value(true))
    .get_matches();

  let options = Options::from_claps(&matches);      
  
  let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), options.port);
  render_prometheus(addr, options, |_request, options|
    async move {
      let zpool_metric = PrometheusMetric::new("zpool_property", MetricType::Gauge, "zpool property");
      let mut s = zpool_metric.render_header();

      for dataset in &options.datasets {
        for property in &options.properties {
          let attributes = vec![("dataset", dataset.as_str()), ("property", property.as_str())];
          let value = get_property(&dataset, &property)?;
          s.push_str(&zpool_metric.render_sample(Some(&attributes), value, None));
        }
      }

      Ok(s)
  })
  .await;
}
