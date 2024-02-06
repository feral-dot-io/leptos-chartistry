# Development

## Design experiments and notes:

- Series is not a `MaybeSignal` because it requires each Line to specify a key when it's a signal for use in <For>. To do this in a typesafe manner means bigger changes to the API however it doesn't seem to make downstream code better. It still mostly ended up being wrapped in a signal -- just limited to the Series instead. It also complicates the internal logic that would probably make it harder to iterate on future designs. The library's API will probably change.

- Data is a `Vec<T>` to simplify building of line Series. To drop the <T> we could use an IntoIterator on the Series or each line. The has the drawback of complicating the chart internals as it still needs an efficient aggregate model. 
It's not clear that it could be efficient (avoiding extra iterations and copies of data) without impacting API ergonomics. For example, per line: `Iterator<Item = (X, Y)>`, per series: `Iterator<Item = (X, [Y])>` and `[Y] -> Y` per line which implies a generic Item = (X, T) and T -> Y. There are usecases for a data transform step but this looks better suited as a step before the data is passed to the library.

- Colours: need a general write up on difficulties

## TODO

For release:
- Check for TODOs
- Top panel
- Usecase / examples page
- Docs
- Demo needs a fixed size container for an environment aspect ratio
- box-sizing: border-box;

- Make repo public, serve docs/ under pages
- Method to rebuild demo
- Write down dev cycle "run this oneshot command"
- Check nix + trunk = release
- Fix up links
- Release to crates.io

Features to add:
- Stacked line ordering
- Line dots
- Bars
- Stacked bars
- Loading status
- Canvas
    - Calculate font
    - Multi-line labels
    - Can we get background colour?

## Caveats

- Timestamps: Should be reworked to avoid overlapping labels. `iter_aligned_range` should be passed a Duration instead of using Period::increment.
- Assumes light background
