# Starting Point

## Targets
- Can build native window. Tested on Mac, but should work on Windows and Linux.
- Can build wasm and embed it in html file.
- The above possible thanks to winit and wgpu

## Client architecture
Three main modules:
1. Engine - core engine code. Low-level things live here. Game should never import this.
2. Framework - glue layer between Engine and Game. Allows to drive the engine using higher-level concepts.
3. Game - tic-tac-toe game implementation.

Game loop is managed by winit, but I am trying to abstract it as much as it makes sense. The whole engine should be as much independent from external dependencies as I can manage to do it. 

## Features
- Rendering
  - Two pipelines
  - Canvas pipeline - 2D, designed with GUIs in mind. Can draw simple, textured, 2D primitives.
  - Scene pipeline - 3D. Draws hardcoded spinning cube. Just to see the engine is working.
- Input
  - Stores mouse position
  - Stores mouse button clicks in three-state model
- GUI
  - Game owns widgets and their data
  - One widget available now: button
  - Widget can be textured
  - Widget can be hit-tested
- Some math for coordinate translation etc.
