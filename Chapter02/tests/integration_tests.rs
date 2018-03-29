extern crate elevator;
extern crate timebomb;
use timebomb::timeout_ms;

#[test]
fn test_main() {
   timeout_ms(|| {
      elevator::run_simulation();
   }, 300000);
}
