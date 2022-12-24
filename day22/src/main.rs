use std::{cmp::Reverse, collections::BinaryHeap};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    enemy_hp: i32,
    #[arg(long)]
    enemy_damage: i32,
    #[arg(long)]
    hp: i32,
    #[arg(long)]
    mana: i32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct State {
    mana_spent: i32,
    hp: i32,
    mana: i32,
    enemy_hp: i32,
    shield_time: i32,
    poison_time: i32,
    recharge_time: i32,
    history: Vec<i32>,
}

fn main() {
    let args = Args::parse();

    let mut states: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    states.push(Reverse(State {
        mana_spent: 0,
        hp: args.hp,
        mana: args.mana,
        enemy_hp: args.enemy_hp,
        shield_time: 0,
        poison_time: 0,
        recharge_time: 0,
        history: Vec::new(),
    }));

    while !states.is_empty() {
        let mut state = states.pop().unwrap().0;

        if state.enemy_hp <= 0 {
            println!("Part 1: {}", state.mana_spent);
            for i in state.history {
                print!("{}, ", i);
            }
            print!("\n");
            break;
        }

        // START MY TURN

        if state.recharge_time > 0 {
            state.mana += 101;
            state.recharge_time -= 1;
        }

        if state.poison_time > 0 {
            state.enemy_hp -= 3;
            state.poison_time -= 1;
        }

        if state.shield_time > 0 {
            state.shield_time -= 1;
        }

        // Decision time
        for i in 0..5 {
            let mut state = state.clone();
            state.history.push(i);
            if i == 0 {
                // Magic missle
                state.mana_spent += 53;
                state.mana -= 53;
                state.enemy_hp -= 4;
            } else if i == 1 {
                // Drain
                state.mana_spent += 73;
                state.mana -= 73;
                state.enemy_hp -= 2;
                state.hp += 2;
            } else if i == 2 {
                // Shield
                if state.shield_time > 0 {
                    continue;
                }
                state.mana_spent += 113;
                state.mana -= 113;
                state.shield_time = 6;
            } else if i == 3 {
                // Poison
                if state.poison_time > 0 {
                    continue;
                }
                state.mana_spent += 173;
                state.mana -= 173;
                state.poison_time = 6;
            } else if i == 4 {
                // Recharge
                if state.recharge_time > 0 {
                    continue;
                }
                state.mana_spent += 229;
                state.mana -= 229;
                state.recharge_time = 5;
            }

            if state.mana < 0 {
                continue;
            }

            // END MY TURN

            if state.poison_time > 0 {
                state.enemy_hp -= 3;
                state.poison_time -= 1;
            }

            if state.enemy_hp <= 0 {
                states.push(Reverse(state));
                continue;
            }

            // Boss' turn
            if state.recharge_time > 0 {
                state.mana += 101;
                state.recharge_time -= 1;
            }

            if state.shield_time > 0 {
                state.shield_time -= 1;

                state.hp -= (args.enemy_damage - 7).max(1);
            } else {
                state.hp -= args.enemy_damage;
            }

            if state.hp > 0 {
                states.push(Reverse(state));
            }
        }
    }
}
