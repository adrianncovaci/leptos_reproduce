use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;


#[derive(Clone)]
struct TimePlotData {
    chart_panels: RwSignal<Vec<ChartPanel>>,
}

impl TimePlotData {
    pub fn add_chart(&mut self) {
        self.chart_panels.set(vec![ChartPanel::default()]);
    }

    pub fn has_topic(&self, data_info: &'static str) -> bool {
        self.chart_panels.with_untracked(|chart_panels| {
            chart_panels.iter().any(|chart_panel| chart_panel.has_topic(data_info))
        })
    }

    pub fn add_topic(&mut self, data_info: &'static str) {
        if self.chart_panels.with(Vec::is_empty) {
            self.add_chart();
        }

        self.chart_panels.update(|chart| {
            if let Some(chart_panel) = chart.last_mut() {
                chart_panel.add_topic(data_info);
            }
        });
    }

    pub fn toggle_topic(&mut self, data_info: &'static str) {
        if self.has_topic(data_info) {
            self.remove_topic(data_info);
        } else {
            self.add_topic(data_info);
        }
    }

    pub fn remove_topic(&mut self, data_info: &'static str) {
        self.chart_panels
            .update(|chart_panels| chart_panels.iter_mut().for_each(|chart_panel| chart_panel.remove_topic(data_info)));
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct ChartPanel {
    topic_buffers: RwSignal<Vec<TopicBuffer>>,
}

impl ChartPanel {
    pub fn is_empty(&self) -> bool {
        self.topic_buffers.with(Vec::is_empty)
    }

    pub fn has_topic(&self, name: &'static str) -> bool {
        self.topic_buffers.with(|topic_buffers| topic_buffers.iter().any(|topic_buffer| topic_buffer.identifier == name))
    }

    pub fn add_topic(&mut self, data_info: &'static str) {
        let new_topic_buffer = TopicBuffer { identifier: data_info };

        self.topic_buffers.update(|topic_buffers| topic_buffers.push(new_topic_buffer));
    }

    pub fn remove_topic(&mut self, data_info: &'static str) {
        self.topic_buffers.update(|topic_buffers| {
            topic_buffers
                .iter()
                .filter(|topic_buffer| topic_buffer.identifier == data_info)
                .for_each(|_topic_buffer| {});

            topic_buffers.retain(|topic_buffer| topic_buffer.identifier != data_info);
        });
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
