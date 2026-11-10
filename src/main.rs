///Pre-build, Auto-generated File
use time_travel_simulator::{Event, System, Simulator};

#[derive(Clone, Default, Debug)]
struct PlayerState {
    hp: i32,
    gold: i32,
    tick_count: u32,
}

impl System for PlayerState {
    fn process(&mut self, event: &Event) {
        match event {
            Event::Tick => {
                self.tick_count += 1;
                // Take 1 damage every tick
                self.hp -= 1;
            }
            Event::Input(action) => {
                if *action == 1 {
                    self.gold += 10;
                }
            }
            Event::NetworkMessage(packet) => {
                if *packet == 99 {
                    self.hp += 50; // Healing potion from server!
                }
            }
        }
    }
}

fn main() {
    println!("--- Starting Simulation ---");
    
    // Take a snapshot every 3 steps to save memory
    let mut sim = Simulator::<PlayerState>::new(3);

    // Simulate some events
    sim.step(Event::Tick); // step 1: hp 99, gold 0
    sim.step(Event::Input(1)); // step 2: hp 98, gold 10
    sim.step(Event::Tick); // step 3: hp 97, gold 10

    println!("At step 3: {:?}", sim.system());

    sim.step(Event::Tick); // step 4: hp 96, gold 10
    sim.step(Event::Tick); // step 5: hp 95, gold 10
    sim.step(Event::NetworkMessage(99)); // step 6: hp 145, gold 10

    println!("At step 6: {:?}", sim.system());

    // Uh oh, we realized we stepped wrong at step 4!
    println!("\n--- Rewinding to step 3 ---");
    sim.rewind_to(3);
    println!("At step 3 (after rewind): {:?}", sim.system());

    // Let's do a different action at step 4 (branching time)
    sim.step(Event::Input(1)); // step 4: hp 96, gold 20
    println!("At step 4 (new branch): {:?}", sim.system());

    sim.step(Event::Tick); // step 5: hp 95, gold 20
    sim.step(Event::Tick); // step 6: hp 94, gold 20
    println!("At step 6 (new branch): {:?}", sim.system());

    // Rewind all the way to the very beginning
    println!("\n--- Rewinding to step 1 ---");
    sim.rewind_to(1);
    println!("At step 1: {:?}", sim.system());
}