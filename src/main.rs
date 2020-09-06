use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use prometheus_exporter_base::{render_prometheus, PrometheusMetric, MetricType};
use std::process::Command;
use std::str::from_utf8;

#[derive(Debug, Clone, Default)]
struct MyOptions {}

#[derive(Debug)]
struct MultiError {
  details: String
}

impl MultiError {
  fn new(msg: &str) -> MultiError {
    MultiError{details: msg.to_string()}
  }
}

impl std::fmt::Display for MultiError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f,"{}",self.details)
  }
}

impl std::error::Error for MultiError {
  fn description(&self) -> &str {
    &self.details
  }
}

impl From<std::io::Error> for MultiError {
  fn from(err: std::io::Error) -> Self {
    MultiError::new(&err.to_string())
  }
}

impl From<std::str::Utf8Error> for MultiError {
  fn from(err: std::str::Utf8Error) -> Self {
    MultiError::new(&err.to_string())
  }
}

impl From<std::num::ParseIntError> for MultiError {
  fn from(err: std::num::ParseIntError) -> Self {
    MultiError::new(&err.to_string())
  }
}

fn get_used_space() -> Result<u64, MultiError> {
  Ok(
    from_utf8(
      &Command::new("zfs")
        .arg("get")
        .arg("-H") // No headers
        .arg("-o") // Get only the value
        .arg("value")
        .arg("-p") // Exact size in bytes
        .arg("used") // Get used space
        .arg("rpool")
        .output()?
        .stdout
    )?
    .trim()
    .parse::<u64>()?
  )
}

fn get_free_space() -> Result<u64, MultiError> {
  Ok(
    from_utf8(
      &Command::new("zfs")
        .arg("get")
        .arg("-H") // No headers
        .arg("-o") // Get only the value
        .arg("value")
        .arg("-p") // Exact size in bytes
        .arg("available") // Get available space
        .arg("rpool")
        .output()?
        .stdout
    )?
    .trim()
    .parse::<u64>()?
  )
}

#[tokio::main]
async fn main() {
  let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9101);
  render_prometheus(addr, MyOptions::default(), |_request, _options|
    async move {
      let used_space_metric = PrometheusMetric::new("rpool_size_used", MetricType::Gauge, "Used space in the zpool in bytes");
      let free_space_metric = PrometheusMetric::new("rpool_size_free", MetricType::Gauge, "Free space in the zpool in bytes");

      let used_space = get_used_space()?;
      let free_space = get_free_space()?;
    
      let mut s = used_space_metric.render_header();
      s.push_str(&used_space_metric.render_sample(None, used_space, None));

      s.push_str(&free_space_metric.render_header());
      s.push_str(&free_space_metric.render_sample(None, free_space, None));

      Ok(s)
  })
  .await;
}
