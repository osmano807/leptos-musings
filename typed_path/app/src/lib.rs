use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path, MatchNestedRoutes};

pub mod routes;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="pt">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    tracing::info!("Initializing App");

    tracing::debug!("Initializing meta context");
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Routes are not typed...
    // https://github.com/leptos-rs/leptos/issues/2175

    view! {
        <Stylesheet id="leptos" href="/pkg/prontuario-eletronico.css" />

        // sets the document title
        <Title text="Hermes" />

        // There should be a way to make this Router somewhat typed
        // and have a method of creating links directly to routes.
        // See Dioxus impl using a enum and some prop macro voodoo
        // For now, try to sync with our custom `TypedPath`
        <Router>
            <Navbar />
            <Routes fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::PageNotFound);
                view! { <ErrorTemplate outside_errors /> }
            }>

            <Route path=path!("") view=HomePage />
            <Route path=path!("/help") view=HelpPage />
            <Route path=path!("/some/:parameter") view=SomeParameterPathPage />

            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! { 
        <p>Home</p> 
        <A href=routes::Home>Home</A>
        <A href=routes::Help>Help</A>
        <A href=routes::SomeParameterPath { parameter: "test".to_string() }>SomeParameterPath</A>
    }
}

#[component]
fn HelpPage() -> impl IntoView {
    view! { <p>Help</p> }
}

#[component]
fn SomeParameterPathPage() -> impl IntoView {
    view! { <p>SomeParameterPath</p> }
}
