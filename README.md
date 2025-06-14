# Math Pet

![Sample](./docs/math_pet.png)

# Gameplay

This is a Tamagotchi-like game. To feed the pet, you must answer simple math problems. Correct answers result in the pet being fed, while wrong answers result in the pet losing HP.

The pet will slowly become hungry over time. When its SAT reaches 0, it will begin losing health. If the pet's health reaches 0, it will die, and the game will reset. The old game save is stored internally within the file, but isn't currently used for anything. I think I may add a "resurrection" mechanic at some point.

The game auto-saves on close.

## Shop

SAT can be "spent" in order to tweak some aspects of the game and affect the pet.

- Heal: Heal the pet for 20 HP
- Increase Max SAT: Increase the max SAT of the pet by 2
- Increase Max HP: Increase the max HP of the pet by 2
- Increase Time to Starve: Reduce how quickly the pet loses SAT
- Increase Risk: Increase the amount of SAT recovered for correct answers by 0.2, but increase the amount of health lost for wrong answers by 0.5.


# Running

## REPL

With cargo:

```powershell
PS path> cargo run
```

Running the executable directly after compiling it:

```powershell
PS path> .\math-pet.exe
```

This will open a GUI in a new window.
