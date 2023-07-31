use chrono::Timelike;
use types::Investment;
use yew::{classes, function_component, html, Callback, Event, Html, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct InvestmentItemProps {
    pub investment: Investment,
    pub deleteinvestment: Callback<String>,
    pub toggleinvestment: Callback<String>,
}

#[function_component(InvestmentItem)]
pub fn investment_item(
    InvestmentItemProps {
        investment,
        deleteinvestment,
        toggleinvestment,
    }: &InvestmentItemProps,
) -> Html {
    let date = investment.created_at.date_naive().format("%d-%m-%Y");

    let time_and_date = &format!(
        "{:02}:{:02} • {}",
        investment.created_at.hour(),
        investment.created_at.minute(),
        date
    );

    let label_style = "w-full p-3 ml-2 text-lg truncate";

    // let completed_investment = match investment.completed {
    //     true => Some("text-gray-600 line-through"),
    //     false => None,
    // };

    let handle_click = {
        let investment = investment.clone();
        let on_delete_investment = deleteinvestment.clone();

        // (todo) fix this
        let id = match investment.id.clone() {
            Some(id) => id.tb,
            None => "".to_string(),
        };
        move |_e: MouseEvent| on_delete_investment.emit(id.clone())
    };

    let handle_toggle = {
        let investment = investment.clone();
        let on_toggle_investment = toggleinvestment.clone();

        let id = match investment.id.clone() {
            Some(id) => id.tb,
            None => "".to_string(),
        };
        move |_e: Event| on_toggle_investment.emit(id.clone())
    };
    //  horrible way to "fix" this
    let id = match investment.id.clone() {
        Some(id) => id.tb,
        None => "".to_string(),
    };
    let id2 = id.clone();
    html! {
        <li>
            <div class="flex flex-col my-2 pl-4 py-1 border rounded border-gray-700 hover:-translate-y-1.5 ease-in duration-300">
                <div class="flex items-center">

                    <input id={id}
                        type="checkbox"
                        class="w-5 h-5 accent-purple-600"
                        onchange={handle_toggle} />

                    <label for={id2}
                    title={investment.clone().inv_name}
                    class={classes!(label_style)}>
                        {&investment.inv_name}
                    </label>
                </div>

                <div class="flex justify-between items-center px-2 pb-2">
                    <p class="text-sm text-gray-600 font-bold">{time_and_date}</p>
                    <button title={"Remove Todo"} onclick={handle_click} class="bg-red-600 hover:bg-red-500 px-2.5 py-1.5 rounded">
                        <svg class="w-5" fill="currentColor" viewBox="0 0 16 16">
                          <path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 \
                          1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1H2.5zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 \
                          0 1-1 0v-7a.5.5 0 0 1 .5-.5zM8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 \
                          8 5zm3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0z"/>
                        </svg>
                    </button>
                </div>
            </div>
        </li>
    }
}
