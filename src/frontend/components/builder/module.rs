use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModuleProps {
    pub module_type: String,
}

#[function_component(Module)]
pub fn module(props: &ModuleProps) -> Html {
    html! {
        <div class={format!("module {}", props.module_type)}>
            { format!("This is a {}", props.module_type) }
        </div>
    }
}
