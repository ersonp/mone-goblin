use types::Investment2;
use yew::{function_component, html, Callback, Event, Html, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct InvestmentItemProps {
    pub investment: Investment2,
    pub delete_investment: Callback<String>,
    pub toggle_investment: Callback<String>,
}

#[function_component(InvestmentItem)]
pub fn investment_item(
    InvestmentItemProps {
        investment,
        delete_investment,
        toggle_investment,
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
        let on_delete_investment = delete_investment.clone();

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
        let on_toggle_investment = toggle_investment.clone();

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
                <tr class="border-b dark:border-background-200 hover:bg-background-50">
                    <td class="px-6 py-4 min-w-max whitespace-nowrap hidden sm:table-cell">
                        {start_date.clone()}
                        <dl class="lg:hidden">
                            <dt class="sr-only">{"End Date"}</dt>
                            <dd class="mt-1">{end_date.clone()}</dd>
                        </dl>
                    </td>
                    <td class="px-6 py-4 min-w-max whitespace-nowrap hidden lg:table-cell">{end_date.clone()}</td>
                    <th class="px-6 py-4 min-w-max font-medium text-text-950 ">
                        {investment.clone().inv_name}
                        <dl class="font-normal text-text-500">
                            <dt class="lg:hidden sr-only">{"Name"}</dt>
                            <dd class="lg:hidden mt-1">{investment.clone().name}</dd>
                            <dt class="sm:hidden sr-only">{"Start Date"}</dt>
                            <dd class="sm:hidden mt-1">{start_date.clone()}</dd>
                            <dt class="sm:hidden sr-only">{"End Date"}</dt>
                            <dd class="sm:hidden mt-1">{end_date.clone()}</dd>
                        </dl>
                    </th>
                    <td class="px-6 py-4 min-w-max hidden lg:table-cell">{investment.clone().name}</td>
                    <td class="px-6 py-4 min-w-max hidden sm:table-cell">
                        {investment.clone().inv_type}
                        <dl class="lg:hidden font-normal text-text-500">
                            <dt class="sr-only">{"Return Type"}</dt>
                            <dd class="mt-1">{investment.clone().return_type}</dd>
                            <dt class="sr-only">{"Return Rate"}</dt>
                            <dd class="mt-1">{investment.clone().return_rate}</dd>
                        </dl>
                    </td>
                    <td class="px-6 py-4 min-w-max hidden lg:table-cell">{investment.clone().return_type}</td>
                    <td class="px-6 py-4 min-w-max hidden lg:table-cell">{investment.clone().return_rate}</td>
                    <td class="px-6 py-4 min-w-max hidden lg:table-cell">{investment.clone().inv_amount} </td>
                    <td class="px-6 py-4 min-w-max font-medium text-text-950">
                        {investment.clone().return_amount}
                        <dl class="lg:hidden font-normal text-text-500">
                            <dt class="sr-only">{"Investment"}</dt>
                            <dd class="mt-1">{investment.clone().inv_amount}</dd>
                            <dt class="sr-only sm:hidden">{"Investment Type"}</dt>
                            <dd class="mt-1 sm:hidden">{investment.clone().inv_type}</dd>
                        </dl>
                    </td>
                    <td class="flex flex-col items-start px-6 py-4">
                        <a href="#" class="font-medium text-secondary-600 hover:underline">{"Renew"}</a>
                        <a href="#" class="font-medium text-accent-600 hover:underline">{"Edit"}</a>
                        <a onclick={handle_click} class="font-medium text-red-600 hover:underline">{"Remove"}</a>
                    </td>
                </tr>

            </tbody>

    }
}
