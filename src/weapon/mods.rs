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

    pub fn get_dmg_mult(&self, dmg_type: Option<Type>, dot: bool) -> f64 {
        let mut total = 1.0;

        for (&dmg_type, inc) in self.damage.iter().filter(|(&dmg_type, _)| dmg_type.mult()) {
            if dmg_type == Type::Faction && dot {
                total *= (inc + 100.0) / 100.0;
            }
            total *= (inc + 100.0) / 100.0;
        }

        if let Some(dmg_type) = dmg_type {
            match self.damage.get(&dmg_type) {
                Some(increase) => {
                    match dmg_type {
                        Type::Slash | Type::Gas => total *= if dot { 1.0 } else { (increase + 100.0) / 100.0 },
                        _ => total *= (increase + 100.0) / 100.0,
                    }
                },
                _ => {},
            }
        }

        total
    }
}