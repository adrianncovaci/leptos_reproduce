use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;


#[derive(Clone)]
struct TimePlotData {
    chart_panels: RwSignal<Vec<ChartPanel>>,
}

impl TimePlotData {
    pub fn has_topic(&self, data_info: &'static str) -> bool {
        self.chart_panels.with_untracked(|chart_panels| {
            chart_panels.iter().any(|chart_panel| chart_panel.has_topic(data_info))
        })
    }

    pub fn toggle_topic(&mut self, data_info: &'static str) {
        self.chart_panels.update(|chart| {
            if let Some(chart_panel) = chart.last_mut() {
                let new_topic_buffer = TopicBuffer { identifier: data_info };
                chart_panel.topic_buffers.update(|topic_buffers| topic_buffers.push(new_topic_buffer));

            }
        });
    }
}

#[derive(Default, Copy, Clone, Debug)]
struct ChartPanel {
    topic_buffers: RwSignal<Vec<TopicBuffer>>,
}

impl ChartPanel {
    pub fn has_topic(&self, name: &'static str) -> bool {
        self.topic_buffers.with(|topic_buffers| topic_buffers.iter().any(|topic_buffer| topic_buffer.identifier == name))
    }
}

#[derive(Clone)]
pub struct TopicBuffer {
    identifier: &'static str,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let tp = TimePlotData {
        chart_panels: RwSignal::new(vec![ChartPanel::default()]),
    };
    provide_context::<StoredValue<TimePlotData>>(StoredValue::new(tp));

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let tp = use_context::<StoredValue<TimePlotData>>().expect("Failed to get TimePlotData");
    let get_available_entries = move || vec![
        "data_1",
        "data_2",
        "data_3",
        "data_4",
        "data_5",
        "data_6",
        "data_7",
    ];

    view! {
        {get_available_entries()
            .into_iter()
            .map(|value| {
                let data_value = StoredValue::new(value);
                view! {
                    <div on:click=move |_| {
                        data_value.with_value(move |dv| tp.get_value().toggle_topic(dv))
                    }>
                        <Show
                            when=move || {
                                data_value.with_value(move |dv| tp.get_value().has_topic(dv))
                            }
                            fallback=|| ()
                        >
                            <button
                                class="btn btn-xs btn-primary me-1"
                                on:click=move |_| {
                                    data_value.with_value(move |dv| tp.get_value().toggle_topic(dv))
                                }
                            >
                                +
                            </button>
                        </Show>
                        <span inner_html=move || data_value.with_value(|dv| dv.to_string())></span>
                    </div>
                }
            })
            .collect_view()}
    }
}
