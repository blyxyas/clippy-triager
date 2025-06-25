//ISSUE #14000 <https://github.com/rust-lang/rust-clippy/issues/14000> - C-bug, I-false-positive

pub fn install_ticker() {
    let mut schedule = tokio::time::interval(std::time::Duration::from_secs(5));
    tokio::spawn({
        async move {
            loop {
                schedule.tick().await;
                println!("Tick");
            }
        }
    });
}
