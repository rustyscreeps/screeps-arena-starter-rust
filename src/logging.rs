use js_sys::JsString;
//use screeps::Game;
use web_sys::console;

pub use log::LevelFilter::*;

struct JsLog;

impl log::Log for JsLog {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }
    fn log(&self, record: &log::Record<'_>) {
        console::log_1(&JsString::from(format!("{}", record.args())));
    }
    fn flush(&self) {}
}

pub fn setup_logging(verbosity: log::LevelFilter) {
    fern::Dispatch::new()
        .level(verbosity)
        .format(|out, message, record| {
            out.finish(format_args!(
                "({}) {}: {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(Box::new(JsLog) as Box<dyn log::Log>)
        .apply()
        .expect("expected setup_logging to only ever be called once per instance");
}
