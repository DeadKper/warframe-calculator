use std::collections::HashMap;

use crate::damage::Type;

#[derive(Clone, Default)]
pub struct Mods<'a> {
    pub combination_name: &'a str,
    pub damage: HashMap<Type, f64>,
    pub crit_chance: f64,
    pub crit_damage: f64,
    pub status_chance: f64,
}

impl<'a> Mods<'a> {
    pub fn new(combination_name: &'a str, crit_chance: f64, crit_damage: f64, status_chance: f64) -> Self {
        Self {
            combination_name,
            damage: HashMap::new(),
            crit_chance,
            crit_damage,
            status_chance,
        }
    }

    pub fn add_damage(&mut self, dmg_type: Type, ammount: f64) {
        if self.damage.contains_key(&dmg_type) {
            self.damage.insert(dmg_type, self.damage[&dmg_type] + ammount as f64);
        } else {
            self.damage.insert(dmg_type, ammount as f64);
        }
    }

    pub fn get_dmg_mult(&self, dot: bool) -> f64 {
        let mut total = 1.0;
        for (dmg, value) in &self.damage {
            if ! dmg.mult() { continue; }
            if dmg == &Type::Faction && dot {
                total *= (value + 100.0) / 100.0;
            }
            total *= (value + 100.0) / 100.0;
        }
        total
    }
}