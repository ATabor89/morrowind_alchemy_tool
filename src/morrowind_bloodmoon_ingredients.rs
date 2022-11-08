use crate::ingredient::Ingredient;

pub fn morrowind_bloodmoon_ingredients() -> Vec<Ingredient> {
    let ingredient_string = r"- name: Bear Pelt
  description: Found by killing Bears
  weight: 1.0
  value: 2
  effects:
  - DrainFatigue
  - FortifyStrength
  - ResistCommonDisease
  - NightEye
  harvest_chance: '80'
- name: Bristleback Leather
  description: Found by killing Bristlebacks (with or without riders)
  weight: 1.0
  value: 2
  effects:
  - Blind
  - FrostDamage
  - ResistFrost
  - Recall
  harvest_chance: '60'
- name: Grahl Eyeball
  description: Found by killing Grahls
  weight: 1.0
  value: 15
  effects:
  - ResistFrost
  - NightEye
  - DrainMagicka
  - FortifyStrength
  harvest_chance: '100'
- name: Gravetar
  description: Found by killing Draugr
  weight: 0.1
  value: 5
  effects:
  - ResistFrost
  - DrainHealth
  - FortifyFatigue
  - DrainLuck
  harvest_chance: '60'
- name: Heartwood
  description: Occasionally found by killing Spriggans
  weight: 1.0
  value: 200
  effects:
  - RestoreMagicka
  - FortifyAgility
  - DrainStrength
  - WeaknessToFire
  harvest_chance: '48'
- name: Holly Berries
  description: Grows all over Solstheim
  weight: 0.1
  value: 5
  effects:
  - ResistFrost
  - FrostShield
  - FrostDamage
  - WeaknessToFire
  harvest_chance: '90'
- name: Horker Tusk
  description: Found by killing Horkers
  weight: 0.1
  value: 5
  effects:
  - DrainAlteration
  - FortifyIntelligence
  - FortifyMaximumMagicka
  - DetectAnimal
  harvest_chance: '60'
- name: Raw Stalhrim
  description: Found in barrows. Ancient Stalhrim Pickaxe required.
  weight: 5.0
  value: 300
  effects:
  - ResistFrost
  - FrostDamage
  - Paralyze
  - RestoreHealth
  harvest_chance: '100'
- name: Ripened Belladonna Berries
  description: Somewhat less common than the unripe variety, but still relatively widespread.
  weight: 0.1
  value: 5
  effects:
  - ResistMagicka
  - RestoreMagicka
  - FortifyMagicka
  - DrainMagicka
  harvest_chance: 100/80/60x3
- name: Snow Bear Pelt
  description: Found by killing Snow Bears
  weight: 1.0
  value: 2
  effects:
  - DrainFatigue
  - FortifySpeed
  - ResistCommonDisease
  - NightEye
  harvest_chance: '100'
- name: Snow Wolf Pelt
  description: Found by killing Snow Wolves
  weight: 1.0
  value: 2
  effects:
  - DrainFatigue
  - FortifySpeed
  - ResistCommonDisease
  - NightEye
  harvest_chance: '100'
- name: Unripened Belladonna Berries
  description: Grows all over Solstheim.Common at barrow entrances.
  weight: 0.1
  value: 5
  effects:
  - ResistMagicka
  - RestoreMagicka
  - FortifyMagicka
  - DrainMagicka
  harvest_chance: 100/80/60x3
- name: Wolf Pelt
  description: Found by killing Wolves
  weight: 1.0
  value: 2
  effects:
  - DrainFatigue
  - FortifySpeed
  - ResistCommonDisease
  - NightEye
  harvest_chance: '80'
- name: Wolfsbane Petals
  description: Grows on Hvitkald Peak. One found in Lassnr's shack in the Skaal village.
  weight: 0.1
  value: 5
  effects:
  - RestoreIntelligence
  - Invisibility
  - DrainEndurance
  - DrainMagicka
  harvest_chance: '100'
";

    serde_yaml::from_str(ingredient_string).expect("Unable to deserialize ingredient string")
}
