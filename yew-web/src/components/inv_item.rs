use chrono::Timelike;
use types::Investment2;
use yew::{function_component, html, Callback, Event, Html, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct InvestmentItemProps {
    pub investment: Investment2,
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
    let start_date = if let Some(date) = investment.start_date {
        date.date_naive().format("%d-%m-%Y").to_string()
    } else {
        String::new()
    };

    let end_date = if let Some(date) = investment.end_date {
        date.date_naive().format("%d-%m-%Y").to_string()
    } else {
        String::new()
    };
    // let date = investment
    //     .created_at
    //     .unwrap_or_default()
    //     .date_naive()
    //     .format("%d-%m-%Y");

    // let time_and_date = &format!(
    //     "{:02}:{:02} • {}",
    //     investment.created_at.unwrap_or_default().hour(),
    //     investment.created_at.unwrap_or_default().minute(),
    //     date
    // );

    let label_style = "w-full p-3 ml-2 text-lg truncate";

    // let completed_investment = match investment.completed {
    //     true => Some("text-gray-600 line-through"),
    //     false => None,
    // };

    let handle_click = {
        let investment = investment.clone();
        let on_delete_investment = deleteinvestment.clone();

        // (todo) fix this
        // let id = match investment.id.clone() {
        //     Some(id) => id.tb,
        //     None => "".to_string(),
        // };
        // move |_e: MouseEvent| on_delete_investment.emit(id.clone())
        move |_e: MouseEvent| on_delete_investment.emit(investment.id.clone())
    };

    let handle_toggle = {
        let investment = investment.clone();
        let on_toggle_investment = toggleinvestment.clone();

        // let id = match investment.id.clone() {
        //     Some(id) => id.tb,
        //     None => "".to_string(),
        // };
        // move |_e: Event| on_toggle_investment.emit(id.clone())
        move |_e: Event| on_toggle_investment.emit(investment.id.clone())
    };
    //  horrible way to "fix" this
    // let id = match investment.id.clone() {
    //     Some(id) => id.tb,
    //     None => "".to_string(),
    // };
    // let id2 = id.clone();
    html! {
            <tbody>
                <tr class="border-b dark:border-gray-700">
                    <td class="px-6 py-4">{start_date}</td>
                    <td class="px-6 py-4">{end_date}</td>
                    <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">{investment.clone().inv_name}</th>
                    <td class="px-6 py-4">{investment.clone().name}</td>
                    <td class="px-6 py-4">{investment.clone().inv_type}</td>
                    <td class="px-6 py-4">{investment.clone().return_rate}</td>
                    <td class="px-6 py-4">{investment.clone().return_rate_type}</td>
                    <td class="px-6 py-4">{investment.clone().inv_amount}</td>
                    <td class="px-6 py-4">{investment.clone().return_amount}</td>
                    <td class="flex items-center px-6 py-4">
                        <a href="#" class="font-medium text-blue-600 dark:text-blue-500 hover:underline">{"Edit"}</a>
                        <a onclick={handle_click} class="font-medium text-red-600 dark:text-red-500 hover:underline ms-3">{"Remove"}</a>
                    </td>
                </tr>

            </tbody>

    }
}
