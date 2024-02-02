# Leptos Chartistry

An extensible charting library for Leptos.

## Review

- https://github.com/cxli233/FriendsDontLetFriends

## TODO

- Colours
    - Need a general write up on difficulties
    - Stacked colours iterate but should use sequential colours with "min step by" fn
    - Default stacked colours is not one colour
- Switch colours to issue a CSS class or raw colour. Try to skip complexity entirely.
- Aspect ratio misapplies options on calculation. e.g., when determining the width and have height+ratio then width options are applied to the height. This implies the outer+height calculation should be removed from the API.
- Tooltip needs a custom formatter. Move to Tick trait. `Fn(&Tick, &dyn TickState<Tick = Tick>) -> String>`. Or possibly rely on TickLabels instead.
- PeriodicTimestamps take a Tz but might work with a fixed offset or naive datetime.
- Chart builder should work with Local instead of Utc.
- Toggling x axis periods results in wonky behaviour.
    - Untoggle year and gaps are left.
    - Keep year, remove a large period (like seconds) and it gets stuck in a very large loop.
    - Clicking around has resulted in labels being left on the chart that don't respond to anything even the chart changing size (?!)
- TickGen is not always uniform dates.
- TickGen is not always uniform for aligned floats?!
- Tooltip formatting
- Colours
- Line dots

## Design experiments and notes:

- Series is not a `MaybeSignal` because it requires each Line to specify a key when it's a signal for use in <For>. To do this in a typesafe manner means bigger changes to the API however it doesn't make downstream code better. It still mostly ended up being wrapped in a signal -- just limited to the Series instead. It also complicates the internal logic that would probably make it harder to iterate on future designs. The library's API will probably change.

- Data is a `Vec<T>` to simplify building of line Series. To drop the <T> we could use an IntoIterator on the Series or each line. The has the drawback of complicating the chart internals as it still needs an efficient aggregate model. 
It's not clear that it could be efficient (avoiding extra iterations and copies of data) without impacting API ergonomics. For example, per line: `Iterator<Item = (X, Y)>`, per series: `Iterator<Item = (X, [Y])>` and `[Y] -> Y` per line which implies a generic Item = (X, T) and T -> Y. There are usecases for a data transform step but this looks better suited as a step before the data is passed to the library.
