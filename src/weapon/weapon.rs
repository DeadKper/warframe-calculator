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
    pub mods: Option<Mods<'a>>,
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
                for (key, value) in &mods.damage {
                    if damage_type != *key || ! self.damage.contains_key(&damage_type) {
                        continue;
                    }

                    return if damage_type.ips() {
                        self.damage[key]
                    } else {
                        self.get_base_damage(None)
                    } * (value + 100.0) / 100.0;
                }
            } else {
                let base = self.get_base_damage(None);
                let mut total = 0.0;
                for (key, value) in &mods.damage {
                    if ! self.damage.contains_key(key) {
                        continue
                    }

                    total += if key.ips() {
                        self.damage[key]
                    } else {
                        base
                    } * (value + 100.0) / 100.0;
                }
                return total;
            }
        }
        self.get_base_damage(None)
    }

    pub fn get_bleed(&self, mods: Option<Mods>) -> f64 {
        let slash = self.damage.get(&Type::Slash);
        match slash {
            Some(slash) => {
                if let Some(mods) = mods {
                    return *slash * mods.get_dmg_mult(true);
                } else {
                    return *slash;
                }
            },
            None => 0.0,
        }
    }
}