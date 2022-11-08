use crate::ingredient::Ingredient;

pub fn morrowind_base_game_ingredients() -> Vec<Ingredient> {
    let ingredient_string = r"- name: Alit Hide
  description: Found by killing Alits
  weight: 1.0
  value: 5
  effects:
  - DrainIntelligence
  - ResistPoison
  - Telekinesis
  - DetectAnimal
  harvest_chance: '60'
- name: Ampoule Pod
  description: Common growing in swamps in the Bitter Coast region; one of two possible outputs of the draggle-tail plant (the other is the Coda Flower)
  weight: 0.1
  value: 2
  effects:
  - WaterWalking
  - Paralyze
  - DetectAnimal
  - DrainWillpower
  harvest_chance: '80'
- name: Ash Salts
  description: Found by killing Ash creatures
  weight: 0.1
  value: 25
  effects:
  - DrainAgility
  - ResistMagicka
  - CureBlightDisease
  - ResistMagicka
  harvest_chance: '60'
- name: Ash Yam
  description: Most common in farm areas, occasionally found in the wild
  weight: 0.5
  value: 1
  effects:
  - FortifyIntelligence
  - FortifyStrength
  - ResistCommonDisease
  - DetectKey
  harvest_chance: '80'
- name: Bittergreen Petals
  description: Grows on thorny vines in drier regions
  weight: 0.1
  value: 5
  effects:
  - RestoreIntelligence
  - Invisibility
  - DrainEndurance
  - DrainMagicka
  harvest_chance: '80'
- name: Black Anther
  description: These flowers are most common in the Ascadian Isles region
  weight: 0.1
  value: 2
  effects:
  - DrainAgility
  - ResistFire
  - DrainEndurance
  - Light
  harvest_chance: '80'
- name: Black Lichen
  description: Somewhat rare, mostly found in caves
  weight: 0.1
  value: 2
  effects:
  - DrainStrength
  - ResistFrost
  - DrainSpeed
  - CurePoison
  harvest_chance: '70'
- name: Bloat
  description: Grows in Bloat Spores, which are unfortunately not found anywhere in the game. The only examples you can find are in random containers, or sold by vendors.
  weight: 0.1
  value: 5
  effects:
  - DrainMagicka
  - FortifyIntelligence
  - FortifyWillpower
  - DetectAnimal
  harvest_chance: '100'
- name: Bonemeal
  description: Found on skeletons (living or dead), and also in urns in most tombs.
  weight: 0.2
  value: 2
  effects:
  - RestoreAgility
  - Telekinesis
  - DrainFatigue
  - DrainPersonality
  harvest_chance: '60'
- name: Bread
  description: Found in most taverns and many houses.
  weight: 0.2
  value: 1
  effects:
  - RestoreFatigue
  - null
  - null
  - null
  harvest_chance: N/A
- name: Bungler's Bane
  description: This shelf-fungus grows on tree trunks in the Bitter Coast region
  weight: 0.5
  value: 1
  effects:
  - DrainSpeed
  - DrainEndurance
  - Dispel
  - DrainStrength
  harvest_chance: '90'
- name: Chokeweed
  description: Found growing in drier regions
  weight: 0.1
  value: 1
  effects:
  - DrainLuck
  - RestoreFatigue
  - CureCommonDisease
  - DrainWillpower
  harvest_chance: '90'
- name: Coda Flower
  description: Found growing in swampy pools in the Bitter Coast region on the draggle-tail plant
  weight: 0.1
  value: 23
  effects:
  - DrainPersonality
  - Levitate
  - DrainIntelligence
  - DrainHealth
  harvest_chance: '75'
- name: Comberry
  description: Grows in shrubs in the West Gash and Ascadian Isles regions
  weight: 0.1
  value: 2
  effects:
  - DrainFatigue
  - RestoreMagicka
  - FireShield
  - Reflect
  harvest_chance: '90'
- name: Corkbulb Root
  description: Mostly found in drier regions in the wild.
  weight: 0.1
  value: 5
  effects:
  - CureParalyzation
  - RestoreHealth
  - LightningShield
  - FortifyLuck
  harvest_chance: '80'
- name: Corprus Weepings
  description: Found by killing Corprus Stalkers
  weight: 0.1
  value: 50
  effects:
  - DrainFatigue
  - FortifyLuck
  - DrainWillpower
  - RestoreHealth
  harvest_chance: '60'
- name: Crab Meat
  description: Found by killing Mudcrabs
  weight: 0.5
  value: 1
  effects:
  - RestoreFatigue
  - ResistShock
  - LightningShield
  - RestoreLuck
  harvest_chance: 60x2
- name: Daedra Skin
  description: Somewhat rare, mostly found at vendors
  weight: 0.2
  value: 200
  effects:
  - FortifyStrength
  - CureCommonDisease
  - Paralyze
  - SwiftSwim
  harvest_chance: '60'
- name: Daedra's Heart
  description: Found by killing most Daedra - Dremora, Ogrim, Clannfears, Daedroth, etc.
  weight: 1.0
  value: 200
  effects:
  - RestoreMagicka
  - FortifyEndurance
  - DrainAgility
  - NightEye
  harvest_chance: '60'
- name: Diamond
  description: Common in barrels in Dwemer ruins. Also found in a small number of caves
  weight: 0.2
  value: 250
  effects:
  - DrainAgility
  - Invisibility
  - Reflect
  - DetectKey
  harvest_chance: '60'
- name: Dreugh Wax
  description: Found by killing Dreugh
  weight: 0.2
  value: 100
  effects:
  - FortifyStrength
  - RestoreStrength
  - DrainLuck
  - DrainWillpower
  harvest_chance: '60'
- name: Ectoplasm
  description: Found by killing Ghosts
  weight: 0.1
  value: 10
  effects:
  - FortifyAgility
  - DetectAnimal
  - DrainStrength
  - DrainHealth
  harvest_chance: '60'
- name: Emerald
  description: Common in barrels in Dwemer ruins
  weight: 0.2
  value: 150
  effects:
  - FortifyMagicka
  - RestoreHealth
  - DrainAgility
  - DrainEndurance
  harvest_chance: N/A
- name: Fire Petal
  description: Found growing in the Molag Amur, Ashlands, and Red Mountain regions
  weight: 0.1
  value: 2
  effects:
  - ResistFire
  - DrainHealth
  - SpellAbsorption
  - Paralyze
  harvest_chance: '80'
- name: Fire Salts
  description: Found by killing Flame Atronachs
  weight: 0.1
  value: 100
  effects:
  - DrainHealth
  - FortifyAgility
  - ResistFrost
  - FireShield
  harvest_chance: '60'
- name: Frost Salts
  description: Found by killing Frost Atronachs
  weight: 0.1
  value: 75
  effects:
  - DrainSpeed
  - RestoreMagicka
  - FrostShield
  - ResistFire
  harvest_chance: '60'
- name: Ghoul Heart
  description: Supposedly the heart tissue of an Ash Ghoul, but not found on them. Mostly in random ingredient containers and sold at vendors.
  weight: 0.5
  value: 150
  effects:
  - Paralyze
  - CurePoison
  - FortifyAttack
  - null
  harvest_chance: N/A
- name: Gold Kanet
  description: Found growing in bushes in the grassy regions
  weight: 0.1
  value: 5
  effects:
  - DrainHealth
  - Burden
  - DrainLuck
  - RestoreStrength
  harvest_chance: '80'
- name: Gravedust
  description: No reliable location. Occasionally for sale at vendors or found in random containers
  weight: 0.1
  value: 1
  effects:
  - DrainIntelligence
  - CureCommonDisease
  - DrainMagicka
  - RestoreEndurance
  harvest_chance: N/A
- name: Green Lichen
  description: Mostly grows on rocks in caves
  weight: 0.1
  value: 1
  effects:
  - FortifyPersonality
  - CureCommonDisease
  - DrainStrength
  - DrainHealth
  harvest_chance: '80'
- name: Guar Hide
  description: Found by killing Guars
  weight: 1.0
  value: 5
  effects:
  - DrainFatigue
  - FortifyEndurance
  - RestorePersonality
  - FortifyLuck
  harvest_chance: '60'
- name: Hackle-Lo Leaf
  description: Found as food in most houses; grows wild in the Azura's Coast region.
  weight: 0.1
  value: 30
  effects:
  - RestoreFatigue
  - Paralyze
  - WaterBreathing
  - RestoreLuck
  harvest_chance: '75'
- name: Heather
  description: This common flower grows throughout the greener regions of Vvardenfell.
  weight: 0.1
  value: 1
  effects:
  - RestorePersonality
  - Feather
  - DrainSpeed
  - DrainPersonality
  harvest_chance: '90'
- name: Hound Meat
  description: Found by killing Nix-Hounds
  weight: 1.0
  value: 2
  effects:
  - RestoreFatigue
  - FortifyFatigue
  - Reflect
  - DetectEnchantment
  harvest_chance: '100'
- name: Hypha Facia
  description: This shelf-fungus is found growing on tree trunks in the Bitter Coast region
  weight: 0.1
  value: 1
  effects:
  - DrainLuck
  - DrainAgility
  - DrainFatigue
  - DetectEnchantment
  harvest_chance: '90'
- name: Kagouti Hide
  description: Found by killing Kagoutis
  weight: 1.0
  value: 2
  effects:
  - DrainFatigue
  - FortifySpeed
  - ResistCommonDisease
  - NightEye
  harvest_chance: '60'
- name: Kresh Fiber
  description: Kreshweed is found growing near sea water, most common in the Azura's Coast, Bitter Coast and West Gash regions.
  weight: 0.1
  value: 1
  effects:
  - RestoreLuck
  - FortifyPersonality
  - DrainMagicka
  - DrainSpeed
  harvest_chance: '80'
- name: Kwama Cuttle
  description: Found by killing Kwama (any type).
  weight: 0.1
  value: 2
  effects:
  - ResistPoison
  - DrainFatigue
  - WaterWalking
  - WaterBreathing
  harvest_chance: '60'
- name: Large Kwama Egg
  description: Found in Egg Mines. Also common as food in houses.
  weight: 2.0
  value: 2
  effects:
  - RestoreFatigue
  - Paralyze
  - FrostShield
  - FortifyHealth
  harvest_chance: '60'
- name: Luminous Russula
  description: Found in the Bitter Coast region, also common in caves
  weight: 0.2
  value: 1
  effects:
  - WaterBreathing
  - DrainFatigue
  - Poison
  - null
  harvest_chance: '90'
- name: Marshmerrow
  description: Found growing near water, most common in the Azura's Coast region
  weight: 0.1
  value: 1
  effects:
  - RestoreHealth
  - DetectEnchantment
  - DrainWillpower
  - DrainFatigue
  harvest_chance: '90'
- name: Moon Sugar
  description: Found in smuggler caves. Most vendors will not deal with you if you are carrying this illegal narcotic substance.
  weight: 0.1
  value: 50
  effects:
  - FortifySpeed
  - Dispel
  - DrainEndurance
  - DrainLuck
  harvest_chance: N/A
- name: Muck
  description: Found in Muckspunge plants which grow near water in many regions
  weight: 0.1
  value: 1
  effects:
  - DrainIntelligence
  - DetectKey
  - DrainPersonality
  - CureCommonDisease
  harvest_chance: '80'
- name: Netch Leather
  description: Found by killing Netch
  weight: 1.0
  value: 1
  effects:
  - FortifyEndurance
  - FortifyIntelligence
  - DrainPersonality
  - CureParalyzation
  harvest_chance: '60'
- name: Pearl
  description: Found in kollops on the sea floor. Also found in barrels in Dwemer ruins.
  weight: 0.2
  value: 100
  effects:
  - DrainAgility
  - Dispel
  - WaterBreathing
  - ResistCommonDisease
  harvest_chance: '50'
- name: Racer Plumes
  description: Found by killing Cliff Racers
  weight: 0.1
  value: 20
  effects:
  - DrainWillpower
  - Levitate
  - null
  - null
  harvest_chance: '60'
- name: Rat Meat
  description: Found by killing Rats
  weight: 1.0
  value: 1
  effects:
  - DrainMagicka
  - Paralyze
  - CurePoison
  - ResistPoison
  harvest_chance: '60'
- name: Raw Ebony
  description: Found in Ebony Mines. Usually a crime to take it.
  weight: 10.0
  value: 200
  effects:
  - DrainAgility
  - CurePoison
  - FrostShield
  - RestoreSpeed
  harvest_chance: 50x8
- name: Raw Glass
  description: Found in Glass Mines. Usually a crime to take it.
  weight: 2.0
  value: 200
  effects:
  - DrainIntelligence
  - DrainStrength
  - DrainSpeed
  - FireShield
  harvest_chance: 60x8
- name: Red Lichen
  description: Found growing on rocks, mostly in the Ashlands. Also for sale at vendors.
  weight: 0.1
  value: 25
  effects:
  - DrainSpeed
  - Light
  - CureCommonDisease
  - DrainMagicka
  harvest_chance: '80'
- name: Resin
  description: No reliable location. Occasionally for sale at vendors or found in random containers.
  weight: 0.1
  value: 10
  effects:
  - RestoreHealth
  - RestoreSpeed
  - Burden
  - ResistCommonDisease
  harvest_chance: N/A
- name: Roobrush
  description: Found growing in hilly regions
  weight: 0.1
  value: 1
  effects:
  - DrainWillpower
  - FortifyAgility
  - DrainHealth
  - CurePoison
  harvest_chance: '90'
- name: Ruby
  description: Often found in barrels in Dwemer ruins
  weight: 0.2
  value: 200
  effects:
  - DrainHealth
  - Feather
  - RestoreIntelligence
  - DrainAgility
  harvest_chance: N/A
- name: Saltrice
  description: Found growing near sea water, most common in the Azura's Coast region. Also a common food item in houses.
  weight: 0.1
  value: 1
  effects:
  - RestoreFatigue
  - FortifyMagicka
  - DrainStrength
  - RestoreHealth
  harvest_chance: '90'
- name: Scales
  description: Found by killing Slaughterfish
  weight: 0.2
  value: 2
  effects:
  - DrainPersonality
  - WaterWalking
  - RestoreEndurance
  - SwiftSwim
  harvest_chance: '60'
- name: Scamp Skin
  description: Found by killing Scamps
  weight: 0.1
  value: 10
  effects:
  - DrainMagicka
  - CureParalyzation
  - RestorePersonality
  - RestoreStrength
  harvest_chance: '60'
- name: Scathecraw
  description: Grows in dry, hilly regions
  weight: 0.1
  value: 2
  effects:
  - DrainStrength
  - CurePoison
  - DrainHealth
  - RestoreWillpower
  harvest_chance: '90'
- name: Scrap Metal
  description: Found by killing Dwemer constructs. Also common in barrels and on shelves in Dwemer ruins.
  weight: 10.0
  value: 20
  effects:
  - DrainHealth
  - LightningShield
  - ResistShock
  - RestoreIntelligence
  harvest_chance: '60'
- name: Scrib Jelly
  description: Found by killing Scribs
  weight: 0.1
  value: 10
  effects:
  - FortifyWillpower
  - CurePoison
  - CureBlightDisease
  - RestoreWillpower
  harvest_chance: '60'
- name: Scrib Jerky
  description: Found as a food item in houses.
  weight: 0.2
  value: 5
  effects:
  - RestoreFatigue
  - FortifyFatigue
  - Burden
  - SwiftSwim
  harvest_chance: N/A
- name: Scuttle
  description: Found as a food item in houses.
  weight: 0.1
  value: 10
  effects:
  - RestoreFatigue
  - FortifyFatigue
  - Feather
  - Telekinesis
  harvest_chance: N/A
- name: Shalk Resin
  description: Found by killing Shalks
  weight: 0.1
  value: 50
  effects:
  - DrainFatigue
  - FortifyHealth
  - DrainPersonality
  - FortifySpeed
  harvest_chance: 100/40
- name: Sload Soap
  description: Rare. Mostly found for sale at vendors or in random containers.
  weight: 0.1
  value: 50
  effects:
  - DrainPersonality
  - FortifyAgility
  - FireShield
  - RestoreAgility
  harvest_chance: N/A
- name: Small Kwama Egg
  description: Found in Egg Mines. Also common food item in houses.
  weight: 0.5
  value: 1
  effects:
  - RestoreFatigue
  - null
  - null
  - null
  harvest_chance: 70*
- name: Spore Pod
  description: Found on Slough Ferns growing in the Bitter Coast region
  weight: 0.1
  value: 1
  effects:
  - DrainStrength
  - DrainFatigue
  - DetectKey
  - Paralyze
  harvest_chance: '100'
- name: Stoneflower Petals
  description: These flowers are common in the West Gash and Ascadian Isles regions.
  weight: 0.1
  value: 1
  effects:
  - RestoreStrength
  - FortifyMagicka
  - DrainLuck
  - FortifyPersonality
  harvest_chance: '90'
- name: Trama Root
  description: Found growing in dry, hilly regions
  weight: 0.1
  value: 10
  effects:
  - RestoreWillpower
  - Levitate
  - DrainMagicka
  - DrainSpeed
  harvest_chance: '80'
- name: Vampire Dust
  description: Found by killing Vampires
  weight: 0.1
  value: 500
  effects:
  - FortifyHealth
  - FortifyStrength
  - SpellAbsorption
  - Vampirism
  harvest_chance: '100'
- name: Violet Coprinus
  description: These mushrooms grow throughout the Bitter Coast region, and are also common in caves.
  weight: 0.5
  value: 1
  effects:
  - WaterWalking
  - DrainFatigue
  - Poison
  - null
  harvest_chance: '90'
- name: Void Salts
  description: Found by killing Storm Atronachs or Winged Twilights
  weight: 0.1
  value: 100
  effects:
  - RestoreMagicka
  - SpellAbsorption
  - Paralyze
  - DrainEndurance
  harvest_chance: '60'
- name: Wickwheat
  description: Found growing in the Grazelands region
  weight: 0.1
  value: 1
  effects:
  - RestoreHealth
  - FortifyWillpower
  - Paralyze
  - DamageIntelligence
  harvest_chance: '90'
- name: Willow Anther
  description: Most common in the Ascadian Isles region
  weight: 0.1
  value: 10
  effects:
  - DrainPersonality
  - FrostShield
  - CureCommonDisease
  - CureParalyzation
  harvest_chance: '85'
- name: Large Corprusmeat Hunk
  description: ''
  weight: 1.0
  value: 0
  effects:
  - DrainFatigue
  - DrainHealth
  - DrainMagicka
  - null
  harvest_chance: ''
- name: Large Wrapped Corprusmeat
  description: ''
  weight: 1.0
  value: 0
  effects:
  - DrainFatigue
  - DrainHealth
  - DrainMagicka
  - null
  harvest_chance: ''
- name: Medium Corprusmeat Hunk
  description: ''
  weight: 0.5
  value: 0
  effects:
  - DrainFatigue
  - DrainHealth
  - DrainMagicka
  - null
  harvest_chance: ''
- name: Medium Wrapped Corprusmeat
  description: ''
  weight: 0.5
  value: 0
  effects:
  - DrainFatigue
  - DrainHealth
  - DrainMagicka
  - null
  harvest_chance: ''
- name: Small Corprusmeat Hunk
  description: ''
  weight: 0.2
  value: 0
  effects:
  - DrainFatigue
  - DrainHealth
  - DrainMagicka
  - null
  harvest_chance: ''
- name: Small Wrapped Corprusmeat
  description: ''
  weight: 0.2
  value: 0
  effects:
  - DrainFatigue
  - DrainHealth
  - DrainMagicka
  - null
  harvest_chance: ''
- name: Wrapped Corprusmeat Hunk
  description: ''
  weight: 0.0
  value: 0
  effects:
  - DrainFatigue
  - DrainHealth
  - DrainMagicka
  - null
  harvest_chance: ''
- name: Bread
  description: ''
  weight: 0.2
  value: 1
  effects:
  - RestoreFatigue
  - null
  - null
  - null
  harvest_chance: ''
- name: Daedra's Heart
  description: Found as offerings at Daedric shrines. See Cursed Items for more details.
  weight: 1.0
  value: 200
  effects:
  - RestoreMagicka
  - FortifyEndurance
  - DrainAgility
  - NightEye
  harvest_chance: ''
- name: Diamond
  description: Found as offerings at Daedric shrines. See Cursed Items for more details.
  weight: 0.2
  value: 250
  effects:
  - DrainAgility
  - Invisibility
  - Reflect
  - DetectKey
  harvest_chance: ''
- name: Emerald
  description: Found as offerings at Daedric shrines. See Cursed Items for more details.
  weight: 0.2
  value: 150
  effects:
  - FortifyMagicka
  - RestoreHealth
  - DrainAgility
  - DrainEndurance
  harvest_chance: ''
- name: Girith's Guar Hide
  description: Stolen from Athanden Girith, a trader in the Grazelands. Recover them in the Girith's Stolen Hides quest.
  weight: 1.0
  value: 5
  effects:
  - DrainFatigue
  - FortifyEndurance
  - RestorePersonality
  - FortifyLuck
  harvest_chance: ''
- name: Human Flesh
  description: Very rare. Only found in certain places. One is Palansour.
  weight: 1.0
  value: 1
  effects:
  - FortifyHealth
  - DrainIntelligence
  - DrainPersonality
  - null
  harvest_chance: ''
- name: Marsus' Guar Hide
  description: Stolen from Marsus Tullius, a trader in the Grazelands. Recover them in the Marsus Tullius' Missing Hides quest.
  weight: 1.0
  value: 5
  effects:
  - DrainFatigue
  - FortifyEndurance
  - RestorePersonality
  - FortifyLuck
  harvest_chance: ''
- name: Meteor Slime
  description: Found on Charles the Plant, at Jobasha's Rare Books in Vivec.
  weight: 0.1
  value: 10
  effects:
  - FortifyWillpower
  - CurePoison
  - CureBlightDisease
  - RestoreWillpower
  harvest_chance: ''
- name: Muffin
  description: Carried by Gakkenfeld, an Orc at the Gro-Bagrat Plantation in the Ascadian Isles region.
  weight: 0.2
  value: 1
  effects:
  - RestoreFatigue
  - null
  - null
  - null
  harvest_chance: ''
- name: Pearl
  description: Found as offerings at Daedric shrines. See Cursed Items for more details.
  weight: 0.2
  value: 100
  effects:
  - DrainAgility
  - Dispel
  - WaterBreathing
  - ResistCommonDisease
  harvest_chance: ''
- name: Poison
  description: Very rare. Only a few are known to exist. Some in Palansour, and also in Mudan Grotto.
  weight: 0.1
  value: 0
  effects:
  - WeaknessToPoison
  - DamageHealth
  - DamageFatigue
  - Poison
  harvest_chance: ''
- name: Raw Ebony
  description: Found as offerings at Daedric shrines. See Cursed Items for more details.
  weight: 10.0
  value: 200
  effects:
  - DrainAgility
  - CurePoison
  - FrostShield
  - RestoreSpeed
  harvest_chance: ''
- name: Raw Glass
  description: Found in Beshara, needed for The Angry Trader quest.
  weight: 2.0
  value: 200
  effects:
  - DrainIntelligence
  - DrainStrength
  - DrainSpeed
  - FireShield
  harvest_chance: ''
- name: Roland's Tear
  description: This variation of Gold Kanet grows near Ald Sotha. Needed for the quest of the same name.
  weight: 0.1
  value: 5
  effects:
  - DrainHealth
  - Burden
  - DrainLuck
  - RestoreStrength
  harvest_chance: ''
- name: Ruby
  description: Found as offerings at Daedric shrines. See Cursed Items for more details.
  weight: 0.2
  value: 200
  effects:
  - DrainHealth
  - Feather
  - RestoreIntelligence
  - DrainAgility
  harvest_chance: ''
- name: Treated Bittergreen Petals
  description: Given by Taros Dral of the Morag Tong to poison Balyn Omavel for Mephala's Quest.
  weight: 0.1
  value: 10
  effects:
  - RestoreIntelligence
  - DrainMagicka
  - DrainEndurance
  - Invisibility
  harvest_chance: ''
";

    serde_yaml::from_str(ingredient_string).expect("Unable to deserialize ingredient string")
}
