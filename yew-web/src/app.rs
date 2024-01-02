use crate::components::switcher::DarkModeContent; // Add this line to import the switcher module

use std::rc::Rc;
use types::Investment2;
use yew::{function_component, html, use_effect_with_deps, use_reducer, Callback, Html};

use crate::{components::inv_list::InvestmentList, controllers::*, state::InvestmentState};

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

        Callback::from(move |inv: Investment2| investment_controller.create_investment(inv))
    };

    let on_delete_investment = {
        let investment_controller = investment_controller.clone();

        Callback::from(move |id: String| investment_controller.delete_investment(id))
    };

    let on_edit_investment = {
        let investment_controller = investment_controller;

        Callback::from(move |id: String| investment_controller.edit_investment(id))
    };

    html! {
        <div class="flex flex-col mt-14 mx-auto gap-6">
            <header class="flex flex-col mx-auto w-full text-text-50">
            </header>
            <main>
                <div class="relative mx-auto my-4 w-full">
                    <div class="absolute top-0 right-1/8 w-[30vw] h-[30vw] bg-accent-600 mix-blend-hard-light rounded-full filter blur-xl opacity-70 animate-blob animation-delay-4000 pointer-events-none"></div>
                    <div class="absolute top-0 left-1/4 w-[30vw] h-[30vw] bg-accent-600 mix-blend-hard-light rounded-full filter blur-xl opacity-70 animate-blob animation-delay-2000 pointer-events-none"></div>
                    <div class="absolute top-0 left-1/4 w-[40vw] h-[40vw] bg-secondary-600 mix-blend-hard-light rounded-full filter blur-xl opacity-70 animate-blob pointer-events-none"></div>
                    <div class="absolute top-3 left-1/2 w-[30vw] h-[30vw] bg-primary-600 mix-blend-hard-light rounded-full filter blur-xl opacity-70 animate-blob animation-delay-3000 pointer-events-none"></div>
                    <div class="absolute top-3 right-0 w-[30vw] h-[30vw] bg-accent-600 mix-blend-hard-light rounded-full filter blur-xl opacity-70 animate-blob2 pointer-events-none"></div>
                    <div class="flex">
                        <h1 class="text-3xl font-black text-text-950">{"Investments"}</h1>
                        <div class="ml-auto flex items-center">
                            <DarkModeContent />
                        </div>
                    </div>
                    <hr class="mb-6 border-t-2" />
                    <div>
                        <InvestmentList investments={investments.investments.clone()} create_investment={on_create_investment} delete_investment={on_delete_investment} toggle_investment={on_edit_investment} />
                    </div>
                </div>
            </main>
            <footer class="mt-3 mb-6">
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
