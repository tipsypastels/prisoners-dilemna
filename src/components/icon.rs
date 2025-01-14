use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct IconProps {
    pub name: AttrValue,
}

#[function_component]
pub fn Icon(props: &IconProps) -> Html {
    let class = format!("fas fa-{}", props.name);
    html! { <i class={class} /> }
}
