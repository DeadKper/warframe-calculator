#![allow(unused_variables, unused_imports, dead_code)]

use warframe::weapon::Weapon;
use warframe::weapon::Mods;
use warframe::damage::Type;

fn main() {
    let mut whipclaw = Weapon::new("Whipclaw", 25.0, 2.0, 20.0);
    whipclaw.damage.insert(Type::Impact, 100.0);
    whipclaw.damage.insert(Type::Puncture, 100.0);
    whipclaw.damage.insert(Type::Slash, 100.0);
    
    whipclaw.add_base_damage(100.0);

    let mut mod_set = Mods::new("Tmp", 25.0, 2.0, 20.0);

    mod_set.add_damage(Type::Base, 165.0);
    mod_set.add_damage(Type::Base, 180.0);
    mod_set.add_damage(Type::Base, 350.0);
    mod_set.add_damage(Type::Base, -60.0);

    println!("{}", mod_set.get_dmg_mult(false));

    mod_set.add_damage(Type::Faction, 50.0);

    println!("{}", mod_set.get_dmg_mult(false));

    println!("{}", whipclaw.get_base_damage(None));
}
