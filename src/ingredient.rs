use std::fmt::Display;

use eframe::{
    egui::{self, Sense, TextFormat, TextStyle, Widget, WidgetInfo, WidgetText, WidgetType},
    emath::NumExt,
    epaint::{text::LayoutJob, Color32},
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub description: String,
    pub weight: f32,
    pub value: u16,
    pub effects: [Option<Effect>; 4],
    pub harvest_chance: String,
    #[serde(skip)]
    pub selected: bool,
}

impl Ingredient {
    /// This function is used to create a default ingredient with an input effects array/slice
    /// This is used as a utility function for unit testing where only the effects field matters
    #[cfg(test)]
    pub fn new_default_ingredient_with_effects(effects: [Option<Effect>; 4]) -> Ingredient {
        Ingredient {
            name: "".to_string(),
            description: "".to_string(),
            weight: 0.0,
            value: 0,
            effects,
            harvest_chance: "".to_string(),
            selected: false,
        }
    }
}

impl Widget for &mut Ingredient {
    // fn ui(self, ui: &mut egui::Ui) -> egui::Response {
    //     ui.group(|ui| {
    //         ui.heading(&self.name);
    //         ui.strong(&self.description);
    //         ui.strong(format!(
    //             "Weight: {}\tValue: {}\tHarvest Chance: {}",
    //             self.weight, self.value, self.harvest_chance
    //         ));
    //         ui.strong("Effects: ");
    //         ui.horizontal(|ui| {
    //             for effect in self.effects.iter().flatten() {
    //                 ui.strong(&effect.to_string());
    //             }
    //         });
    //     })
    //     .response
    // }

    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Ingredient {
            name,
            description,
            weight,
            value,
            effects,
            harvest_chance,
            selected,
        } = self;

        let button_padding = ui.spacing().button_padding;
        let total_extra = button_padding + button_padding;

        let wrap_width = ui.available_width() - total_extra.x;
        let mut text = LayoutJob::default();
        let mut name_format = TextFormat::simple(
            egui::FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
            ui.style().noninteractive().text_color(),
        );
        name_format.underline = egui::Stroke::new(0.25, Color32::WHITE);
        let format = TextFormat::simple(
            egui::FontId::new(18.0, eframe::epaint::FontFamily::Proportional),
            ui.style().noninteractive().text_color(),
        );
        text.append(&format!("{}\n", name), 0.0, name_format);
        text.append(&format!("{}\n", description,), 0.0, format.clone());
        text.append(
            &format!(
                "{}\n",
                effects
                    .iter()
                    .flatten()
                    .map(|effect| effect.to_string())
                    .join("\t")
            ),
            0.0,
            format.clone(),
        );
        text.append(
            &format!(
                "Weight: {}\tValue: {}\tHarvest Chance: {}",
                weight, value, harvest_chance
            ),
            0.0,
            format,
        );
        let text = WidgetText::from(text);
        let text = text.into_galley(ui, None, wrap_width, TextStyle::Button);

        let mut desired_size = total_extra + text.size();
        desired_size.y = desired_size.y.at_least(ui.spacing().interact_size.y);
        desired_size.x = ui.available_width();
        let (rect, response) = ui.allocate_at_least(desired_size, Sense::click());
        response.widget_info(|| {
            WidgetInfo::selected(WidgetType::SelectableLabel, *selected, text.text())
        });

        if ui.is_rect_visible(response.rect) {
            let text_pos = ui
                .layout()
                .align_size_within_rect(text.size(), rect.shrink2(button_padding))
                .min;

            let visuals = ui.style().interact_selectable(&response, *selected);

            if *selected || response.hovered() || response.has_focus() {
                let rect = rect.expand(visuals.expansion);

                ui.painter()
                    .rect(rect, visuals.rounding, visuals.bg_fill, visuals.bg_stroke);
            }

            text.paint_with_visuals(ui.painter(), text_pos, &visuals);
        }

        response
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize, Hash, Eq, EnumIter)]

pub enum Effect {
    // Chameleon,
    CureBlightDisease,
    CureCommonDisease,
    // CureDisease,
    // CureParalysis,
    CureParalyzation,
    CurePoison,
    DetectAnimal,
    DetectEnchantment,
    DetectKey,
    // DetectLife,
    Dispel,
    Feather,
    FireShield,
    FortifyAttack,
    FortifyStrength,
    FortifyIntelligence,
    FortifyWillpower,
    FortifyAgility,
    FortifySpeed,
    FortifyEndurance,
    FortifyPersonality,
    FortifyLuck,
    FortifyHealth,
    FortifyMagicka,
    FortifyMaximumMagicka,
    FortifyFatigue,
    FrostShield,
    Invisibility,
    Levitate,
    Light,
    LightningShield,
    NightEye,
    Recall,
    Reflect,
    // ReflectDamage,
    // ReflectSpell,
    ResistCommonDisease,
    // ResistDisease,
    ResistFire,
    ResistFrost,
    ResistMagicka,
    ResistParalysis,
    ResistPoison,
    ResistShock,
    RestoreStrength,
    RestoreIntelligence,
    RestoreWillpower,
    RestoreAgility,
    RestoreSpeed,
    RestoreEndurance,
    RestorePersonality,
    RestoreLuck,
    RestoreHealth,
    RestoreMagicka,
    RestoreFatigue,
    // Shield,
    // ShockShield,
    SpellAbsorption,
    SwiftSwim,
    Telekinesis,
    WaterBreathing,
    WaterWalking,
    Blind,
    Burden,
    // DamageStrength,
    DamageIntelligence,
    // DamageWillpower,
    // DamageAgility,
    // DamageSpeed,
    // DamageEndurance,
    // DamagePersonality,
    // DamageLuck,
    DamageHealth,
    DamageMagicka,
    DamageFatigue,
    DrainAlteration,
    DrainStrength,
    DrainIntelligence,
    DrainWillpower,
    DrainAgility,
    DrainSpeed,
    DrainEndurance,
    DrainPersonality,
    DrainLuck,
    DrainHealth,
    DrainMagicka,
    DrainFatigue,
    // FireDamage,
    FrostDamage,
    Paralyze,
    Poison,
    // ShockDamage,
    // Silence,
    Vampirism,
    WeaknessToFire,
    WeaknessToPoison,
    // ModdedEffect(String),
}

impl Effect {
    pub fn effects_list() -> Vec<Effect> {
        Effect::iter()
            .sorted_by(|effect_1, effect_2| effect_1.to_string().cmp(&effect_2.to_string()))
            .collect()
    }
}

impl Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Effect::Chameleon => f.write_str("Chameleon"),
            Effect::CureBlightDisease => f.write_str("Cure Blight Disease"),
            Effect::CureCommonDisease => f.write_str("Cure Common Disease"),
            // Effect::CureDisease => f.write_str("Cure Disease"),
            // Effect::CureParalysis => f.write_str("Cure Paralysis"),
            Effect::CureParalyzation => f.write_str("Cure Paralyzation"),
            Effect::CurePoison => f.write_str("Cure Poison"),
            Effect::DetectAnimal => f.write_str("Detect Animal"),
            Effect::DetectEnchantment => f.write_str("Detect Enchantment"),
            Effect::DetectKey => f.write_str("Detect Key"),
            // Effect::DetectLife => f.write_str("Detect Life"),
            Effect::Dispel => f.write_str("Dispel"),
            Effect::Feather => f.write_str("Feather"),
            Effect::FireShield => f.write_str("Fire Shield"),
            Effect::FortifyAttack => f.write_str("Fortify Attack"),
            Effect::FortifyStrength => f.write_str("Fortify Strength"),
            Effect::FortifyIntelligence => f.write_str("Fortify Intelligence"),
            Effect::FortifyWillpower => f.write_str("Fortify Willpower"),
            Effect::FortifyAgility => f.write_str("Fortify Agility"),
            Effect::FortifySpeed => f.write_str("Fortify Speed"),
            Effect::FortifyEndurance => f.write_str("Fortify Endurance"),
            Effect::FortifyPersonality => f.write_str("Fortify Personality"),
            Effect::FortifyLuck => f.write_str("Fortify Luck"),
            Effect::FortifyHealth => f.write_str("Fortify Health"),
            Effect::FortifyMagicka => f.write_str("Fortify Magicka"),
            Effect::FortifyMaximumMagicka => f.write_str("Fortify Maximum Magicka"),
            Effect::FortifyFatigue => f.write_str("Fortify Fatigue"),
            Effect::FrostShield => f.write_str("Frost Shield"),
            Effect::Invisibility => f.write_str("Invisibility"),
            Effect::Levitate => f.write_str("Levitate"),
            Effect::Light => f.write_str("Light"),
            Effect::LightningShield => f.write_str("Lightning Shield"),
            Effect::NightEye => f.write_str("Night Eye"),
            Effect::Recall => f.write_str("Recall"),
            Effect::Reflect => f.write_str("Reflect"),
            // Effect::ReflectDamage => f.write_str("Reflect Damage"),
            // Effect::ReflectSpell => f.write_str("Reflect Spell"),
            Effect::ResistCommonDisease => f.write_str("Resist Common Disease"),
            // Effect::ResistDisease => f.write_str("Resist Disease"),
            Effect::ResistFire => f.write_str("Resist Fire"),
            Effect::ResistFrost => f.write_str("Resist Frost"),
            Effect::ResistMagicka => f.write_str("Resist Magicka"),
            Effect::ResistParalysis => f.write_str("Resist Paralysis"),
            Effect::ResistPoison => f.write_str("Resist Poison"),
            Effect::ResistShock => f.write_str("Resist Shock"),
            Effect::RestoreStrength => f.write_str("Restore Strength"),
            Effect::RestoreIntelligence => f.write_str("Restore Intelligence"),
            Effect::RestoreWillpower => f.write_str("Restore Willpower"),
            Effect::RestoreAgility => f.write_str("Restore Agility"),
            Effect::RestoreSpeed => f.write_str("Restore Speed"),
            Effect::RestoreEndurance => f.write_str("Restore Endurance"),
            Effect::RestorePersonality => f.write_str("Restore Personality"),
            Effect::RestoreLuck => f.write_str("Restore Luck"),
            Effect::RestoreHealth => f.write_str("Restore Health"),
            Effect::RestoreMagicka => f.write_str("Restore Magicka"),
            Effect::RestoreFatigue => f.write_str("Restore Fatigue"),
            // Effect::Shield => f.write_str("Shield"),
            // Effect::ShockShield => f.write_str("Shock Shield"),
            Effect::SpellAbsorption => f.write_str("Spell Absorption"),
            Effect::SwiftSwim => f.write_str("Swift Swim"),
            Effect::Telekinesis => f.write_str("Telekinesis"),
            Effect::WaterBreathing => f.write_str("Water Breathing"),
            Effect::WaterWalking => f.write_str("Water Walking"),
            Effect::Blind => f.write_str("Blind"),
            Effect::Burden => f.write_str("Burden"),
            // Effect::DamageStrength => f.write_str("Damage Strength"),
            Effect::DamageIntelligence => f.write_str("Damage Intelligence"),
            // Effect::DamageWillpower => f.write_str("Damage Willpower"),
            // Effect::DamageAgility => f.write_str("Damage Agility"),
            // Effect::DamageSpeed => f.write_str("Damage Speed"),
            // Effect::DamageEndurance => f.write_str("Damage Endurance"),
            // Effect::DamagePersonality => f.write_str("Damage Personality"),
            // Effect::DamageLuck => f.write_str("Damage Luck"),
            Effect::DamageHealth => f.write_str("Damage Health"),
            Effect::DamageMagicka => f.write_str("Damage Magicka"),
            Effect::DamageFatigue => f.write_str("Damage Fatigue"),
            Effect::DrainAlteration => f.write_str("Drain Alteration"),
            Effect::DrainStrength => f.write_str("Drain Strength"),
            Effect::DrainIntelligence => f.write_str("Drain Intelligence"),
            Effect::DrainWillpower => f.write_str("Drain Willpower"),
            Effect::DrainAgility => f.write_str("Drain Agility"),
            Effect::DrainSpeed => f.write_str("Drain Speed"),
            Effect::DrainEndurance => f.write_str("Drain Endurance"),
            Effect::DrainPersonality => f.write_str("Drain Personality"),
            Effect::DrainLuck => f.write_str("Drain Luck"),
            Effect::DrainHealth => f.write_str("Drain Health"),
            Effect::DrainMagicka => f.write_str("Drain Magicka"),
            Effect::DrainFatigue => f.write_str("Drain Fatigue"),
            // Effect::FireDamage => f.write_str("Fire Damage"),
            Effect::FrostDamage => f.write_str("Frost Damage"),
            Effect::Paralyze => f.write_str("Paralyze"),
            Effect::Poison => f.write_str("Poison"),
            // Effect::ShockDamage => f.write_str("Shock Damage"),
            // Effect::Silence => f.write_str("Silence"),
            Effect::Vampirism => f.write_str("Vampirism"),
            Effect::WeaknessToFire => f.write_str("Weakness To Fire"),
            Effect::WeaknessToPoison => f.write_str("Weakness To Poison"),
        }
    }
}
