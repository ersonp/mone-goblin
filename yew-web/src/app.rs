use chrono::{offset::Local, Datelike};
use std::rc::Rc;
use yew::{function_component, html, use_effect_with_deps, use_reducer, Callback, Html};

use crate::{
    components::{inv_form::InvestmentForm, inv_list::InvestmentList},
    controllers::*,
    state::InvestmentState,
};

#[function_component(App)]
pub fn app() -> Html {
    let investments = use_reducer(InvestmentState::default);
    let investment_controller = Rc::new(InvestmentController::new(investments.clone()));

    // Get all investments on app startup
    {
        let investment_controller = investment_controller.clone();

        use_effect_with_deps(
            move |_| {
                investment_controller.init_investments();
                || {} // return empty destructor closure (cleanup use_effect)
            },
            (),
        ); // only call on first render
    }

    let on_create_investment = {
        let investment_controller = investment_controller.clone();

        Callback::from(move |title: String| investment_controller.create_investment(title))
    };

    let on_delete_investment = {
        let investment_controller = investment_controller.clone();

        Callback::from(move |id: String| investment_controller.delete_investment(id))
    };

    let on_toggle_investment = {
        let investment_controller = investment_controller.clone();

        Callback::from(move |id: String| investment_controller.toggle_investment(id))
    };

    html! {
        <div class="flex flex-col mt-14 w-[400px] mx-auto gap-6">
            <header class="flex flex-col mx-auto w-full">
                <h1 class="text-3xl text-center font-black mb-5">{"Whattodo!"}</h1>

                <img class="h-32 mb-8 hover:scale-110 ease-in-out duration-500" src="img/investment-done-flat.svg" alt="Todo App logo"/>

                <InvestmentForm createinvestment={on_create_investment} />
            </header>


            <main class="mx-auto my-4 w-full">
                <h1 class="text-3xl font-black">{"Investments"}</h1>

                <hr class="mb-6 border-t-2" />

                <InvestmentList investments={investments.investments.clone()} deleteinvestment={on_delete_investment} toggleinvestment={on_toggle_investment} />
            </main>

            <footer class="mt-3 mb-6">
                <a
                  class="italic tracking-wider hover:text-sky-500 ease-in duration-300"
                  href="https://github.com/ersonp/mone-goblin"
                  target="_blank"
                >
                  {format!("MIT Licensed | Copyright © {} Enrique Marín", Local::now().year())}
                </a>
            </footer>
        </div>
    }
}

/*
 * ARRANCAR UN CONTENEDOR DOCKER DE SURREALDB CON UN FICHERO docker-compose.yml:
 * sudo docker compose up -d
 *
 * ENTRAR EN LA CLI DE SURREALDB EN EL CONTENEDOR CREADO:
 * sudo docker exec -it surrealdb /surreal sql -c http://localhost:8000 -u root -p root --ns namespace --db database --pretty
 *
 * VER SI EL CONTENEDOR DOCKER ESTÁ INICIADO:
 * sudo docker ps  (CON EL FLAG --a SE LISTAN TODOS LOS CONTENEDORES, ACTIVOS Y NO ACTIVOS)
 *
 * DETENER EL CONTENEDOR DE DOCKER:
 * sudo docker stop surrealdb
 *
 * VOLVER A INICIAR EL CONTENEDOR DE DOCKER:
 * sudo docker start surrealdb
 */

/*
 * https://doc.rust-lang.org/std/collections/vec_deque/struct.VecDeque.html
 * https://freeiconshop.com/
 * https://tailwindcss.com/docs/accent-color
 * https://docs.rs/yew/0.20.0/yew/functional/fn.use_reducer.html
 * https://flowbite.com/docs/forms/checkbox/#bordered
 */
