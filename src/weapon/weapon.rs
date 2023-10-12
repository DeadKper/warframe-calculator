use std::collections::HashMap;

use crate::weapon::Mods;
use crate::damage::Type;

#[derive(Clone, Default)]
pub struct Weapon<'a> {
    pub name: &'a str,
    pub damage: HashMap<Type, f64>,
    pub crit_chance: f64,
    pub crit_damage: f64,
    pub status_chance: f64,
    pub mods: Option<&'a Mods<'a>>,
}

impl<'a> Weapon<'a> {
    pub fn new(name: &'a str, crit_chance: f64, crit_damage: f64, status_chance: f64) -> Self {
        Self {
            name,
            damage: HashMap::new(),
            crit_chance,
            crit_damage,
            status_chance,
            mods: None,
        }
    }

    pub fn add_base_damage(&mut self, damage: f64) {
        let total = self.get_base_damage(None);
        for (_, value) in &mut self.damage {
            *value += *value / total * damage;
        }
    }

    pub fn get_base_damage(&self, damage_type: Option<Type>) -> f64 {
        if let Some(damage_type) = damage_type {
            if self.damage.contains_key(&damage_type) {
                return self.damage[&damage_type];
            }
        } else {
            let mut total = 0.0;
            for (_, value) in &self.damage {
                total += value;
            }
            return total;
        }
        0.0
    }

    pub fn get_damage(&self, damage_type: Option<Type>) -> f64 {
        let mods = &self.mods;
        if let Some(mods) = mods {
            if let Some(damage_type) = damage_type {
                if ! self.damage.contains_key(&damage_type) {
                    return 0.0;
                }

                return self.damage[&damage_type] * mods.get_dmg_mult(damage_type.dot());
            } else {
                return self.get_base_damage(None) * mods.get_dmg_mult(false);
            }
        }
        self.get_base_damage(None)
    }
}