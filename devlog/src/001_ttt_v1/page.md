# Tic-Tac-Toe v1

## The goal
Local only, one player, no networking, boots directly to game, enemy picks first available cell.

## New engine features
- Input system (mouse position, mouse click)
- GUI system (buttons, hit-testing)
- Asset manager (textures, embedded into binary)
- Texture drawing in canvas renderer

## Decisions
- GUI is stateless
- GUI is retained
- Game creates GUI widgets
- GUI manager stores and manages the widgets
- Button as first GUI widget. Four states, textured.

## Next
- Text rendering (show winner, text on 'restart' button)

## Live demo
<canvas id="canvas"></canvas>
<script type="module">
    import init from "./pkg/client.js";
    init().then(() => {
        console.log("WASM Loaded");
    });
</script>
