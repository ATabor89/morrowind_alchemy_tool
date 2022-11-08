use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use eframe::egui::{self, Widget};
use serde::{Deserialize, Serialize};

use super::{Effect, Ingredient};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Potion {
    pub ingredients: [Option<Rc<RefCell<Ingredient>>>; 4],
    pub effects: Vec<Effect>,
}

impl Display for Potion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Potion:\n\tIngredients:\n\t\t{}\n\tEffects:\n\t\t{}",
            self.ingredients
                .iter()
                .flatten()
                .map(|ing| ing.borrow().name.clone())
                .collect::<Vec<String>>()
                .join("\n\t\t"),
            self.effects
                .iter()
                .map(|eff| eff.to_string())
                .collect::<Vec<String>>()
                .join("\n\t\t"),
        ))
    }
}

impl Widget for &mut Potion {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.group(|ui| {
            ui.heading("Ingredients: ");
            ui.horizontal(|ui| {
                for ingredient in self.ingredients.iter().flatten() {
                    ui.strong(&ingredient.borrow().name);
                }
            });
            ui.heading("Effects: ");
            ui.horizontal(|ui| {
                for effect in self.effects.iter() {
                    ui.strong(&effect.to_string());
                }
            });
        })
        .response
    }
}

impl Potion {
    pub fn new_potion_from_ingredients(ingredients: &[&Rc<RefCell<Ingredient>>]) -> Potion {
        let mut ingredients: Vec<Option<Rc<RefCell<Ingredient>>>> = ingredients
            .iter()
            .map(|&potential_ingredient| Some(potential_ingredient.clone()))
            .collect();
        // Resize if needed
        while ingredients.len() < 4 {
            ingredients.resize_with(4, || None);
        }

        let effects = Self::effects(&ingredients);

        // Create a potion from the potential_ingredients
        Potion {
            ingredients: ingredients.try_into().unwrap_or([None, None, None, None]),
            effects,
        }
    }

    #[cfg(test)]
    pub fn new_potion_from_optional_ingredients(ingredients: &[Option<Ingredient>]) -> Potion {
        let mut ingredients: Vec<Option<Rc<RefCell<Ingredient>>>> = ingredients
            .iter()
            .flatten()
            .cloned()
            .map(|ingredient| Some(Rc::new(RefCell::new(ingredient))))
            .collect();
        // Resize if needed
        while ingredients.len() < 4 {
            ingredients.resize_with(4, || None);
        }

        let effects = Self::effects(&ingredients);

        // Create a potion from the potential_ingredients
        Potion {
            ingredients: ingredients.try_into().unwrap_or([None, None, None, None]),
            effects,
        }
    }

    fn effects(ingredients: &[Option<Rc<RefCell<Ingredient>>>]) -> Vec<Effect> {
        let mut effects_map: HashMap<Effect, u8> = HashMap::new();
        for ingredient in ingredients.iter().flatten() {
            for effect in ingredient.borrow().effects.iter().flatten() {
                if let Some(times_found) = effects_map.get_mut(effect) {
                    *times_found += 1;
                } else {
                    effects_map.insert(*effect, 1);
                }
            }
        }

        effects_map
            .iter()
            .filter_map(|(effect, times_found)| {
                if *times_found > 1 {
                    Some(*effect)
                } else {
                    None
                }
            })
            .collect()
    }
}
