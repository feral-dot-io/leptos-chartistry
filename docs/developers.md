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

Features to add:
- Stacked line ordering
- Stacked bars
- Loading status
- Canvas
    - Calculate font
    - Multi-line labels
    - Can we get background colour?
- Arrow head (or big red dot) on line chart + linecap
- Tick:
    - f32 
    - integers
    - Option<Tick>
- Legend:
    - lines with gradients don't render well
    - bar chart should render a block of colour
    - In interpolation_mixed.rs I'd like to show only the named lines
    - Render in pure SVG, not HTML

- Site annoyances
    - Clicking show code doesn't show the chart
    - #aspect ratio section needs a link to docs.rs for context

- Colours:
    - divergent gradient should be able to specify the centre point
    - missing alpha channel

- Tooltip:
    - Expand left_cursor to a more general cursor.
    - Add "tend to centre" cursor

- 0.2 bump:
    - Tooltip.class + possibly elsewhere
    - Move Tooltip.show_x_ticks to TickLabels
    - Consider removing anything that can be controlled exclusively by CSS
        - cursor_distance?
    - Marker.scale should be independent of line width with (use px) a default relative to the line width.

## Development cycle

For the demo site:

```
trunk serve --config ./demo/Trunk.toml --open
```

## Release checklist

- `git checkout -b release-vX.Y.Z`
- Update the version in `leptos-chartistry/Cargo.toml`
- `cargo update`
- Commit
- `cargo semver-checks -p leptos-chartistry`
- `nix flake check` -- success? All systems go!
- `git push --set-upstream origin release-vX.Y.Z`
- Wait for CI, review PR, squash and merge
- `git checkout master; git pull; git tag -a vX.Y.Z; git push --tags`
- `cargo publish -p leptos-chartistry` -- no turning back...
