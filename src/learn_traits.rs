use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
struct Hovercraft {
    name: String
}

impl WaterCapable for Hovercraft {}

impl LandCapable for Hovercraft {
    fn drive(&self, _hc: Hovercraft) where Self: Display {
        println!("driving: {}", self);
    }
}

impl Display for Hovercraft {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("print")
    }
}

impl Amphibious for Hovercraft {
   fn admin(&self, hc: Hovercraft) -> String {
        hc.name + ":execute in admin trait function"
    }
}
trait Amphibious: WaterCapable + LandCapable + Display {

    fn admin(&self, hc: Hovercraft) -> String;
}
trait WaterCapable {
    fn float(&self) {
        println!("floating");
    }
}
trait LandCapable {
    fn drive(&self, hc: Hovercraft) where Self: Display {
        println!("driving: {}, hovercraft name {}", self, hc.name);
    }
}
#[warn(dead_code)]
fn road_trip(vehicle: &(impl LandCapable + Display)) {
    vehicle.drive(Hovercraft { name: "road_trio".to_string() });
}

fn traverse_frozen_lake(vehicle: &impl Amphibious) {
    vehicle.drive(Hovercraft { name: "traverse_frozen_lake_drive".to_string() });
    vehicle.float();
}

#[test]
fn test_learn_trait() {
    let hc = Hovercraft { name: "king".to_string()} ;
    traverse_frozen_lake(&hc);
    let res = hc.admin(hc.clone());

    println!("res: {}", res)
}

