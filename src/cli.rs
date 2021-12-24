use clap::{App, Arg, ArgMatches};

pub(crate) fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("tuple-space-server")
        .arg(
            Arg::with_name("config-file")
                .long("config-file")
                .short("c")
                .takes_value(true)
                .required(true)
                .help("Config file to use"),
        )
        .get_matches()
}
