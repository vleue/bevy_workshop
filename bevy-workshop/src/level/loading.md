# Custom Asset Format

## Asset Format

### Use an Existing Level Editor

For a real project, I strongly recommend [LDtk](https://ldtk.io) (Level Designer toolkit).

### Build Your Own Level Editor

If you have some custom needs, or want complete control, you can build your own level editor that would output the level is some parsable format, for example:

```json
{
    "platforms": [
        {
            "start": 3,
            "end": 7,
            "height": 2
        }
    ],
    "start": [4, 3],
}
```

### The Quick and Dirty Way

LDtk support is not built in, and we won't have time to build a custom editor. Let's go with a basic format that you can manually edit with a good idea of how it should render: emojis to the rescue!

```level

    🙂
  🟩🟩🟩🟩🟩

```

## Asset Loader

## Loading the Level
