# echo-frame-engine

**Echo Frame** is a modular engine for actor- and event-based games and simulations, focused on being lightweight, extensible, and easy to integrate with different platforms such as WebAssembly (WASM) and pure Rust applications.

## build

### windows

```sh
make windows
```

### linux
```sh
make linux
```

### macOS x86_64
```sh
make macX86
```

### macOS arm64
```sh
make macARM64
```

### wasm
```sh
make wasm
```

# games

space for game development through the engine

## space-highway

Explore space and survive in a dangerous galaxy with endless possibilities. Explore planets and moons; find life; manage your resources; ally with or fight against factions to upgrade your ship and survive as long as possible.

### quick start
```sh
make wasm
npm run gsh -- run wasm
npm run gsh -- run dev
```

### watching wasm changes
```sh
npm run gsh -- run wasm:watch
```

### build
```sh
npm run gsh -- run build
```
