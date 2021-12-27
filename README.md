Snek
----

A classic "Snake" game, implemented in Rust and Friends.

- Use SDL bindings for graphics
- Have a text console version?
- WASM (emscripten) target?

Rules:
- Board is 2D plane divided into cells.
- Board perimiter is Walls.
- There must always be one Snekberry.
- Snek can face any one of four directions.
- Snek cannot face backwards.
- Snek moves one cell forward in facing direction each tick.
- Snek begins as 3 segments.
- If Snek eat Snekberry, Snek gains a Snek Segment.
- If Snek eat Snekberry, Rock is added at random to empty space on Board.
- If Snek move to Wall, Snek die.
- If Snek move to Rock, Snek die.
- If Snek move to Snek Segment, Snek die.
- Snek lives until Snek dies.
