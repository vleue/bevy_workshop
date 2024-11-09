# Enemies

This part is left as an exercise to the avid reader. Use it to expand on all you've learned until now. An asset `spritesheet_enemies.png` is provided with some sprites that can be used.

## Add Enemy Locations to the Level

Tips:
* Add a new emoji and place it in the level
* Add a new tile type and parse the emoji to it

## Load Assets and Display Them

Tips:
* Load the new spritesheet in the `load_assets` system
* Add a new marker component
* Spawn the enemy when displaying the level with the marker component

## Add "AI"

You should decide how this enemy will act:
* Will it be stationary?
* Will it walk back and forth on a platform?
* Will it wait for the player to come close then rush to them?

Tips:
* Add a new system with a query on your marker component
* If it needs to know the ground, add a query with the `Ground` entities
* If it needs to know the position of the player, add a query with the `Player` entity

## Collisions With Enemy - Their Death, or Yours

If the enemy touch the player, what happens? Does it depend on the side that was touched? Can enemies be stomped on?

Tips:
* Add a new system with a query on your marker component and another on the `Player` entity
* Compute their AABB and find if they intersects
* Find on which side the player is
* Either kill the enemy (despawn the entity) or the player (switch state back to menu)

## Juice it up!

Enemies are several sprites, use them to show an animation. Add audio effects when they collide with the player. Use a visual effect to change their look when they get killed.
