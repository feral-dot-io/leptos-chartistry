# Development

These are mostly thoughts and internal notes (for now).

## Design experiments, notes, and caveats:

- Series is not a `MaybeSignal` because it requires each Line to specify a key when it's a signal for use in <For>. To do this in a typesafe manner means bigger changes to the API however it doesn't seem to make downstream code better. It still mostly ended up being wrapped in a signal -- just limited to the Series instead. It also complicates the internal logic that would probably make it harder to iterate on future designs. The library's API will probably change.

- Data is a `Vec<T>` to simplify building of line Series. To drop the <T> we could use an IntoIterator on the Series or each line. The has the drawback of complicating the chart internals as it still needs an efficient aggregate model. It's not clear that it could be efficient (avoiding extra iterations and copies of data) without impacting API ergonomics. For example, per line: `Iterator<Item = (X, Y)>`, per series: `Iterator<Item = (X, [Y])>` and `[Y] -> Y` per line which implies a generic Item = (X, T) and T -> Y. There are usecases for a data transform step but this looks better suited as a step before the data is passed to the library.

- Colours: need a general write up on difficulties. Assumes a light background.

- Timestamps: Should be reworked to avoid overlapping labels. `iter_aligned_range` should be passed a Duration instead of using Period::increment.

- Trying to generate timestamps over a long range with a small period (like ns) will result in huge lists being generated. Not fit for purpose. The current release navigates this by avoiding nanoseconds and not expecting a jump from "show me years" to "show me seconds" in the same chart.

- aligned_floats sometimes generates weird values. The printed value doesn't always reflect the co-ordinate. The printed values look rounded and it may just be a visual formatting issue. This is only really noticed when you have a very specific range and handwritten labels would be noticeably better.

- Specifying a Chart's font_width is a notable "why???". The current release relies on sensible defaults.

## TODO

For release:
- Method to rebuild demo
- Write down dev cycle "run this oneshot command"
- Check nix + trunk = release
- Fix up links
- Release to crates.io
- Update AR example with a link to crates.io
- screenshot on readme

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

## Release checklist

- Run `cargo update` and commit.
- Update the version in `Cargo.toml` and commit.
- Run `./demo/release` and commit (requires Nix flakes).
