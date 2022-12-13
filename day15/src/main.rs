use clap::Parser;
use rayon::prelude::*;
use std::hash::Hash;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

#[derive(Clone, PartialEq, Eq)]
struct Recipe {
    ingredients: HashMap<Ingredient, i64>,
}

impl Hash for Recipe {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut ordered_ingredients: Vec<(&Ingredient, &i64)> = self.ingredients.iter().collect();
        ordered_ingredients.sort_by_key(|i| &i.0.name);
        for ingredient in ordered_ingredients {
            ingredient.0.hash(state);
            ingredient.1.hash(state);
        }
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let ingredients: Vec<Ingredient> = reader
        .lines()
        .map(|line| parse_ingredient(line.expect("Failed to parse line")))
        .collect();

    let mut states: HashSet<Recipe> = HashSet::new();
    states.insert(Recipe {
        ingredients: HashMap::new(),
    });
    for _ in 0..100 {
        states = states
            .par_iter()
            .map(|s| {
                ingredients
                    .par_iter()
                    .map(|i| {
                        let mut new_recipe = s.clone();
                        if new_recipe.ingredients.contains_key(i) {
                            *new_recipe.ingredients.get_mut(i).unwrap() += 1;
                        } else {
                            new_recipe.ingredients.insert(i.clone(), 1);
                        }
                        new_recipe
                    })
                    .collect::<Vec<Recipe>>()
            })
            .flatten()
            .collect();

        println!("{}", states.len());
    }
    let best_state = states.par_iter().max_by_key(|s| score_recipe(s)).unwrap();
    let best_score = score_recipe(best_state);
    for ingredient in best_state.ingredients.iter() {
        println!("{}: {}", ingredient.0.name, ingredient.1);
    }

    println!("Recipe score: {}", best_score);

    let best_limited_calorie_recipe = states
        .par_iter()
        .filter(|s| {
            s.ingredients
                .iter()
                .map(|i| i.0.calories * i.1)
                .sum::<i64>()
                == 500
        })
        .max_by_key(|s| score_recipe(s))
        .unwrap();

    println!(
        "Best limited calorie recipe score: {}",
        score_recipe(best_limited_calorie_recipe)
    );
}

fn score_recipe(recipe: &Recipe) -> i64 {
    let total_ingredients = recipe
        .ingredients
        .iter()
        .fold(Ingredient::default(), |acc, i| {
            let mut acc = acc.clone();
            acc.calories += i.0.calories * i.1;
            acc.capacity += i.0.capacity * i.1;
            acc.durability += i.0.durability * i.1;
            acc.flavor += i.0.flavor * i.1;
            acc.texture += i.0.texture * i.1;
            acc
        });

    total_ingredients.capacity.max(0)
        * total_ingredients.durability.max(0)
        * total_ingredients.flavor.max(0)
        * total_ingredients.texture.max(0)
}

fn parse_ingredient(input: String) -> Ingredient {
    let components: Vec<&str> = input.split_whitespace().collect();
    Ingredient {
        name: components[0].trim_end_matches(":").to_string(),
        capacity: components[2].trim_end_matches(",").parse().unwrap(),
        durability: components[4].trim_end_matches(",").parse().unwrap(),
        flavor: components[6].trim_end_matches(",").parse().unwrap(),
        texture: components[8].trim_end_matches(",").parse().unwrap(),
        calories: components[10].parse().unwrap(),
    }
}
