# Morrowind Alchemy Tool

## Check It Out
You can run the tool from your browser at the following link:

https://atabor89.github.io/morrowind_alchemy_tool/

## Introduction
This is a helper tool designed to help the user create potions in the game THe Elder Scrolls III: Morrowind.

## Details
In Morrowind, the player can create any number of potions by combining up to four ingredients. An ingredient can contain up to four effects and any effects that match between at least two ingredients will appear in the resulting potion.

As a player, it can be difficult to remember which ingredients contain the exact effects you're looking for. This tool allows the user to specify the effect or effects that they wish to see in a final potion and then presents all of the possible ingredient combinations which will create that final potion.

Unnecessary potions, such as a three or four-ingredient potion whose effects are the exact same as a two-ingredient potion, are filtered out. The user can also opt to allow extra effects that were not in their initial desired effects. For instance, if the desired effect is a "Restore Health" potion but the resulting potion also contains "Restore Fatigue", this would be permitted after allowing extra effects.

### Notes
The current implementation of this tool may not represent all best practices. While the backend code is relatively straightforward to implement, this project served as an exercise in building and hosting WebAssembly, as well as EGUI itself. Integrating the backend logic into the UI framework posed a different challenge. It is highly likely that improvements can be made and I welcome feedback.
