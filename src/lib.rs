use std::env;
use std::time::Duration;
use std::thread::sleep;
use notify_rust::Notification;

#[derive(Debug)]
pub struct Config {
    minutes: u16,
    seconds: u16,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let minutes = match args.next() {
            Some(arg) => arg,
            None => { return Err("Didn't find any arguments") },
        };

        match minutes.parse::<u16>() {
            Ok(arg) => Ok(Config { minutes: arg, seconds: arg * 60 }),
            Err(_) => { Err("Could not parse to integer") },
        }
    }

    pub fn notification_handler() {
        Notification::new()
            .summary("time is over")
            .body("break time")
            .icon("clock")
            .show();
    }

    pub fn countdown(&self) {
        for _ in 1..self.seconds {
            sleep(Duration::from_secs(1));
        }

        Config::notification_handler();
    }

}
