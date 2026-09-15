# Tic-Tac-Toe v1

## The goal
Local only, one player, no networking, boots directly to game, enemy picks first available cell.

## New engine features
- Input system (mouse position, mouse click)
- GUI system (buttons, hit-testing)
- Asset manager (textures, embedded into binary)
- Texture drawing in canvas renderer
- Fonts
  - Load from ttf file (external dependency :( )
  - Rasterize to texture
  - Quad and Label GUI elements to show text

## Decisions
- GUI is stateless
- GUI is retained
- Game creates GUI widgets
- GUI manager stores and manages the widgets
- Button as first GUI widget. Four states, textured.
- Fonts
  - Font atlas is one, wide texture. Low character height to make the texture load on browser (max 2048px width)
  - Hardcoded charset
  - Assuming every character has the same width

## Next
- Refactoring. Move `fontdue` dependencies to the engine. Textures and fonts should be loaded by the engine, not by framework
- Refactoring. Move `winit` to the engine. Framework should not know anything about window building
- Refactoring. Remove `engine` dependency from the game. It should know only `framework`
- Refactoring. Remove `winit` dependency from the game. It should be somehow passed from the engine (through framework)
- Refactoring. Maybe remove framework context and use global objects? Feels like there is no need for passing the context around everywhere

## Live demo
<canvas id="canvas"></canvas>
<script type="module">
    import init from "./pkg/client.js";
    init().then(() => {
        console.log("WASM Loaded");
    });
</script>
