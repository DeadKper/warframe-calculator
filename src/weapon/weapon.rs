use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::weapon::Mods;
use crate::damage::{Type, Attribute};

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

    fn has_dmg(&self, dmg_type: Type) -> bool {
        !dmg_type.mult() && (
            self.damage.contains_key(&dmg_type)
            || self.mods.is_some()
            && self.mods.unwrap().damage.contains_key(&dmg_type)
        )
    }

    pub fn valid_dmg_list(&self) -> Vec<Type> {
        let mut items: Vec<Type> = Vec::new();

        for dmg_type in Type::iter().filter(move |&dmg_type| self.has_dmg(dmg_type)) {
            items.push(dmg_type);
        }

        items
    }

    pub fn get_damage(&self, dmg_type: Option<Type>, dot: bool) -> f64 {
        match self.mods {
            Some(mods) => {
                if let Some(dmg_type) = dmg_type {
                    if dmg_type.mult() || !self.has_dmg(dmg_type) {
                        0.0
                    } else if dmg_type.ips() && self.damage.contains_key(&dmg_type) {
                        self.damage[&dmg_type] * mods.get_dmg_mult(Some(dmg_type), dot)
                    } else {
                        self.get_base_damage(None) * mods.get_dmg_mult(Some(dmg_type), dot)
                    }
                } else {
                    let mut damage = 0.0;
                    Type::iter()
                        .filter(|dmg_type|
                            !dmg_type.mult()
                            && (
                                self.damage.contains_key(dmg_type)
                                || mods.damage.contains_key(dmg_type)
                            ))
                        .for_each(|dmg_type|
                            damage += self.get_damage(Some(dmg_type), false)
                        );
                    damage
                }
            },
            _ => self.get_base_damage(None)
        }
    }

    pub fn get_attr(&self, attr: Attribute) -> f64 {
        let base = match attr {
            Attribute::CritChance => self.crit_chance,
            Attribute::CritDamage => self.crit_damage,
            Attribute::StatusChance => self.status_chance,
        };

        let mult = match self.mods {
            None => 1.0,
            Some(mods) => {
                match attr {
                    Attribute::CritChance => (mods.crit_chance + 100.0) / 100.0,
                    Attribute::CritDamage => (mods.crit_damage + 100.0) / 100.0,
                    Attribute::StatusChance => (mods.status_chance + 100.0) / 100.0,
                }
            },
        };

        base * mult
    }

    pub fn get_status_weight(&self, dmg_type: Type) -> f64 {
        self.get_damage(Some(dmg_type), false) / self.get_damage(None, false) * 100.0
    }

    pub fn get_status_chance(&self, dmg_type: Type) -> f64 {
        self.get_attr(Attribute::StatusChance) / 100.0 * self.get_status_weight(dmg_type)
    }

    pub fn get_crit_mult(&self) -> f64{
        self.get_attr(Attribute::CritChance) / 100.0 * self.get_attr(Attribute::CritDamage)
    }

    pub fn print_stats(&self) {
        println!("Weapon: {}", self.name);
        match self.mods {
            Some(mods) => println!("Mods: {}", mods.combination_name),
            _ => {},
        }
        println!();

        println!("Crit Chance: {:.2}", self.get_attr(Attribute::CritChance));
        println!("Crit Damage: {:.2}", self.get_attr(Attribute::CritDamage));
        println!("Status Chance: {:.2}", self.get_attr(Attribute::StatusChance));
        println!();

        for (dmg_type, value) in &self.damage {
            println!("Base {}: {:.2}", dmg_type, value);
        }
        println!("Base Total: {:.2}", self.get_base_damage(None));
        println!();

        for &dmg_type in self.valid_dmg_list().iter() {
            println!("Hit {}: {:.2}", dmg_type, self.get_damage(Some(dmg_type), false) * self.get_crit_mult());
        }
        println!("Hit Total: {:.2}", self.get_damage(None, false) * self.get_crit_mult());
        println!();

        let mut total_dot = 0.0;
        for dmg_type in Type::iter().filter(|&dmg_type| dmg_type.dot() && self.has_dmg(dmg_type)) {
            let dot = self.get_damage(Some(dmg_type), true) * self.get_crit_mult() * self.get_status_chance(dmg_type) / 100.0;
            total_dot += dot;
            println!("DoT {} Weight: {:.2}%", dmg_type, self.get_status_weight(dmg_type));
            println!("DoT {} Chance: {:.2}%", dmg_type, self.get_status_chance(dmg_type));
            println!("DoT {} Damage: {:.2}", dmg_type, dot);
        }
        println!("DoT Total: {:.2}", total_dot);
    }
}