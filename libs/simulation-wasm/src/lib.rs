use std::error::Error;
pub use self::{animal::*, world::*, food::*};

mod animal;
mod food;
mod world;

use lib_simulation as sim;
use rand::prelude::*;
use wasm_bindgen::prelude::*;
use serde::Serialize;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(config: JsValue) -> Self {
        let config: sim::Config = config.into_serde().unwrap();

        let mut rng = thread_rng();
        let sim = sim::Simulation::random(config, &mut rng);

        Self { rng, sim }
    }

    pub fn default_config() -> JsValue {
        JsValue::from_serde(&sim::Config::default()).unwrap()
    }

    pub fn config(&self) -> JsValue {
        JsValue::from_serde(self.sim.config()).unwrap()
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        JsValue::from_serde(&world).unwrap()
    }

    pub fn step(&mut self) -> Option<String> {
        self.sim.step(&mut self.rng).map(|stats| stats.to_string())
    }

    pub fn train(&mut self) -> String {
        self.sim.train(&mut self.rng).to_string()
    }

    // pub fn export_stats(&mut self) -> Result<(), Box<dyn Error>> {
    //     todo!()
    // }
}

#[wasm_bindgen]
pub struct Statistics {

}