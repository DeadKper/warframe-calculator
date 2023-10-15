use warframe::damage::Type;
use warframe::weapon::Mods;
use warframe::weapon::Weapon;

fn main() {
    let mut whipclaw = Weapon::new("Whipclaw", 25.0, 2.0, 20.0);
    whipclaw.damage.insert(Type::Impact, 100.0);
    whipclaw.damage.insert(Type::Puncture, 100.0);
    whipclaw.damage.insert(Type::Slash, 100.0);

    whipclaw.add_base_damage(100.0);
    whipclaw.crit_chance += 30.0;

    let mut mod_set = Mods::new(
        "Stack Stick Set",
        (40.0 + 10.0) * 11.0 + 220.0,
        90.0 + 60.0,
        40.0 * 11.0 + 60.0,
    );

    mod_set.add_damage(Type::Base, 165.0 + 180.0 + 350.0 - 60.0);
    mod_set.add_damage(Type::Slash, 90.0 + 120.0);

    whipclaw.mods = Some(&mod_set);

    whipclaw.print_display();
    whipclaw.print_base_stats();
    whipclaw.print_stats();
}
