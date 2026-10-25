# TODO

## Refactor

- tictactoe game restart
- allow different resolutions, window resizing
- TextureKey: generate in engine
- proper GUI primitives:
  - DONE quad - textured and colored rectangle
  - DONE label - collection of quads, displaying text
  - button - quad with states (future) and label. emits click events
  - DONE make Widget interface to consolidate?
- GUI: id widgets by WidgetKey, similar to TextureKey. or better, use handle with type embedded inside 
- GUI: context emits events to game. but now Widgets can do it. may need some refactoring

## Fix

- no quad ordering. labels on buttons draw under them
