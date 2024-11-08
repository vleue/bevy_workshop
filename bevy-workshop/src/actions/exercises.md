# Exercises

Don't forget to checkout the branch:

```sh
git checkout 08-action-zones
```

Let's review what was changed: <https://github.com/vleue/bevy_workshop/compare/07-level-loading..08-action-zones>

## Use Fixed Z Indexes

Use fixed values for the z indexes so that the game renders the same every time.

Tips:
* Look for every `from_xyz` and replace the third value by a different index
* Ground in the back, then flag, then player in the front

## Switches

Add a switch zone that enables the flag in the level.

Tips:
* Create a new tile in the level and display it
* Start with the flag disabled
    * Add a `bool` to the component: `Flag(bool)`
    * When `false`, don't trigger winning when reaching the flag
* When getting near the switch, change the `Flag` component to `true`
* You can use different sprites to show whether the flag is enabled or not

## PowerUps

Add an item to pick up that change how the player jumps.

Tips:
* Create a new tile in the level and display it
* When getting near the powerup, add a new component to the player
* In the system controlling player jumps, query for the optional component
* If the component is present, change how jumping behaves
