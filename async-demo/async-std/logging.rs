//! Prints the runtime's execution log on the standard output.

use async_std::task;

fn main() {
    // femme::with_level(log::LevelFilter::Trace);
    femme::with_level(log::LevelFilter::Info);

    task::block_on(async {
        let handle = task::spawn(async {
            log::info!("Hello world!\n");
        });

        handle.await;
    })
}
