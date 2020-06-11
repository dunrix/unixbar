use super::base::{Sender, Widget};
use libc_strftime as lctime;
use format::data::Format;
use std::thread;
use std::time::Duration;

pub struct StrfTime {
    format: String,
}

impl Widget for StrfTime {
    fn current_value(&self) -> Format {
        Format::Str(lctime::strftime_local(&self.format, lctime::epoch()))
    }

    fn spawn_notifier(&mut self, tx: Sender<()>) {
        let seconds = if self.format.contains("%S") { 1 } else { 60 };
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(seconds));
            tx.send(());
        });
    }
}

impl StrfTime {
    pub fn new(format: &str) -> Box<StrfTime> {
        // locale settings
        lctime::tz_set();
        lctime::set_locale();

        Box::new(StrfTime {
            format: format.to_owned(),
        })
    }
}
