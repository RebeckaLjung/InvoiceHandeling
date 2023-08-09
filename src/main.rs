mod read_events;
use read_events::EventReader;
fn main() {
    let output = EventReader::read_events();
}
