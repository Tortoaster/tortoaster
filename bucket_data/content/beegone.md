*Beegone is two-player game similar to chess, but with bees as pieces and played on a honeycomb grid. I made it in 2023 to learn more about WebAssembly and frontend programming.*

## Features

The project has:

* Game logic implemented in Rust, compiled to WebAssembly
* A Svelte frontend that uses the compiled WASM
* Shared types between Rust and TypeScript through [Typeshare](https://github.com/1password/typeshare)
* An SVG GUI, with fancy animations that only work on FireFox because other browsers didn't bother to support them
* As of recent, strange visual artifacts in FireFox due to an unfortunate update
* A computer player that knows *how* to play the game (but not much more than that)

## Rules

The game is played by two players, taking turns. Each player starts with a queen, a drone, and two nurses. During your turn, you can either move a piece, capture an enemy piece or use a piece's special ability.

### Moving

Every piece can move to any of the six tiles adjacent to it, as long as there is no other piece or wall there. Moreover, if a piece is adjacent to a piece of the same team, it can 'hop over' that piece, as long as that tile is accessible (either empty or they can capture the piece standing there). 

### Capturing

Most pieces can capture an enemy piece adjacent to them by moving their piece onto the same tile. However:

* Guards can only capture other guards, gatherers, builders, nurses, workers and drones
* Gatherers can only capture other gatherers, builders, nurses, workers and drones
* Builders can only capture other builders, nurses, workers and drones
* Nurses can only capture other nurses, workers and drones
* Workers can only capture other workers and drones
* Drones can only capture other drones

Summarized, pieces can only capture the same or 'weaker' pieces, where guards > gatherers > builders > nurses > workers > drones. Additionally:

* Queens are unable to capture at all, and cannot be captured normally
* Walls can only be captured by builders

### Special abilities

Some pieces have special abilities:

* Queens can spawn drones on any adjacent tile, or, if a drone is already adjacent, she can spawn workers on any adjacent tile
* Nurses can promote adjacent workers to builders, gatherers or guards
* Gatherers can walk in a straight line any number of tiles, as long as there is nothing blocking the way (they may also do this right after hopping over an ally)
* Builders can place walls on adjacent tiles

### Winning the game

Unfortunately, I never finished the project properly; one minor detail that's still on the backlog is implementing a win condition. The good news is that you can now continue playing indefinitely.

If I were to finish the game, however, I would likely add the following win conditions:

* It's your opponent's turn, but they're unable to make a move
* You capture the opponent's queen with a guard (this would require changing the rules a bit)

This hasn't really been playtested, though. If you want to give it a try, the game can be played [here](https://beegone.tortoaster.com/) (and the source code can be found on [my GitHub](https://github.com/Tortoaster/beegone)). If you have any feedback or ideas on how to make the game more fun, please leave a comment!
