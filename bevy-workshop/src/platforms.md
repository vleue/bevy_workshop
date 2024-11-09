# Platforms Support

## Native

Crossbuilding?

## wasm

* Build steps
    * wasmbindgen-cli
* WebGL2 or WebGPU
* HTML template
    * with audio trick
* Assets should be served as HTTP

## SteamDeck

* Fullscreen

### Gamepad Controls

## Mobile

* Fullscreen

### iOS

* XCode setup

### Android

* Gradle setup

### Touchscreen Controls

Split the touchscreen into zones

#### Action Button

* One zone is "action", in our case jump

#### Direction Stick

* The other is direction. The user start touching at some point, then move right or left: that difference is handled as the direction information.

## Consoles?

* NDA galore
