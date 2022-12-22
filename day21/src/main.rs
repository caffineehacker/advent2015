use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

fn main() {
    //let args = Args::parse();

    let boss_hp = 109;
    let boss_damage = 8;
    let boss_armor = 2;

    let my_hp = 100;

    // The goal is to spend the least amount of gold and to win
    // The winner can be predicted with boss_hp / (my_damage - boss_armor).min(1) <= my_hp / (boss_damage - my_armor).min(1)

    // Weapons are (gold, damage), must have exactly 1
    let weapons = vec![(8, 4), (10, 5), (25, 6), (40, 7), (74, 8)];

    // Armor are (gold, armor), can have 0 or 1
    let armor = vec![(13, 1), (31, 2), (53, 3), (75, 4), (102, 5)];

    // Rings are (gold, damage, armor), can have 0, 1, or 2
    let rings = vec![
        (20, 0, 1),
        (25, 1, 0),
        (40, 0, 2),
        (50, 2, 0),
        (80, 0, 3),
        (100, 3, 0),
    ];

    do_part1(
        &weapons,
        &armor,
        &rings,
        boss_hp,
        boss_damage,
        boss_armor,
        my_hp,
    );

    do_part2(
        &weapons,
        &armor,
        &rings,
        boss_hp,
        boss_damage,
        boss_armor,
        my_hp,
    );
}

fn do_part1(
    weapons: &Vec<(u32, u32)>,
    armor: &Vec<(u32, u32)>,
    rings: &Vec<(u32, u32, u32)>,
    boss_hp: u32,
    boss_damage: u32,
    boss_armor: u32,
    my_hp: u32,
) {
    let mut best_gold = 1000000;
    for w in 0..weapons.len() {
        let weapon = weapons[w];
        let damage = weapon.1;
        let gold = weapon.0;
        if gold >= best_gold {
            continue;
        }
        for a in 0..=armor.len() {
            let armor = if a == 0 { (0, 0) } else { armor[a - 1] };
            let gold = gold + armor.0;
            if gold >= best_gold {
                break;
            }

            let armor = armor.1;
            for r1 in 0..=rings.len() {
                let ring1 = if r1 == 0 { (0, 0, 0) } else { rings[r1 - 1] };
                let gold = gold + ring1.0;
                if gold >= best_gold {
                    break;
                }

                let damage = damage + ring1.1;
                let armor = armor + ring1.2;
                for r2 in 0..=rings.len() {
                    if r1 == 0 && r2 != 0 || r1 == r2 {
                        continue;
                    }

                    let ring2 = if r2 == 0 { (0, 0, 0) } else { rings[r2 - 1] };
                    let gold = gold + ring2.0;
                    if gold >= best_gold {
                        break;
                    }

                    let damage = damage + ring2.1;
                    let armor = armor + ring2.2;

                    if boss_hp as i32 / (damage as i32 - boss_armor as i32).max(1)
                        <= my_hp as i32 / (boss_damage as i32 - armor as i32).max(1)
                    {
                        best_gold = gold;
                    }
                }
            }
        }
    }

    println!("Best gold win: {}", best_gold);
}

fn do_part2(
    weapons: &Vec<(u32, u32)>,
    armor: &Vec<(u32, u32)>,
    rings: &Vec<(u32, u32, u32)>,
    boss_hp: u32,
    boss_damage: u32,
    boss_armor: u32,
    my_hp: u32,
) {
    let mut worst_gold = 0;
    for w in (0..weapons.len()).rev() {
        let weapon = weapons[w];
        let damage = weapon.1;
        let gold = weapon.0;
        for a in (0..=armor.len()).rev() {
            let armor = if a == 0 { (0, 0) } else { armor[a - 1] };
            let gold = gold + armor.0;

            let armor = armor.1;
            for r1 in (0..=rings.len()).rev() {
                let ring1 = if r1 == 0 { (0, 0, 0) } else { rings[r1 - 1] };
                let gold = gold + ring1.0;

                let damage = damage + ring1.1;
                let armor = armor + ring1.2;
                for r2 in (0..=rings.len()).rev() {
                    if r1 == 0 && r2 != 0 || r1 == r2 {
                        continue;
                    }

                    let ring2 = if r2 == 0 { (0, 0, 0) } else { rings[r2 - 1] };
                    let gold = gold + ring2.0;
                    if gold <= worst_gold {
                        break;
                    }

                    let damage = damage + ring2.1;
                    let armor = armor + ring2.2;

                    if boss_hp as i32 / (damage as i32 - boss_armor as i32).max(1)
                        > my_hp as i32 / (boss_damage as i32 - armor as i32).max(1)
                    {
                        worst_gold = gold;
                    }
                }
            }
        }
    }

    println!("Worst gold loss: {}", worst_gold);
}
