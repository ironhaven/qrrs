use clap::{crate_authors, crate_version};
use clap::{App, AppSettings, Arg, ArgMatches};

#[derive(Debug)]
pub struct Config<'a> {
    pub input: Option<&'a str>,
    pub output: Option<&'a str>,
    pub read: bool,
    pub terminal_output: bool,
}

pub struct Arguments<'a> {
    pub matches: ArgMatches<'a>,
}

impl<'a> Arguments<'a> {
    pub fn new() -> Self {
        let matches = App::new("qrrs")
            .about("CLI tool for working with qr-codes")
            .version(crate_version!())
            .author(crate_authors!())
            .setting(AppSettings::ColoredHelp)
            .arg(
                Arg::with_name("INPUT")
                    .help("Input data")
                    .index(1)
                    .required(true),
            )
            .arg(
                Arg::with_name("OUTPUT")
                    .help("Output file")
                    .index(2)
                    .required_unless("read")
                    .required_unless("terminal"),
            )
            .arg(
                Arg::with_name("read")
                    .short("r")
                    .help("Reads the qr-code instead of generating it")
                    .long("read")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("terminal")
                    .short("t")
                    .help("Displays code in terminal")
                    .long("terminal")
                    .takes_value(false),
            )
            .get_matches();

        Arguments { matches }
    }
    pub fn get_config(&'a self) -> Config<'a> {
        Config {
            input: self.matches.value_of("INPUT"),
            output: self.matches.value_of("OUTPUT"),
            read: self.matches.is_present("read"),
            terminal_output: self.matches.is_present("terminal"),
        }
    }
}
