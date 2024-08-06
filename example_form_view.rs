#[component]
pub fn VisualizarEvolucao() -> impl IntoView {
    let form_data_id = queries::params::form_data_id();

    let form_data: Resource<Result<FormData, AppError>> = queries::form_data::get_form_data(form_data_id().unwrap().into());

    view! {
        <SuspenseSkeleton>
            {move || {
                let validate = SignalResult::from_result(form_data_id.get())
                    .combine(SignalResult::from_option_result(form_data.get()));
                match validate {
                    SignalResult::Ok(hlist_pat!(form_data_id, form_data)) => {
                        EitherOf3::A(
                            view! {
                                <h1 class="text-2xl font-bold">Formulário</h1>
                                <FormEvolucao form_data />
                            },
                        )
                    }
                    SignalResult::Err(errors) => EitherOf3::B(view! { <ErrorReporter errors /> }),
                    SignalResult::Loading => EitherOf3::C(view! { <Skeleton /> }),
                }
            }}
        </SuspenseSkeleton>
    }
}

#[component]
fn FormEvolucao(
    #[prop(into)] form_data: MaybeSignal<FormData>,
    #[prop(default = true)] auto_submit: bool,
    #[prop(default = false)] new_form: bool,
) -> impl IntoView {
    let save_form = ServerAction::<UpsertFormDataSrv>::new();

    use crate::components::form::{
        ActionFormAutoSubmit, GrowableTextArea, HiddenField, InputField,
    };

    view! {
        <SuspenseSkeleton>
            <div class="p-2 w-full">
                <ActionFormAutoSubmit
                    action=save_form
                    class="gap-2 p-2 rounded shadow-md"
                    auto_submit
                >
                    <Transition fallback=move || view! { <Skeleton /> }>
                        <InputField
                            r#type="datetime-local"
                            label="Created At"
                            name="date_created"
                            value=BrowserDateTime::from(form_data().date_created).to_string()
                            class="w-72"
                        />

                        <GrowableTextArea
                            name="my_text"
                            label="Text"
                            value=form_data().my_text.unwrap_or_default()
                        />

                        <div class="flex justify-center items-center m-2">
                            <button class="btn btn-primary btn-active">Enviar</button>
                        </div>
                    </Transition>
                </ActionFormAutoSubmit>
            </div>
        </SuspenseSkeleton>
    }
}