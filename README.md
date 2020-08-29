# mc-population-reverser-rs
Takes a population seed and generates corresponding world seed(s)

In Cargo.toml:

[dependencies]<br>
reverser = { git = "https://github.com/v31l-sys/mc-population-reverser-rs" }

================================================================

In main.rs:<br><br>
use reverser::population_reverser::population_seed_utils::*;

```rust
//2721704043401555507 is the seed that was used to set the Java Random seed state
//in the ChunkRandom set population seed function.

//32 in this case is the ChunkX section coordinate
//64 in this case is the ChunkZ section coordinate

let seeds: Vec<i64> = reverse(2721704043401555507, 32, 64);

assert!(seeds.contains(&(41823749187923 as i64)),
        format!("Failed to get seeds... returned: {:?}", seeds));
```
