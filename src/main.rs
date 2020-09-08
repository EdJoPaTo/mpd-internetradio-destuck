use mpd::{Client, State, Status};
use std::thread::sleep;
use std::time::Duration;

const BETWEEN_CHECKS: Duration = Duration::from_secs(30);
const BETWEEN_ELAPSED: Duration = Duration::from_millis(400);

fn main() {
    let mut conn = Client::connect("127.0.0.1:6600").expect("failed to connect to mpd");

    println!("Connected. Running...");

    loop {
        check(&mut conn).expect("failed to perform check");
        sleep(BETWEEN_CHECKS);
    }
}

fn check(conn: &mut Client) -> Result<(), mpd::error::Error> {
    let before = conn.status()?;

    #[cfg(debug_assertions)]
    println!(
        "before  {:?} {:?} {:?}",
        before.state, before.elapsed, before.duration
    );

    if is_internet_radio_playing(&before) {
        sleep(BETWEEN_ELAPSED);
        let after = conn.status()?;

        #[cfg(debug_assertions)]
        println!(
            "after   {:?} {:?} {:?}",
            after.state, after.elapsed, after.duration
        );

        if is_internet_radio_playing(&after) && before.elapsed == after.elapsed {
            conn.stop()?;
            conn.play()?;
        }
    }

    Ok(())
}

fn is_internet_radio_playing(status: &Status) -> bool {
    status.state == State::Play && status.duration == None
}
