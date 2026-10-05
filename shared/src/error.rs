use std::error::Error;

minimer::define_app_error!(pub AppError);

minimer::impl_from_error!(AppError, serde_json::Error);

pub fn render_error_chain(error: &dyn Error) -> String {
    let mut rendered: String = error.to_string();
    let mut next: Option<&dyn Error> = error.source();

    while let Some(source) = next {
        rendered.push_str(" -> ");
        rendered.push_str(&source.to_string());
        next = source.source();
    }

    rendered
}
