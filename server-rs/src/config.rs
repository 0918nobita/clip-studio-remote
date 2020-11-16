use clap::{App, Arg};

#[derive(Copy, Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub send_keys: bool,
}

impl Config {
    pub fn parse(app: App) -> Self {
        let app = app
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .takes_value(true)
                    .default_value("8080"),
            )
            .arg(
                Arg::with_name("send-keys")
                    .help("Whether this program will execute sendkeys command (macOS: osascript, linux: xdotool)")
                    .short("k")
                    .long("send-keys")
                    .takes_value(false),
            );

        let matches = app.get_matches();

        let port = matches.value_of("port").unwrap_or("8080");
        let port = port
            .to_string()
            .parse::<u16>()
            .expect("Faild to parse the port number");

        let send_keys = matches.is_present("send-keys");

        Config { port, send_keys }
    }
}
