use crate::components::{layout::Layout, strategies::StrategiesList};
use yew::prelude::*;

#[function_component]
pub fn StrategiesPage() -> Html {
    let selection1 = use_state(|| None::<usize>);
    let selection2 = use_state(|| None::<usize>);

    html! {
        <Layout>
            <StrategiesList
                selection1={selection1}
                selection2={selection2}
            />
        </Layout>
    }
}
