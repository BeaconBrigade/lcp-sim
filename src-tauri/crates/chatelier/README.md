# chatelier

`chatelier` is an implementation of [Le Chatelier's Principle](https://en.wikipedia.org/wiki/Le_Chatelier%27s_principle).
It's built on top of [`chem-eq`](https://crates.io/crates/chem-eq).

## Features:

`chatelier` can quantitatively determine the effect of changes in:

- [x] Concentration
- [ ] Temperature
- [ ] Volume
- [ ] Pressure

Temperature can be artificially implemented by setting the k expression of the `System` and then calling `update()` on
the `System`. 

## Usage

View the [examples](https://github.com/beaconbrigade/lcp-sim/tree/master/src-tauri/crates/chatelier/examples) and
[crate documentation](https://docs.rs/chatelier) for more information.