use mpd::{Client, State};
use std::thread::sleep;
use std::time::Duration;

const BETWEEN_CHECKS: Duration = Duration::from_secs(15);
const BETWEEN_ELAPSED: Duration = Duration::from_millis(400);

fn main() {
    let mut conn = Client::connect("127.0.0.1:6600").expect("failed to connect to mpd");

    loop {
        check(&mut conn).expect("failed to perform check");
        sleep(BETWEEN_CHECKS);
    }
}

fn check(conn: &mut Client) -> Result<(), mpd::error::Error> {
    let status = conn.status()?;
    println!("check {:?} {:?}", status.state, status.duration);
    if status.state == State::Play && status.duration == None {
        let before = status.elapsed;
        sleep(BETWEEN_ELAPSED);
        let after = conn.status()?.elapsed;
        println!("before & after {:?} {:?}", before, after);

        if before == after {
            conn.stop()?;
            conn.play()?;
        }
    }

    Ok(())
}
