use std::collections::HashMap;

pub struct Wire {
    name: String,
    pub value: u16,
}

impl Wire {
    fn new(name: String, value: u16) -> Wire {
        Wire { name, value }
    }
}

pub struct WireBox {
    wires: HashMap<String, Wire>,
}

impl WireBox {
    pub fn new() -> WireBox {
        WireBox { wires : HashMap::new() }
    }

    pub fn create_wire(&mut self, name: &str, val: u16) -> &Wire {
        if !self.wires.contains_key(name) {
            self.wires.insert(String::from(name), Wire::new(String::from(name), val));
        }
        let x = self.wires.get(name).unwrap();
        if x.value!= val {
            println!("XXX {} {}/{}", name, x.value, val);
        }
        x
    }

    pub fn get_wire(&self, name: &str) -> Option<&Wire> {
        self.wires.get(name)
    }

    pub fn get_all_wires(&self) -> HashMap<String, u16> {
        let mut res: HashMap<String, u16> = HashMap::new();
        for wire in self.wires.values() {
            res.insert(String::from(&wire.name), wire.value);
        }
        res
    }
}
