use chrono::prelude::*;
use leptos::prelude::*;
use leptos_chartistry::*;

#[derive(Clone, Debug)]
struct DataPoint {
    time: &'static str,
    value: f64,
    field: &'static str,
    epochtime: u64,
}

#[derive(Clone, Debug)]
struct DataPoint2 {
    time: &'static str,
    parsed_time: DateTime<Utc>,
    value: f64,
    field: &'static str,
    epochtime: u64,
}

#[component]
pub fn App() -> impl IntoView {
    let title = "Conductivity".to_string();

    // Parse data ahead of time
    let mut data_points2 = TEST_DATA
        .iter()
        .map(|p| {
            let parsed_time = p.time.parse::<DateTime<Utc>>().unwrap();
            DataPoint2 {
                time: p.time,
                parsed_time,
                value: p.value,
                field: p.field,
                epochtime: p.epochtime,
            }
        })
        .collect::<Vec<_>>();
    // Now we can sort the result by our parsed time
    data_points2.sort_by_key(|p| p.parsed_time);

    let series = Series::new(|p: &DataPoint2| p.parsed_time)
        .line(Line::new(|p: &DataPoint2| p.value).with_name("Conductivity"));

    for point in &data_points2 {
        log::debug!(
            "DataPoint2: time={:?}, parsed={:?}",
            point.time,
            point.parsed_time,
        );
    }

    view! {
        <div class="chart-container">
            <Chart
                aspect_ratio=AspectRatio::from_outer_height(800.0, 1.2)
                series=series
                data=data_points2
                top=RotatedLabel::middle(title)
                left=TickLabels::aligned_floats()
                bottom=TickLabels::timestamps()
                inner=[
                    AxisMarker::left_edge().into_inner(),
                    AxisMarker::bottom_edge().into_inner(),
                    XGridLine::default().into_inner(),
                    YGridLine::default().into_inner(),
                    YGuideLine::over_mouse().into_inner(),
                    XGuideLine::over_data().into_inner(),
                ]
                tooltip=Tooltip::left_cursor().show_x_ticks(true)
            />
        </div>
    }
}

const TEST_DATA: &[DataPoint] = &[
    DataPoint {
        time: "2025-05-15T10:22:00Z",
        value: 8.308056,
        field: "ph",
        epochtime: 1747304520000,
    },
    DataPoint {
        time: "2025-05-15T10:23:00Z",
        value: 8.299055,
        field: "ph",
        epochtime: 1747304580000,
    },
    DataPoint {
        time: "2025-05-15T10:24:00Z",
        value: 8.296056,
        field: "ph",
        epochtime: 1747304640000,
    },
    DataPoint {
        time: "2025-05-15T10:25:00Z",
        value: 8.297052,
        field: "ph",
        epochtime: 1747304700000,
    },
    DataPoint {
        time: "2025-05-15T10:26:00Z",
        value: 8.302062,
        field: "ph",
        epochtime: 1747304760000,
    },
    DataPoint {
        time: "2025-05-15T10:27:00Z",
        value: 8.308062,
        field: "ph",
        epochtime: 1747304820000,
    },
    DataPoint {
        time: "2025-05-15T10:28:00Z",
        value: 8.31105,
        field: "ph",
        epochtime: 1747304880000,
    },
    DataPoint {
        time: "2025-05-16T07:00:00Z",
        value: 7.625041,
        field: "ph",
        epochtime: 1747378800000,
    },
    DataPoint {
        time: "2025-05-16T07:01:00Z",
        value: 7.633033,
        field: "ph",
        epochtime: 1747378860000,
    },
    DataPoint {
        time: "2025-05-16T07:02:00Z",
        value: 7.630042,
        field: "ph",
        epochtime: 1747378920000,
    },
    DataPoint {
        time: "2025-05-16T07:03:00Z",
        value: 7.635038,
        field: "ph",
        epochtime: 1747378980000,
    },
    DataPoint {
        time: "2025-05-16T07:04:00Z",
        value: 7.63704,
        field: "ph",
        epochtime: 1747379040000,
    },
    DataPoint {
        time: "2025-05-16T07:05:00Z",
        value: 7.634039,
        field: "ph",
        epochtime: 1747379100000,
    },
    DataPoint {
        time: "2025-05-16T07:06:00Z",
        value: 7.635037,
        field: "ph",
        epochtime: 1747379160000,
    },
    DataPoint {
        time: "2025-05-16T07:07:00Z",
        value: 7.634034,
        field: "ph",
        epochtime: 1747379220000,
    },
    DataPoint {
        time: "2025-05-16T07:08:00Z",
        value: 7.633039,
        field: "ph",
        epochtime: 1747379280000,
    },
    DataPoint {
        time: "2025-05-16T07:09:00Z",
        value: 7.641044,
        field: "ph",
        epochtime: 1747379340000,
    },
    DataPoint {
        time: "2025-05-16T07:10:00Z",
        value: 7.642046,
        field: "ph",
        epochtime: 1747379400000,
    },
    DataPoint {
        time: "2025-05-16T07:11:00Z",
        value: 7.65404,
        field: "ph",
        epochtime: 1747379460000,
    },
    DataPoint {
        time: "2025-05-16T07:12:00Z",
        value: 7.651046,
        field: "ph",
        epochtime: 1747379520000,
    },
    DataPoint {
        time: "2025-05-16T07:13:00Z",
        value: 7.634032,
        field: "ph",
        epochtime: 1747379580000,
    },
    DataPoint {
        time: "2025-05-16T07:14:00Z",
        value: 7.645033,
        field: "ph",
        epochtime: 1747379640000,
    },
    DataPoint {
        time: "2025-05-16T07:15:00Z",
        value: 7.640043,
        field: "ph",
        epochtime: 1747379700000,
    },
    DataPoint {
        time: "2025-05-16T07:16:00Z",
        value: 7.638047,
        field: "ph",
        epochtime: 1747379760000,
    },
    DataPoint {
        time: "2025-05-16T07:17:00Z",
        value: 7.641045,
        field: "ph",
        epochtime: 1747379820000,
    },
    DataPoint {
        time: "2025-05-16T07:18:00Z",
        value: 7.644033,
        field: "ph",
        epochtime: 1747379880000,
    },
    DataPoint {
        time: "2025-05-16T07:19:00Z",
        value: 7.637039,
        field: "ph",
        epochtime: 1747379940000,
    },
    DataPoint {
        time: "2025-05-16T07:20:00Z",
        value: 7.608044,
        field: "ph",
        epochtime: 1747380000000,
    },
    DataPoint {
        time: "2025-05-16T07:21:00Z",
        value: 7.568047,
        field: "ph",
        epochtime: 1747380060000,
    },
    DataPoint {
        time: "2025-05-16T07:22:00Z",
        value: 7.607044,
        field: "ph",
        epochtime: 1747380120000,
    },
    DataPoint {
        time: "2025-05-16T07:23:00Z",
        value: 7.630035,
        field: "ph",
        epochtime: 1747380180000,
    },
    DataPoint {
        time: "2025-05-16T07:24:00Z",
        value: 7.624038,
        field: "ph",
        epochtime: 1747380240000,
    },
    DataPoint {
        time: "2025-05-16T07:25:00Z",
        value: 7.637045,
        field: "ph",
        epochtime: 1747380300000,
    },
    DataPoint {
        time: "2025-05-16T07:26:00Z",
        value: 7.639039,
        field: "ph",
        epochtime: 1747380360000,
    },
    DataPoint {
        time: "2025-05-16T07:27:00Z",
        value: 7.639041,
        field: "ph",
        epochtime: 1747380420000,
    },
    DataPoint {
        time: "2025-05-16T07:28:00Z",
        value: 7.63904,
        field: "ph",
        epochtime: 1747380480000,
    },
    DataPoint {
        time: "2025-05-16T07:29:00Z",
        value: 7.635036,
        field: "ph",
        epochtime: 1747380540000,
    },
    DataPoint {
        time: "2025-05-16T07:30:00Z",
        value: 7.641036,
        field: "ph",
        epochtime: 1747380600000,
    },
    DataPoint {
        time: "2025-05-16T07:31:00Z",
        value: 7.641041,
        field: "ph",
        epochtime: 1747380660000,
    },
    DataPoint {
        time: "2025-05-16T07:32:00Z",
        value: 7.640038,
        field: "ph",
        epochtime: 1747380720000,
    },
    DataPoint {
        time: "2025-05-16T07:33:00Z",
        value: 7.638047,
        field: "ph",
        epochtime: 1747380780000,
    },
    DataPoint {
        time: "2025-05-16T07:34:00Z",
        value: 7.634042,
        field: "ph",
        epochtime: 1747380840000,
    },
    DataPoint {
        time: "2025-05-16T07:35:00Z",
        value: 7.626033,
        field: "ph",
        epochtime: 1747380900000,
    },
    DataPoint {
        time: "2025-05-16T07:36:00Z",
        value: 7.632046,
        field: "ph",
        epochtime: 1747380960000,
    },
    DataPoint {
        time: "2025-05-16T07:37:00Z",
        value: 7.659037,
        field: "ph",
        epochtime: 1747381020000,
    },
    DataPoint {
        time: "2025-05-16T07:38:00Z",
        value: 7.646038,
        field: "ph",
        epochtime: 1747381080000,
    },
    DataPoint {
        time: "2025-05-16T07:39:00Z",
        value: 7.661044,
        field: "ph",
        epochtime: 1747381140000,
    },
    DataPoint {
        time: "2025-05-16T07:40:00Z",
        value: 7.653034,
        field: "ph",
        epochtime: 1747381200000,
    },
    DataPoint {
        time: "2025-05-16T07:41:00Z",
        value: 7.655046,
        field: "ph",
        epochtime: 1747381260000,
    },
    DataPoint {
        time: "2025-05-16T07:42:00Z",
        value: 7.655038,
        field: "ph",
        epochtime: 1747381320000,
    },
    DataPoint {
        time: "2025-05-16T07:43:00Z",
        value: 7.672032,
        field: "ph",
        epochtime: 1747381380000,
    },
    DataPoint {
        time: "2025-05-16T07:44:00Z",
        value: 7.661047,
        field: "ph",
        epochtime: 1747381440000,
    },
    DataPoint {
        time: "2025-05-16T07:45:00Z",
        value: 7.658038,
        field: "ph",
        epochtime: 1747381500000,
    },
    DataPoint {
        time: "2025-05-16T07:46:00Z",
        value: 7.644033,
        field: "ph",
        epochtime: 1747381560000,
    },
    DataPoint {
        time: "2025-05-16T07:47:00Z",
        value: 7.638041,
        field: "ph",
        epochtime: 1747381620000,
    },
    DataPoint {
        time: "2025-05-16T07:48:00Z",
        value: 7.63404,
        field: "ph",
        epochtime: 1747381680000,
    },
    DataPoint {
        time: "2025-05-16T07:49:00Z",
        value: 7.614043,
        field: "ph",
        epochtime: 1747381740000,
    },
    DataPoint {
        time: "2025-05-16T07:50:00Z",
        value: 7.625047,
        field: "ph",
        epochtime: 1747381800000,
    },
    DataPoint {
        time: "2025-05-16T07:51:00Z",
        value: 7.663046,
        field: "ph",
        epochtime: 1747381860000,
    },
    DataPoint {
        time: "2025-05-16T07:52:00Z",
        value: 7.615032,
        field: "ph",
        epochtime: 1747381920000,
    },
    DataPoint {
        time: "2025-05-16T07:53:00Z",
        value: 7.629035,
        field: "ph",
        epochtime: 1747381980000,
    },
    DataPoint {
        time: "2025-05-16T07:54:00Z",
        value: 7.636045,
        field: "ph",
        epochtime: 1747382040000,
    },
    DataPoint {
        time: "2025-05-16T07:55:00Z",
        value: 7.618033,
        field: "ph",
        epochtime: 1747382100000,
    },
    DataPoint {
        time: "2025-05-16T07:56:00Z",
        value: 7.61804,
        field: "ph",
        epochtime: 1747382160000,
    },
    DataPoint {
        time: "2025-05-16T07:57:00Z",
        value: 7.645035,
        field: "ph",
        epochtime: 1747382220000,
    },
    DataPoint {
        time: "2025-05-16T07:58:00Z",
        value: 7.615038,
        field: "ph",
        epochtime: 1747382280000,
    },
    DataPoint {
        time: "2025-05-16T07:59:00Z",
        value: 7.62604,
        field: "ph",
        epochtime: 1747382340000,
    },
    DataPoint {
        time: "2025-05-16T08:00:00Z",
        value: 7.618036,
        field: "ph",
        epochtime: 1747382400000,
    },
    DataPoint {
        time: "2025-05-16T08:01:00Z",
        value: 7.617044,
        field: "ph",
        epochtime: 1747382460000,
    },
    DataPoint {
        time: "2025-05-16T08:02:00Z",
        value: 7.642034,
        field: "ph",
        epochtime: 1747382520000,
    },
    DataPoint {
        time: "2025-05-16T08:03:00Z",
        value: 7.627042,
        field: "ph",
        epochtime: 1747382580000,
    },
    DataPoint {
        time: "2025-05-16T08:04:00Z",
        value: 7.625039,
        field: "ph",
        epochtime: 1747382640000,
    },
    DataPoint {
        time: "2025-05-16T08:05:00Z",
        value: 7.624041,
        field: "ph",
        epochtime: 1747382700000,
    },
    DataPoint {
        time: "2025-05-16T08:06:00Z",
        value: 7.624038,
        field: "ph",
        epochtime: 1747382760000,
    },
    DataPoint {
        time: "2025-05-16T08:07:00Z",
        value: 7.621038,
        field: "ph",
        epochtime: 1747382820000,
    },
    DataPoint {
        time: "2025-05-16T08:08:00Z",
        value: 7.62304,
        field: "ph",
        epochtime: 1747382880000,
    },
    DataPoint {
        time: "2025-05-16T08:09:00Z",
        value: 7.622036,
        field: "ph",
        epochtime: 1747382940000,
    },
    DataPoint {
        time: "2025-05-16T08:10:00Z",
        value: 7.623039,
        field: "ph",
        epochtime: 1747383000000,
    },
    DataPoint {
        time: "2025-05-16T08:11:00Z",
        value: 7.624045,
        field: "ph",
        epochtime: 1747383060000,
    },
    DataPoint {
        time: "2025-05-16T08:12:00Z",
        value: 7.62304,
        field: "ph",
        epochtime: 1747383120000,
    },
    DataPoint {
        time: "2025-05-16T08:13:00Z",
        value: 7.621047,
        field: "ph",
        epochtime: 1747383180000,
    },
    DataPoint {
        time: "2025-05-16T08:14:00Z",
        value: 7.623044,
        field: "ph",
        epochtime: 1747383240000,
    },
    DataPoint {
        time: "2025-05-16T08:15:00Z",
        value: 7.614041,
        field: "ph",
        epochtime: 1747383300000,
    },
    DataPoint {
        time: "2025-05-16T08:16:00Z",
        value: 7.610036,
        field: "ph",
        epochtime: 1747383360000,
    },
    DataPoint {
        time: "2025-05-16T08:17:00Z",
        value: 7.611034,
        field: "ph",
        epochtime: 1747383420000,
    },
    DataPoint {
        time: "2025-05-16T08:30:00Z",
        value: 7.621044,
        field: "ph",
        epochtime: 1747384200000,
    },
    DataPoint {
        time: "2025-05-16T08:31:00Z",
        value: 7.622033,
        field: "ph",
        epochtime: 1747384260000,
    },
    DataPoint {
        time: "2025-05-16T08:32:00Z",
        value: 7.625047,
        field: "ph",
        epochtime: 1747384320000,
    },
    DataPoint {
        time: "2025-05-16T08:33:00Z",
        value: 7.628033,
        field: "ph",
        epochtime: 1747384380000,
    },
    DataPoint {
        time: "2025-05-16T08:34:00Z",
        value: 7.616036,
        field: "ph",
        epochtime: 1747384440000,
    },
    DataPoint {
        time: "2025-05-16T08:35:00Z",
        value: 7.618032,
        field: "ph",
        epochtime: 1747384500000,
    },
    DataPoint {
        time: "2025-05-16T08:36:00Z",
        value: 7.625046,
        field: "ph",
        epochtime: 1747384560000,
    },
    DataPoint {
        time: "2025-05-16T08:37:00Z",
        value: 7.622046,
        field: "ph",
        epochtime: 1747384620000,
    },
    DataPoint {
        time: "2025-05-16T08:37:22.730790106Z",
        value: 7.630036,
        field: "ph",
        epochtime: 1747384642730,
    },
];
