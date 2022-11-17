#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{cell::RefCell, path::PathBuf, rc::Rc};

#[cfg(not(target_arch = "wasm32"))]
use std::{fs::File, io::BufReader};

#[cfg(test)]
mod tests;

mod ingredient;
use egui::{style::{Widgets, Selection, WidgetVisuals}, Color32, Rounding, Stroke, Visuals};
use ingredient::*;
mod potion;
use itertools::Itertools;
use potion::*;
use serde::{Deserialize, Serialize};

use eframe::{egui::{self, Widget}, epaint::Shadow};

#[cfg(not(target_arch = "wasm32"))]
use eframe::epaint::Vec2;

#[derive(Serialize, Deserialize)]
struct Config {
    ingredient_lists: Vec<PathBuf>,
}

// Native
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(1920.0, 1080.0)),
        ..Default::default()
    };
    // native_options.initial_window_size = Some(Vec2::new(1920.0, 1080.0));
    eframe::run_native(
        "Morrowind Alchemy Tool",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    wasm_logger::init(wasm_logger::Config::default());

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "morrowind_alchemy_tool", // hardcode it
        web_options,
        Box::new(|cc| {
            let app = App::new(cc);
            Box::new(app)
        }),
    )
    .expect("failed to start eframe");
}

#[derive(Debug, Serialize, Deserialize)]
struct App {
    ingredients: Vec<Rc<RefCell<Ingredient>>>,
    desired_effects: [Option<Effect>; 4],
    previous_effects: [Option<Effect>; 4],
    potential_ingredients: Vec<Rc<RefCell<Ingredient>>>,
    filtered_ingredients: Vec<Rc<RefCell<Ingredient>>>,
    potential_potions: Vec<Potion>,
    allow_extra_effects: bool,
    visuals: Visuals,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        cc.egui_ctx.set_visuals(Self::dark());

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // TODO: Resolve this storage after figuring out WASM hosting issue
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        App {
            ingredients: { create_ingredients() },
            desired_effects: [None, None, None, None],
            previous_effects: [None, None, None, None],
            potential_ingredients: Vec::new(),
            filtered_ingredients: Vec::new(),
            potential_potions: Vec::new(),
            allow_extra_effects: false,
            visuals: Self::dark(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn create_ingredients() -> Vec<Rc<RefCell<Ingredient>>> {
    let morrowind_base_game_ingredients: Vec<Ingredient> =
        serde_yaml::from_str(include_str!("../res/Morrowind Base Game Ingredients.yaml"))
            .unwrap_or_else(|_| Vec::new());
    let mut morrowind_tribunal_ingredients: Vec<Ingredient> =
        serde_yaml::from_str(include_str!("../res/Morrowind Tribunal Ingredients.yaml"))
            .unwrap_or_else(|_| Vec::new());
    let mut morrowind_bloodmoon_ingredients: Vec<Ingredient> =
        serde_yaml::from_str(include_str!("../res/Morrowind Bloodmoon Ingredients.yaml"))
            .unwrap_or_else(|_| Vec::new());
    let mut ingredients = morrowind_base_game_ingredients;
    ingredients.append(&mut morrowind_tribunal_ingredients);
    ingredients.append(&mut morrowind_bloodmoon_ingredients);

    let mut ingredients: Vec<Rc<RefCell<Ingredient>>> = ingredients
        .iter()
        .cloned()
        .map(|ingredient| Rc::new(RefCell::new(ingredient)))
        .collect();
    ingredients.sort_by(|ingredient_1, ingredient_2| {
        // Although this never crashed with the simpler .borrow() calls, let's remove the chance
        if let Ok(ingredient_1) = ingredient_1.try_borrow() {
            if let Ok(ingredient_2) = ingredient_2.try_borrow() {
                ingredient_1.name.cmp(&ingredient_2.name)
            } else {
                ingredient_1.name.cmp(&"".to_string())
            }
        } else {
            "".cmp("")
        }
    });

    ingredients
}

#[cfg(not(target_arch = "wasm32"))]
fn create_ingredients() -> Vec<Rc<RefCell<Ingredient>>> {
    let config_path: PathBuf = PathBuf::from("config.yaml");
    let data = std::fs::read_to_string(&config_path).unwrap_or_else(|_| {
        r#"ingredient_lists:
  - res/Morrowind Base Game Ingredients.yaml
  - res/Morrowind Tribunal Ingredients.yaml
  - res/Morrowind Bloodmoon Ingredients.yaml
"#
        .to_string()
    });
    let config = serde_yaml::from_str(&data).unwrap_or_else(|_| Config {
        ingredient_lists: vec![
            PathBuf::from("res/Morrowind Base Game Ingredients.yaml"),
            PathBuf::from("res/Morrowind Tribunal Ingredients.yaml"),
            PathBuf::from("res/Morrowind Bloodmoon Ingredients.yaml"),
        ],
    });

    let mut ingredients = Vec::new();

    for ingredient_list in config.ingredient_lists {
        let ingredient_list = File::open(ingredient_list).expect("Unable to open ingredient list");
        let mut ingredient_list: Vec<Ingredient> =
            serde_yaml::from_reader(BufReader::new(ingredient_list))
                .expect("Unable to deserialize ingredient list");
        ingredients.append(&mut ingredient_list);
    }

    let mut ingredients: Vec<Rc<RefCell<Ingredient>>> = ingredients
        .iter()
        .cloned()
        .map(|ingredient| Rc::new(RefCell::new(ingredient)))
        .collect();
    ingredients.sort_by(|ingredient_1, ingredient_2| {
        // Although this never crashed with the simpler .borrow() calls, let's remove the chance
        if let Ok(ingredient_1) = ingredient_1.try_borrow() {
            if let Ok(ingredient_2) = ingredient_2.try_borrow() {
                ingredient_1.name.cmp(&ingredient_2.name)
            } else {
                ingredient_1.name.cmp(&"".to_string())
            }
        } else {
            "".cmp("")
        }
    });

    ingredients
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
            ui.horizontal(|ui| {
                ui.heading("Morrowind Alchemy Tool");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    // custom_global_dark_light_mode_buttons(ui);
                    self.custom_global_dark_light_mode_buttons(ui);
                    // egui::widgets::global_dark_light_mode_buttons(ui);
                });
            });

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.create_effect_dropdown(ui, "Desired Effect 1", 0);
            self.create_effect_dropdown(ui, "Desired Effect 2", 1);
            self.create_effect_dropdown(ui, "Desired Effect 3", 2);
            self.create_effect_dropdown(ui, "Desired Effect 4", 3);
            if ui.checkbox(&mut self.allow_extra_effects, "Allow Extra Effects In Potion Generation").changed() {
                // We have changed this modifier so we should generate potions
                self.generate_potions();
            }
            if !self.desired_effects.iter().zip(self.previous_effects.iter()).all(|(current_effect, previous_effect)| current_effect == previous_effect) {
                // Some effect changed, reset values
                self.potential_ingredients = get_potential_ingredients(&self.desired_effects, &self.ingredients);
                // Unselect ingredients
                for ingredient in self.potential_ingredients.iter_mut() {
                    match ingredient.try_borrow_mut() {
                        Ok(mut ingredient) => ingredient.selected = false,
                        Err(_) => continue, // Unable to borrow ingredient so continuing is better than crashing
                    }
                }
                self.potential_potions.clear();
                self.previous_effects = self.desired_effects;
            }
            ui.separator();
            if !self.potential_ingredients.is_empty() {
                ui.heading("Click ingredients to select for use in final potions, or use the buttons below to select all or none.");
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    if ui.button("Select All").clicked() {
                        for ingredient in self.potential_ingredients.iter_mut() {
                            match ingredient.try_borrow_mut() {
                                Ok(mut ingredient) => ingredient.selected = true,
                                Err(_) => continue, // Unable to borrow ingredient so continuing is better than crashing
                            }
                        }
                        // We have changed the selected ingredients, so let's generate potions
                        self.generate_potions();
                    };
                    if ui.button("Select None").clicked() {
                        for ingredient in self.potential_ingredients.iter_mut() {
                            match ingredient.try_borrow_mut() {
                                Ok(mut ingredient) => ingredient.selected = false,
                                Err(_) => continue, // Unable to borrow ingredient so continuing is better than crashing
                            }
                        }
                        // We have changed the selected ingredients, so let's generate potions
                        self.generate_potions();
                    };
                });
                ui.separator();
            }
            ui.heading("Potential Ingredients");
            ui.separator();
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                self.create_ingredient_area(ui);
                ui.add_space(10.0);
                ui.separator();
                ui.heading("Generated Potions");
                ui.separator();
                self.create_potion_area(ui);
            });    
        });
        
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                let taco_button = ui.button("Buy Me A Taco");
                if taco_button.clicked() || taco_button.middle_clicked() {
                    ui.ctx().output().open_url = Some(egui::output::OpenUrl {
                        url: "https://ko-fi.com/atabor89".to_string(),
                        new_tab: true,
                    });
                }
            });
        });
    }
}

impl App {
    fn custom_global_dark_light_mode_buttons(&mut self, ui: &mut egui::Ui) {
        let visuals = &mut ui.ctx().style().visuals.clone();
        ui.horizontal(|ui| {
            ui.selectable_value(visuals, Self::light(), "☀ Light");
            ui.selectable_value(visuals, Self::dark(), "🌙 Dark");
        });
        ui.ctx().set_visuals(visuals.clone());
    }

    /// Default dark theme.
    pub fn dark() -> egui::Visuals {
        egui::Visuals {
            dark_mode: true,
            override_text_color: None,
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: Color32::from_gray(27), // window background
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(60)), // separators, indentation lines, windows outlines
                    // fg_stroke: Stroke::new(1.0, Color32::from_gray(140)), // normal text color
                    fg_stroke: Stroke::new(1.0, Color32::WHITE), // normal text color
                    rounding: Rounding::same(2.0),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_gray(60), // button background
                    bg_stroke: Default::default(),
                    fg_stroke: Stroke::new(1.0, Color32::from_gray(180)), // button text
                    rounding: Rounding::same(2.0),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_gray(70),
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(150)), // e.g. hover over window edge or button
                    fg_stroke: Stroke::new(1.5, Color32::from_gray(240)),
                    rounding: Rounding::same(3.0),
                    expansion: 1.0,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_gray(55),
                    bg_stroke: Stroke::new(1.0, Color32::WHITE),
                    fg_stroke: Stroke::new(2.0, Color32::WHITE),
                    rounding: Rounding::same(2.0),
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    bg_fill: Color32::from_gray(27),
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(60)),
                    fg_stroke: Stroke::new(1.0, Color32::from_gray(210)),
                    rounding: Rounding::same(2.0),
                    expansion: 0.0,
                },
            },
            selection: Selection {
                bg_fill: Color32::DARK_GRAY, //Color32::from_rgb(0, 92, 128)
                stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)), // Original color: Color32::from_rgb(192, 222, 255)
            },
            hyperlink_color: Color32::from_rgb(90, 170, 255),
            faint_bg_color: Color32::from_gray(35),
            extreme_bg_color: Color32::from_gray(10), // e.g. TextEdit background
            code_bg_color: Color32::from_gray(64),
            warn_fg_color: Color32::from_rgb(255, 143, 0), // orange
            error_fg_color: Color32::from_rgb(255, 0, 0),  // red
            window_rounding: Rounding::same(6.0),
            window_shadow: Shadow::big_dark(),
            popup_shadow: Shadow::small_dark(),
            resize_corner_size: 12.0,
            text_cursor_width: 2.0,
            text_cursor_preview: false,
            clip_rect_margin: 3.0, // should be at least half the size of the widest frame stroke + max WidgetVisuals::expansion
            button_frame: true,
            collapsing_header_frame: false,
        }
    }

    /// Default light theme.
    pub fn light() -> egui::Visuals {
        egui::Visuals {
            dark_mode: false,
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: Color32::from_gray(248), // window background - should be distinct from TextEdit background
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(190)), // separators, indentation lines, windows outlines
                    fg_stroke: Stroke::new(1.0, Color32::BLACK),  // normal text color; original color: Color32::from_gray(80)
                    rounding: Rounding::same(2.0),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_gray(230), // button background
                    bg_stroke: Default::default(),
                    fg_stroke: Stroke::new(1.0, Color32::from_gray(60)), // button text
                    rounding: Rounding::same(2.0),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: Color32::from_gray(220),
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(105)), // e.g. hover over window edge or button
                    fg_stroke: Stroke::new(1.5, Color32::BLACK),
                    rounding: Rounding::same(3.0),
                    expansion: 1.0,
                },
                active: WidgetVisuals {
                    bg_fill: Color32::from_gray(165),
                    bg_stroke: Stroke::new(1.0, Color32::BLACK),
                    fg_stroke: Stroke::new(2.0, Color32::BLACK),
                    rounding: Rounding::same(2.0),
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    bg_fill: Color32::from_gray(220),
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(160)),
                    fg_stroke: Stroke::new(1.0, Color32::BLACK),
                    rounding: Rounding::same(2.0),
                    expansion: 0.0,
                },
            },
            selection: Selection {
                bg_fill: Color32::from_rgb(144, 209, 255),
                stroke: Stroke::new(1.0, Color32::from_rgb(0, 83, 125)),
            },
            hyperlink_color: Color32::from_rgb(0, 155, 255),
            faint_bg_color: Color32::from_gray(242),
            extreme_bg_color: Color32::from_gray(255), // e.g. TextEdit background
            code_bg_color: Color32::from_gray(230),
            warn_fg_color: Color32::from_rgb(255, 0, 0), // red also, beecause orange doesn't look great because of https://github.com/emilk/egui/issues/1455
            error_fg_color: Color32::from_rgb(255, 0, 0), // red
            window_shadow: Shadow::big_light(),
            popup_shadow: Shadow::small_light(),
            ..Self::dark()
        }
    }

    fn create_ingredient_area(&mut self, ui: &mut egui::Ui) {
        if self.potential_ingredients.is_empty() {
            ui.group(|ui| {
                egui::ScrollArea::vertical()
                .id_source("no_ingredient_area")
                .max_height(if self.potential_potions.is_empty() {
                    ui.available_height() - 120.0
                } else {
                    ui.available_height() / 3.0 })
                .show(ui, |ui| {
                    ui.heading("No Ingredients Found - Select One or More Desired Effects");
                });
                });
            } else {
                ui.group(|ui| {
                    egui::ScrollArea::vertical()
                    .max_height(if self.potential_potions.is_empty() {
                        ui.available_height() - 120.0
                    } else {
                        ui.available_height() / 3.0 })
                        .id_source("ingredient_scroll_area")
                        .show(ui, |ui| {
                        let num_ingredients = self.potential_ingredients.len();
                    let mut ingredient_selection_changed = false;
                    for (index, ingredient) in self.potential_ingredients.iter_mut().enumerate() {
                        let Ok(mut ingredient) = ingredient.try_borrow_mut() else {
                            // Unable to borrow ingredient so continue to the next one
                            // It's better than crashing
                            continue;  
                        };
                        if ingredient.ui(ui)
                            .clicked()
                        {
                            ingredient.selected = !ingredient.selected;
    
                            ingredient_selection_changed = true;
                        }
        
                        if index != num_ingredients - 1 {
                            ui.separator();
                        }
                    }
    
                    if ingredient_selection_changed {
                        // We have changed the selected ingredients, so let's generate potions
                        self.generate_potions();
                    }
                });
            });
        }
    }

    fn create_potion_area(&mut self, ui: &mut egui::Ui) {
        if self.potential_potions.is_empty() {
            ui.group(|ui| {
                egui::ScrollArea::vertical()
                    .id_source("no_potion_area")
                    .max_height(ui.available_height() - 10.0)
                    .show(ui, |ui| {
                        ui.heading("No Potions Found - Add More Ingredients, Change Desired Effects, or Allow Extra Effects");
                    });
            });
        }
        else {
            ui.group(|ui| {
                egui::ScrollArea::vertical()
                    .id_source("potion_scroll_area")
                    .max_height(ui.available_height() - 10.0)
                    .show(ui, |ui| {
                        let num_potions = self.potential_potions.len();
                        for (index, potion) in self.potential_potions.iter_mut().enumerate() {
                            potion.ui(ui);
                            if index != num_potions - 1 {
                                ui.separator();
                            }
                        }
                    });
            });
        }
    }

    fn generate_potions(&mut self) {
        self.filtered_ingredients = self
            .potential_ingredients
            .iter()
            .filter(|potential_ingredient| {
                let Ok(potential_ingredient) = potential_ingredient.try_borrow() else {
                    // Unable to borrow ingredient so just return false
                    // It's better than crashing
                    return false;  
                };
                potential_ingredient.selected
            })
            .cloned()
            .collect();
        self.potential_potions = create_potential_potions(&self.desired_effects, &self.filtered_ingredients).iter().filter(|potential_potion| {
                if self.allow_extra_effects {
                    true
                } else {
                        potential_potion.effects.iter().all(|&effect| self.desired_effects.contains(&Some(effect)))
                }
            })
            .sorted_by(|potion_a, potion_b| {
                potion_a.ingredients.len().cmp(&potion_b.ingredients.len())
            })
            .cloned()
            .collect();
        let two_ingredient_potions: Vec<&Potion> = self.potential_potions.iter().filter(|potion| potion.ingredients.iter().flatten().count() == 2).collect();
        let mut three_ingredient_potions: Vec<&Potion> = self.potential_potions.iter().filter(|potion| potion.ingredients.iter().flatten().count() == 3).collect();
        three_ingredient_potions.retain(|three_ingredient_potion| {
                // If we find any exact match, it's an old potion
                for two_ingredient_potion in two_ingredient_potions.iter() {
                    if three_ingredient_potion.effects.iter().all(|effect| {
                        two_ingredient_potion.effects.contains(effect)
                    }) {
                        return false;
                    }
                }

                true
            });
        let mut four_ingredient_potions: Vec<&Potion> = self.potential_potions.iter().filter(|potion| potion.ingredients.iter().flatten().count() == 4).collect();
        four_ingredient_potions.retain(|four_ingredient_potion| {
                for two_ingredient_potion in two_ingredient_potions.iter() {
                    if four_ingredient_potion.effects.iter().all(|effect| {
                        two_ingredient_potion.effects.contains(effect)
                    }) {
                        return false;
                    }
                }

                true
            });
        four_ingredient_potions.retain(|four_ingredient_potion| {
                for three_ingredient_potion in three_ingredient_potions.iter() {
                    if four_ingredient_potion.effects.iter().all(|effect| {
                        three_ingredient_potion.effects.contains(effect)
                    }) {
                        return false;
                    }
                }

                true
            });
        self.potential_potions = {
                let mut t: Vec<Potion> = Vec::new();
                t.append(&mut two_ingredient_potions.iter().cloned().cloned().collect());
                t.append(&mut three_ingredient_potions.iter().cloned().cloned().collect());
                t.append(&mut four_ingredient_potions.iter().cloned().cloned().collect());

                t
            }
    }

    fn create_effect_dropdown(&mut self, ui: &mut egui::Ui, label: &str, effect_index: usize) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.heading(format!("{}: ", label));
            egui::ComboBox::from_id_source(label)
                .selected_text(if let Some(effect) = self.desired_effects[effect_index] {
                    effect.to_string()
                } else {
                    String::from("None")
                })
                .width(160.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.desired_effects[effect_index], None, "None");
                    for effect in Effect::effects_list().iter() {
                        ui.selectable_value(
                            &mut self.desired_effects[effect_index],
                            Some(*effect),
                            effect.to_string(),
                        );
                    }
                });
        });
        ui.end_row();
    }
}

fn get_potential_ingredients(
    desired_effects: &[Option<Effect>; 4],
    ingredients: &[Rc<RefCell<Ingredient>>],
) -> Vec<Rc<RefCell<Ingredient>>> {
    let desired_effects: Vec<&Effect> = desired_effects.iter().flatten().collect();
    let potential_ingredients: Vec<Rc<RefCell<Ingredient>>> = ingredients
        .iter()
        .filter(|ingredient| {
            // filter the ingredients iterator
            let Ok(ingredient) = ingredient.try_borrow() else {
                // Skip ingredient by returning false if we are unable to borrow
                return false;
            };
            ingredient
                .effects // get the effects array
                .iter() // and grab an iterator to that
                .flatten() // flatten to get a new iterator, removing any None variant, and ripping out the Effect from Option<Effect>
                .filter(|ingredient_effect| { // filter the flattened iterator of the ingredient's effects
                    desired_effects.contains(ingredient_effect) // If the current ingredient_effect is contained in the desired_effects, we have a match for the filter
                })
                .count() // Count the number of effects
                > 0 // If we have more than 0 matched effects, this ingredient can be used to make a potion with at least one desired effect
        })
        .cloned()
        .collect();
    potential_ingredients
}

fn create_potential_potions(
    desired_effects: &[Option<Effect>; 4],
    potential_ingredients: &[Rc<RefCell<Ingredient>>],
) -> Vec<Potion> {
    // Convert the user input, containing possible None variants, into a Vector of &Effect (removing None variants)
    let desired_effects: Vec<&Effect> = desired_effects.iter().flatten().collect();
    let mut potions: Vec<Potion> = Vec::new();

    // For combinations of 2, 3, and 4 ingredients
    for i in 2..=4 {
        let mut potential_potions: Vec<Potion> = potential_ingredients
            .iter() // iterate over the ingredients // Clone to remove the reference
            .combinations(i) // Create combinations of ingredients
            .filter_map(|ingredient_combo| {
                // create a new potion from the ingredient_combo
                let potential_potion =
                    Potion::new_potion_from_ingredients(ingredient_combo.as_slice());

                // Get the resulting effects of the potential_potion
                let potential_potion_effects = &potential_potion.effects;

                // if all of the desired effects are contained within the potential_potion_effects
                if desired_effects
                    .iter()
                    .all(|desired_effect| potential_potion_effects.contains(desired_effect))
                {
                    // return the potential_potion
                    Some(potential_potion)
                } else {
                    None
                }
            })
            .collect();

        potions.append(&mut potential_potions);
    }

    potions
}
