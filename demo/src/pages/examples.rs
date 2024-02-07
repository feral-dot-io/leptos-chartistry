use leptos::*;
use leptos_chartistry::*;

const ASPECT_RATIO: AspectRatio = AspectRatio::inner_ratio(300.0, 300.0);

struct MyData {
    x: f64,
    y1: f64,
    y2: f64,
}

impl MyData {
    fn new(x: f64, y1: f64, y2: f64) -> Self {
        Self { x, y1, y2 }
    }
}

fn load_data() -> Signal<Vec<MyData>> {
    Signal::derive(|| {
        vec![
            MyData::new(0.0, 1.0, 0.0),
            MyData::new(1.0, 3.0, 1.0),
            MyData::new(2.0, 5.0, 2.5),
            MyData::new(3.0, 6.0, 3.0),
            MyData::new(4.0, 5.0, 3.0),
            MyData::new(5.0, 3.0, 4.0),
            MyData::new(6.0, 2.5, 8.0),
            MyData::new(7.0, 4.0, 6.0),
            MyData::new(8.0, 7.0, 4.5),
            MyData::new(10.0, 9.0, 3.0),
        ]
    })
}

#[component]
pub fn Examples() -> impl IntoView {
    let data = load_data();
    view! {
        <article id="examples">
            <h1>"Examples"</h1>
            <nav>
                <ul class="background-box">
                    <li>
                        <a href="#series">"By chart series"</a>": "
                        <ul>
                            <li><a href="#series-line">"Line charts"</a></li>
                            <li><a href="#series-bar">"Bar charts"</a></li>
                            <li><a href="#series-scatter">"Scatter charts"</a></li>
                        </ul>
                    </li>
                    <li>
                        <a href="#edge">"By edge layout"</a>": "
                        <ul>
                            <li><a href="#edge-legend">"Legend"</a></li>
                            <li><a href="#edge-text">"Text label"</a></li>
                            <li><a href="#edge-ticks">"Tick labels"</a></li>
                        </ul>
                    </li>
                    <li>
                        <a href="#inner">"By inner layout"</a>": "
                        <ul>
                            <li><a href="#inner-axis">"Axis marker"</a></li>
                            <li><a href="#inner-grid">"Grid line"</a></li>
                            <li><a href="#inner-guide">"Guide line"</a></li>
                            <li><a href="#inner-legend">"Legend"</a></li>
                        </ul>
                    </li>
                    <li>
                        <a href="#feature">"By feature"</a>": "
                        <ul>
                            <li><a href="#feature-colour">"Colours"</a></li>
                            <li><a href="#feature-width">"Line widths"</a></li>
                        </ul>
                    </li>
                </ul>
            </nav>

            <div id="series">
                <div id="series-line">
                    <h2>"Line charts"</h2>
                    <div class="card">
                        "todo"
                    </div>
                </div>

                <div id="series-bar">
                    <h2>"Bar charts"</h2>
                    <p>"Planned"</p>
                </div>

                <div id="series-scatter">
                    <h2>"Scatter charts"</h2>
                    <p>"Planned"</p>
                </div>
            </div>

            <h2 id="edge">"Edge layout options"</h2>
            <div class="cards">
                <figure id="edge-legend" class="background-box">
                    <figcaption>
                        <h3>"Legend"</h3>
                        <p>"Add legends to your chart."</p>
                    </figcaption>
                    <ExampleEdgeLegend data=data />
                </figure>
            </div>

        </article>
    }
}

#[component]
fn ExampleEdgeLegend(data: Signal<Vec<MyData>>) -> impl IntoView {
    let series = Series::new(|data: &MyData| data.x)
        .line(Line::new(|data: &MyData| data.y1).with_name("asdf"))
        .line(Line::new(|data: &MyData| data.y2).with_name("qwer"))
        .with_x_range(0.0, 10.0)
        .with_y_range(0.0, 10.0);
    view! {
        <Chart
            aspect_ratio=ASPECT_RATIO
            series=series
            data=data
            // Legends placed on all edges
            top=Legend::middle()
            bottom=Legend::end()
            left=Legend::start()
            right=Legend::end()
        />
    }
}
