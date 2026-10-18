## Shinobi Optimiser

Performs a shallow brute force search to maximise the DPS output of a jitsu + shinbi setup

### Usage

Three input json files are needed:

- `pops.json` should contain the full jitsu setup and pop counts for a specific test round
- `pops_shinobi.json` should contain the full shinbi setup and pop counts for the same test round
- `actual.json` can be a copy of one of the files above, or an initial setup to optimise from

Then to optimise with to some depth, e.g. 2, run 

```bash
cargo r -r path/to/jsons/directory/ 2
```

Pressing `F` will run a full optimisation of the setup, pressing `o` will run through optimisation passes one at a time.

There is heavily diminishing returns above depth 2 or 3 because of the brute force nature of the search used. I might look into other algorithms in the future (not guaranteed).
