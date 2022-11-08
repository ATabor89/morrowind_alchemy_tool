use crate::ingredient::Ingredient;

pub fn morrowind_tribunal_ingredients() -> Vec<Ingredient> {
    let ingredient_string = r"- name: Adamantium Ore
  description: Found in Raw Adamantium Rocks. The best use of Adamantium Ore is making Adamantium Armor rather than creating potions.
  weight: 50.0
  value: 300
  effects:
  - Burden
  - RestoreMagicka
  - Poison
  - Reflect
  harvest_chance: '60'
- name: Durzog Meat
  description: Found by killing Durzogs
  weight: 2.0
  value: 7
  effects:
  - FortifyAgility
  - FortifyStrength
  - Blind
  - DamageMagicka
  harvest_chance: '100'
- name: Golden Sedge Flowers
  description: Found in planters throughout Mournhold
  weight: 1.0
  value: 1
  effects:
  - DrainMagicka
  - FortifyStrength
  - FortifyAttack
  - SwiftSwim
  harvest_chance: '70'
- name: Horn Lily Bulb
  description: Found in planters throughout Mournhold
  weight: 1.0
  value: 1
  effects:
  - ResistParalysis
  - DrainHealth
  - RestoreStrength
  - RestoreEndurance
  harvest_chance: 70x2
- name: Lloramor Spines
  description: Found in planters throughout Mournhold
  weight: 1.0
  value: 1
  effects:
  - SpellAbsorption
  - Invisibility
  - Poison
  - DetectEnchantment
  harvest_chance: 70x2
- name: Meadow Rye
  description: Found in planters throughout Mournhold
  weight: 1.0
  value: 1
  effects:
  - FortifySpeed
  - DamageHealth
  - RestoreSpeed
  - DrainSpeed
  harvest_chance: 100x2
- name: Nirthfly Stalks
  description: Found in planters throughout Mournhold
  weight: 1.0
  value: 1
  effects:
  - DamageHealth
  - FortifySpeed
  - RestoreSpeed
  - DrainSpeed
  harvest_chance: 70x2
- name: Noble Sedge Flowers
  description: Found in planters throughout Mournhold
  weight: 1.0
  value: 1
  effects:
  - DamageHealth
  - RestoreAgility
  - Poison
  - FortifyAgility
  harvest_chance: '70'
- name: Scrib Cabbage
  description: Found in planters throughout Mournhold
  weight: 1.0
  value: 1
  effects:
  - DrainIntelligence
  - DamageHealth
  - RestoreAgility
  - FortifyAgility
  harvest_chance: 70x2
- name: Sweetpulp
  description: Found in planters throughout Mournhold
  weight: 1.0
  value: 1
  effects:
  - Paralyze
  - Levitate
  - ResistParalysis
  - RestoreHealth
  harvest_chance: 70x2
- name: Timsa-Come-By flowers
  description: Found in planters throughout Mournhold
  weight: 1.0
  value: 1
  effects:
  - Dispel
  - ResistParalysis
  - DrainMagicka
  - RestoreEndurance
  harvest_chance: 70x2
";

    serde_yaml::from_str(ingredient_string).expect("Unable to deserialize ingredient string")
}
