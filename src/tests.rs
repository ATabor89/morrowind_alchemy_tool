use super::*;
use strum::IntoEnumIterator;

// use lazy_static::lazy_static;
// lazy_static! {
//     static ref INGREDIENTS: Vec<Ingredient> = {
//         let config_file =
//             File::open("config.yaml").expect("Unable to open config file: config.yaml");
//         let config: Config = serde_yaml::from_reader(BufReader::new(config_file))
//             .expect("Unable to deserialize config file");

//         let mut ingredients = Vec::new();

//         for ingredient_list in config.ingredient_lists {
//             let ingredient_list =
//                 File::open(ingredient_list).expect("Unable to open ingredient list");
//             let mut ingredient_list: Vec<Ingredient> =
//                 serde_yaml::from_reader(BufReader::new(ingredient_list))
//                     .expect("Unable to deserialize ingredient list");
//             ingredients.append(&mut ingredient_list);
//         }
//         ingredients
//     };
// }

mod two_ingredient_tests {
    use super::*;

    #[test]
    fn test_two_ingredient_potion_with_one_effect() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::LightningShield),
                Some(Effect::FortifyLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::DrainAgility),
                Some(Effect::RestoreMagicka),
                Some(Effect::DrainWillpower),
                Some(Effect::RestoreHealth),
            ])),
            None,
            None,
        ]);

        let mut expected_effects = vec![Effect::RestoreHealth];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_two_ingredient_potion_with_two_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::CureParalyzation),
                Some(Effect::RestoreHealth),
                Some(Effect::LightningShield),
                Some(Effect::FortifyLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::DrainFatigue),
                Some(Effect::FortifyLuck),
                Some(Effect::DrainWillpower),
                Some(Effect::RestoreHealth),
            ])),
            None,
            None,
        ]);

        let mut expected_effects = vec![Effect::RestoreHealth, Effect::FortifyLuck];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_two_ingredient_potion_with_three_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::CureParalyzation),
                Some(Effect::RestoreHealth),
                Some(Effect::LightningShield),
                Some(Effect::FortifyLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::LightningShield),
                Some(Effect::FortifyLuck),
                Some(Effect::RestoreHealth),
                None,
            ])),
            None,
            None,
        ]);

        let mut expected_effects = vec![
            Effect::RestoreHealth,
            Effect::LightningShield,
            Effect::FortifyLuck,
        ];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_two_ingredient_potion_with_four_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::CureParalyzation),
                Some(Effect::RestoreHealth),
                Some(Effect::DrainFatigue),
                Some(Effect::FortifyLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::DrainFatigue),
                Some(Effect::FortifyLuck),
                Some(Effect::CureParalyzation),
                Some(Effect::RestoreHealth),
            ])),
            None,
            None,
        ]);

        let mut expected_effects = vec![
            Effect::RestoreHealth,
            Effect::FortifyLuck,
            Effect::DrainFatigue,
            Effect::CureParalyzation,
        ];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_two_ingredient_potion_without_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::RestoreMagicka),
                Some(Effect::RestoreAgility),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::DrainAgility),
                Some(Effect::DrainHealth),
                Some(Effect::DrainWillpower),
                Some(Effect::DrainEndurance),
            ])),
            None,
            None,
        ]);

        assert_eq!(potion.effects.len(), 0);
    }
}

#[test]
fn test_effects_list_is_sorted() {
    let list = Effect::effects_list();
    assert_eq!(list.len(), Effect::iter().count());
    let mut sorted_list = list.clone();
    sorted_list.sort_by_key(|effect| effect.to_string());
    assert_eq!(sorted_list, list);
}

mod three_ingredient_tests {
    use super::*;

    /// This is a bit of a nonsensical test as you would have no reason to create this potion in-game
    /// We should still confirm that the output is as expected
    #[test]
    fn test_three_ingredient_potion_with_one_effect() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::LightningShield),
                Some(Effect::FortifyLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::DrainAgility),
                Some(Effect::RestoreMagicka),
                Some(Effect::DrainWillpower),
                Some(Effect::DrainMagicka),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::DrainEndurance),
                Some(Effect::DrainAlteration),
                Some(Effect::RestoreHealth),
                Some(Effect::DrainLuck),
            ])),
            None,
        ]);

        let mut expected_effects = vec![Effect::RestoreHealth];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_three_ingredient_potion_with_two_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::LightningShield),
                Some(Effect::FortifyLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::LightningShield),
                Some(Effect::RestoreMagicka),
                Some(Effect::DrainWillpower),
                Some(Effect::DrainMagicka),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::DrainEndurance),
                Some(Effect::DrainAlteration),
                Some(Effect::RestoreHealth),
                Some(Effect::DrainLuck),
            ])),
            None,
        ]);

        let mut expected_effects = vec![Effect::LightningShield, Effect::RestoreHealth];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_three_ingredient_potion_with_three_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::LightningShield),
                Some(Effect::FortifyLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::LightningShield),
                Some(Effect::RestoreMagicka),
                Some(Effect::DrainWillpower),
                Some(Effect::DrainMagicka),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreFatigue),
                Some(Effect::DrainAlteration),
                Some(Effect::RestoreHealth),
                Some(Effect::DrainLuck),
            ])),
            None,
        ]);

        let mut expected_effects = vec![
            Effect::LightningShield,
            Effect::RestoreHealth,
            Effect::RestoreFatigue,
        ];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_three_ingredient_potion_with_four_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::LightningShield),
                Some(Effect::FortifyLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::LightningShield),
                Some(Effect::RestoreMagicka),
                Some(Effect::DrainWillpower),
                Some(Effect::DrainMagicka),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreFatigue),
                Some(Effect::DrainAlteration),
                Some(Effect::RestoreHealth),
                Some(Effect::FortifyLuck),
            ])),
            None,
        ]);

        let mut expected_effects = vec![
            Effect::FortifyLuck,
            Effect::LightningShield,
            Effect::RestoreHealth,
            Effect::RestoreFatigue,
        ];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_three_ingredient_potion_with_five_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::DamageHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::LightningShield),
                Some(Effect::FortifyLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::LightningShield),
                Some(Effect::DrainWillpower),
                Some(Effect::DrainMagicka),
                Some(Effect::RestoreFatigue),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::DrainMagicka),
                Some(Effect::RestoreHealth),
                Some(Effect::DrainWillpower),
                Some(Effect::FortifyLuck),
            ])),
            None,
        ]);

        let mut expected_effects = vec![
            Effect::FortifyLuck,
            Effect::LightningShield,
            Effect::RestoreFatigue,
            Effect::DrainWillpower,
            Effect::DrainMagicka,
        ];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_three_ingredient_potion_with_six_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::LightningShield),
                Some(Effect::FortifyLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::LightningShield),
                Some(Effect::DrainWillpower),
                Some(Effect::DrainMagicka),
                Some(Effect::RestoreFatigue),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::DrainMagicka),
                Some(Effect::RestoreHealth),
                Some(Effect::DrainWillpower),
                Some(Effect::FortifyLuck),
            ])),
            None,
        ]);

        let mut expected_effects = vec![
            Effect::FortifyLuck,
            Effect::LightningShield,
            Effect::RestoreHealth,
            Effect::RestoreFatigue,
            Effect::DrainWillpower,
            Effect::DrainMagicka,
        ];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_three_ingredient_potion_with_no_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::LightningShield),
                Some(Effect::FortifyLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::DrainAgility),
                Some(Effect::DrainEndurance),
                Some(Effect::DrainWillpower),
                Some(Effect::DrainMagicka),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::Light),
                Some(Effect::DamageMagicka),
                Some(Effect::NightEye),
                Some(Effect::DamageHealth),
            ])),
            None,
        ]);

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert!(actual_effects.is_empty());
    }
}

mod four_ingredient_tests {
    use super::*;

    #[test]
    fn test_four_ingredient_potion_with_one_effect() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::LightningShield),
                Some(Effect::FortifyLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::ResistCommonDisease),
                Some(Effect::CureBlightDisease),
                Some(Effect::CureCommonDisease),
                Some(Effect::CureParalyzation),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::DrainEndurance),
                Some(Effect::CurePoison),
                Some(Effect::FortifyPersonality),
                Some(Effect::DrainLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::FortifyEndurance),
                Some(Effect::FortifyAttack),
                Some(Effect::ResistCommonDisease),
                Some(Effect::FortifyAgility),
            ])),
        ]);

        let mut expected_effects = vec![Effect::ResistCommonDisease];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_four_ingredient_potion_with_two_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::LightningShield),
                Some(Effect::CurePoison),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::ResistCommonDisease),
                Some(Effect::CureBlightDisease),
                Some(Effect::CureCommonDisease),
                Some(Effect::CureParalyzation),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::DrainEndurance),
                Some(Effect::CurePoison),
                Some(Effect::FortifyPersonality),
                Some(Effect::DrainLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::FortifyEndurance),
                Some(Effect::FortifyAttack),
                Some(Effect::ResistCommonDisease),
                Some(Effect::FortifyAgility),
            ])),
        ]);

        let mut expected_effects = vec![Effect::ResistCommonDisease, Effect::CurePoison];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_four_ingredient_potion_with_three_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::LightningShield),
                Some(Effect::CurePoison),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::ResistCommonDisease),
                Some(Effect::CureBlightDisease),
                Some(Effect::CureCommonDisease),
                Some(Effect::CureParalyzation),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::DrainEndurance),
                Some(Effect::CurePoison),
                Some(Effect::FortifyPersonality),
                Some(Effect::DrainLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::CureCommonDisease),
                Some(Effect::FortifyAttack),
                Some(Effect::ResistCommonDisease),
                Some(Effect::FortifyAgility),
            ])),
        ]);

        let mut expected_effects = vec![
            Effect::ResistCommonDisease,
            Effect::CurePoison,
            Effect::CureCommonDisease,
        ];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_four_ingredient_potion_with_four_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::LightningShield),
                Some(Effect::CurePoison),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::ResistCommonDisease),
                Some(Effect::CureBlightDisease),
                Some(Effect::CureCommonDisease),
                Some(Effect::CureParalyzation),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::CureParalyzation),
                Some(Effect::CurePoison),
                Some(Effect::FortifyPersonality),
                Some(Effect::DrainLuck),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::CureCommonDisease),
                Some(Effect::FortifyAttack),
                Some(Effect::ResistCommonDisease),
                Some(Effect::FortifyAgility),
            ])),
        ]);

        let mut expected_effects = vec![
            Effect::ResistCommonDisease,
            Effect::CureParalyzation,
            Effect::CurePoison,
            Effect::CureCommonDisease,
        ];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_four_ingredient_potion_with_five_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::LightningShield),
                Some(Effect::CurePoison),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::ResistCommonDisease),
                Some(Effect::CureBlightDisease),
                Some(Effect::CureCommonDisease),
                Some(Effect::CureParalyzation),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::CureParalyzation),
                Some(Effect::CurePoison),
                Some(Effect::FortifyPersonality),
                Some(Effect::FortifyAttack),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::CureCommonDisease),
                Some(Effect::FortifyAttack),
                Some(Effect::ResistCommonDisease),
                Some(Effect::FortifyAgility),
            ])),
        ]);

        let mut expected_effects = vec![
            Effect::ResistCommonDisease,
            Effect::CureParalyzation,
            Effect::CurePoison,
            Effect::CureCommonDisease,
            Effect::FortifyAttack,
        ];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_four_ingredient_potion_with_six_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::FortifyPersonality),
                Some(Effect::CurePoison),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::ResistCommonDisease),
                Some(Effect::CureBlightDisease),
                Some(Effect::CureCommonDisease),
                Some(Effect::CureParalyzation),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::CureParalyzation),
                Some(Effect::CurePoison),
                Some(Effect::FortifyPersonality),
                Some(Effect::FortifyAttack),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::CureCommonDisease),
                Some(Effect::FortifyAttack),
                Some(Effect::ResistCommonDisease),
                Some(Effect::FortifyAgility),
            ])),
        ]);

        let mut expected_effects = vec![
            Effect::ResistCommonDisease,
            Effect::CureParalyzation,
            Effect::CurePoison,
            Effect::CureCommonDisease,
            Effect::FortifyAttack,
            Effect::FortifyPersonality,
        ];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_four_ingredient_potion_with_seven_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::RestoreFatigue),
                Some(Effect::FortifyPersonality),
                Some(Effect::CurePoison),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),
                Some(Effect::CurePoison),
                Some(Effect::CureCommonDisease),
                Some(Effect::CureParalyzation),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreFatigue),
                Some(Effect::CureCommonDisease),
                Some(Effect::FortifyAgility),
                Some(Effect::FortifyAttack),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::FortifyPersonality),
                Some(Effect::CureParalyzation),
                Some(Effect::FortifyAgility),
                Some(Effect::ResistCommonDisease),
            ])),
        ]);

        let mut expected_effects = vec![
            Effect::RestoreHealth,
            Effect::RestoreFatigue,
            Effect::FortifyPersonality,
            Effect::CurePoison,
            Effect::CureCommonDisease,
            Effect::CureParalyzation,
            Effect::FortifyAgility,
        ];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_four_ingredient_potion_with_eight_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),      // 1
                Some(Effect::RestoreFatigue),     // 2
                Some(Effect::FortifyPersonality), // 3
                Some(Effect::CurePoison),         // 4
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreHealth),     // 1
                Some(Effect::CurePoison),        // 4
                Some(Effect::CureCommonDisease), // 5
                Some(Effect::CureParalyzation),  // 6
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreFatigue),      // 2
                Some(Effect::CureCommonDisease),   // 5
                Some(Effect::FortifyAgility),      // 7
                Some(Effect::ResistCommonDisease), // 8
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::FortifyPersonality),  // 3
                Some(Effect::CureParalyzation),    // 6
                Some(Effect::FortifyAgility),      // 7
                Some(Effect::ResistCommonDisease), // 8
            ])),
        ]);

        let mut expected_effects = vec![
            Effect::RestoreHealth,
            Effect::RestoreFatigue,
            Effect::FortifyPersonality,
            Effect::CurePoison,
            Effect::CureCommonDisease,
            Effect::CureParalyzation,
            Effect::FortifyAgility,
            Effect::ResistCommonDisease,
        ];
        expected_effects.sort_by_key(|effect| effect.to_string());

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert_eq!(expected_effects.len(), actual_effects.len());

        for (actual_effect, expected_effect) in actual_effects.iter().zip(expected_effects.iter()) {
            assert_eq!(actual_effect, expected_effect);
        }
    }

    #[test]
    fn test_four_ingredient_potion_with_no_effects() {
        let potion = Potion::new_potion_from_optional_ingredients(&[
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::ResistCommonDisease),
                Some(Effect::CureBlightDisease),
                Some(Effect::CureCommonDisease),
                Some(Effect::CureParalyzation),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::CurePoison),
                Some(Effect::DetectAnimal),
                Some(Effect::DetectEnchantment),
                Some(Effect::DetectKey),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::RestoreStrength),
                Some(Effect::Dispel),
                Some(Effect::Feather),
                Some(Effect::FireShield),
            ])),
            Some(Ingredient::new_default_ingredient_with_effects([
                Some(Effect::FortifyAttack),
                Some(Effect::FortifyStrength),
                Some(Effect::FortifyIntelligence),
                Some(Effect::FortifyWillpower),
            ])),
        ]);

        let mut actual_effects = potion.effects;
        actual_effects.sort_by_key(|effect| effect.to_string());

        assert!(actual_effects.is_empty());
    }
}
