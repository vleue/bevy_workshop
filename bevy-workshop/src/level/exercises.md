# Exercises

Don't forget to checkout the branch:

```sh
git checkout 07-level-loading
```

Let's review what was changed: <https://github.com/vleue/bevy_workshop/compare/06-basic-game..07-level-loading>

## Handle the Player Starting Position

Spawn the player where there's a smiley 🙂

Tips:
* Return an error if there's more than one 🙂 in the level
* Switch its position by `32.0` on the y axis

## Make a Fun Level

Tips:
* Have fun!

## Try Hot Reloading

Tips:
* It needs to enable a feature on Bevy: `file_watcher`
* Check if the asset changed, then despawn the level and respawn it from the updated file