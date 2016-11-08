extern crate flexi_logger;
#[macro_use]
extern crate log;

use flexi_logger::LogOptions;

#[test]
fn files_dscr() {
    assert_eq!((),
               LogOptions::new()
                   .log_to_file(true)
                   .discriminant(Some("foo".to_string()))
                   .init(Some("info".to_string()))
                   .unwrap());

    error!("This is an error message");
    warn!("This is a warning");
    info!("This is an info message");
    debug!("This is a debug message - you must not see it!");
    trace!("This is a trace message - you must not see it!");
}
