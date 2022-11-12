use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use eframe::egui::{self, Widget};
use egui::{
    text::LayoutJob, Color32, NumExt, Sense, TextFormat, TextStyle, WidgetInfo, WidgetText,
    WidgetType,
};
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
        ui.set_width(ui.available_width());
        let button_padding = ui.spacing().button_padding;
        let total_extra = button_padding + button_padding;

        let wrap_width = ui.available_width() - total_extra.x;
        let mut text = LayoutJob::default();
        let mut heading_format = TextFormat::simple(
            egui::FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
            ui.style().noninteractive().text_color(),
        );
        heading_format.underline = egui::Stroke::new(0.25, Color32::WHITE);
        let format = TextFormat::simple(
            egui::FontId::new(18.0, eframe::epaint::FontFamily::Proportional),
            ui.style().noninteractive().text_color(),
        );
        text.append("Ingredients\n", 0.0, heading_format.clone());
        text.append(
            &format!(
                "{}\n",
                self.ingredients
                    .iter()
                    .flatten()
                    .map(|ingredient| match ingredient.try_borrow() {
                        Ok(ingredient) => ingredient.name.clone(), // TODO: Can we avoid cloning here?
                        Err(_) => String::new(),
                    })
                    .collect::<Vec<_>>()
                    .join("\t")
            ),
            0.0,
            format.clone(),
        );
        text.append("Potion Effects\n", 0.0, heading_format);
        text.append(
            &self
                .effects
                .iter()
                .map(|effect| effect.to_string())
                .collect::<Vec<_>>()
                .join("\t"),
            0.0,
            format,
        );
        let text = WidgetText::from(text);
        let text = text.into_galley(ui, None, wrap_width, TextStyle::Button);

        let mut desired_size = total_extra + text.size();
        desired_size.y = desired_size.y.at_least(ui.spacing().interact_size.y);
        desired_size.x = ui.available_width();
        let (rect, response) = ui.allocate_at_least(desired_size, Sense::click());
        response
            .widget_info(|| WidgetInfo::selected(WidgetType::SelectableLabel, false, text.text()));

        if ui.is_rect_visible(response.rect) {
            let text_pos = ui
                .layout()
                .align_size_within_rect(text.size(), rect.shrink2(button_padding))
                .min;

            let visuals = ui.style().interact_selectable(&response, false);

            text.paint_with_visuals(ui.painter(), text_pos, &visuals);
        }

        response
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
            let Ok(ingredient) = ingredient.try_borrow() else {
                // Unable to borrow so it's better to continue than crash
                continue;
            };
            for effect in ingredient.effects.iter().flatten() {
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
