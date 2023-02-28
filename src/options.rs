use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "zpool exporter")]
pub(crate) struct Options {
    /// Sets the port the exporter uses
    #[structopt(short, long, default_value = "9101")]
    pub port: u16,

    /// Which properties of the zpool should be retrieved
    #[structopt(short = "P", long, default_value = "used,available", use_delimiter = true)]
    pub properties: Vec<String>,

    /// Which datasets should be polled for properties
    #[structopt(short, long, use_delimiter = true)]
    pub datasets: Vec<String>,
}

//impl Options {
//  pub fn from_claps(matches: &clap::ArgMatches<'_>) -> Options {
//    Options {
//      // All options are either required or have a default value, so unwrap should be safe
//      port: matches.value_of("port").map(|e| {
//        e.parse()
//      }).unwrap()
//      .expect("Invalid port number"),
//      properties: matches.values_of("properties").map(|e| {
//        e.into_iter()
//          .map(|a| {
//            a.to_owned()
//          })
//          .collect()
//      }).unwrap(),
//      datasets: matches.values_of("datasets").map(|e| {
//        e.into_iter()
//          .map(|a| {
//            a.to_owned()
//          })
//          .collect()
//      }).unwrap(),
//    }
//  }
//}
