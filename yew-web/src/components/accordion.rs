use yew::prelude::*;

pub struct Accordion {
    open: bool,
}

pub enum Form {
    Toggle,
}

impl Component for Accordion {
    type Message = Form;
    type Properties = ();

    fn create(_: &yew::Context<Self>) -> Self {
        Self { open: false }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Form::Toggle => {
                self.open = !self.open;
                true
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div class="w-full mx-auto">
                <div class="rounded">
                    <div class={if self.open { "bg-gray-50 dark:bg-gray-700 rounded-t" } else { "" }}>
                        <button  class="flex items-center justify-between w-full p-3 font-medium rtl:text-left rounded-t hover:bg-gray-50 hover:dark:bg-gray-700" onclick={ctx.link().callback(|_| Form::Toggle)}>
                        <span class="flex items-center text-gray-500 dark:text-grey-400"> {"Total: 5,00,000"}</span>
                        <svg class="w-7" fill="currentColor" viewBox="0 0 24 24">
                        <path d="M2 18H12V20H2V18ZM2 11H22V13H2V11ZM2 4H22V6H2V4ZM18 \
                        18V15H20V18H23V20H20V23H18V20H15V18H18Z" />
                        </svg>
                        </button>
                    </div>
                    <div class={if self.open { "block" } else { "hidden" }}>
                        <p class="w-full p-4 text-gray-500 text-base bg-gray-50 dark:bg-gray-700 rounded-b">
                        <div class="w-full md:w-auto flex flex-col md:flex-row space-y-2 md:space-y-0 items-stretch md:items-center justify-end md:space-x-3 flex-shrink-0">
                            <button type="button" class="flex items-center justify-center text-white bg-primary-700 hover:bg-primary-800 focus:ring-4 focus:ring-primary-300 font-medium rounded-lg text-sm px-4 py-2 dark:bg-primary-600 dark:hover:bg-primary-700 focus:outline-none dark:focus:ring-primary-800">
                                {"Add product"}
                            </button>
                        </div>
                        </p>
                    </div>
                </div>
            </div>
        }
    }
}

// <div id="accordion-open" data-accordion="open">
// <h2 id="accordion-open-heading-1">
// <button type="button" class="flex items-center justify-between w-full p-5 font-medium rtl:text-right text-gray-500 border border-b-0 border-gray-200 rounded-t-xl focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-800 dark:border-gray-700 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 gap-3" data-accordion-target="#accordion-open-body-1" aria-expanded="true" aria-controls="accordion-open-body-1">
//     <span class="flex items-center"><svg class="w-5 h-5 me-2 shrink-0" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"></path></svg> {"What is Flowbite?"}</span>
//     <svg class="w-3 h-3 rotate-180 shrink-0" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
//     <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5 5 1 1 5"/>
//     </svg>
// </button>
// </h2>
// <div id="accordion-open-body-1" class="hidden" aria-labelledby="accordion-open-heading-1">
// <div class="p-5 border border-b-0 border-gray-200 dark:border-gray-700 dark:bg-gray-900">
//     <p class="mb-2 text-gray-500 dark:text-gray-400">{"Flowbite is an open-source library of interactive components built on top of Tailwind CSS including buttons, dropdowns, modals, navbars, and more."}</p>
// </div>
// </div>
// <h2 id="accordion-open-heading-2">
// <button type="button" class="flex items-center justify-between w-full p-5 font-medium rtl:text-right text-gray-500 border border-b-0 border-gray-200 focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-800 dark:border-gray-700 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 gap-3" data-accordion-target="#accordion-open-body-2" aria-expanded="false" aria-controls="accordion-open-body-2">
//     <span class="flex items-center"><svg class="w-5 h-5 me-2 shrink-0" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"></path></svg>{"Is there a Figma file available?"}</span>
//     <svg class="w-3 h-3 rotate-180 shrink-0" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
//     <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5 5 1 1 5"/>
//     </svg>
// </button>
// </h2>
// <div id="accordion-open-body-2" class="hidden" aria-labelledby="accordion-open-heading-2">
// <div class="p-5 border border-b-0 border-gray-200 dark:border-gray-700">
//     <p class="mb-2 text-gray-500 dark:text-gray-400">{"Flowbite is first conceptualized and designed using the Figma software so everything you see in the library has a design equivalent in our Figma file."}</p>
//     <p class="text-gray-500 dark:text-gray-400">{"Check out the"} <a href="https://flowbite.com/figma/" class="text-blue-600 dark:text-blue-500 hover:underline">{"Figma design system"}</a> {"based on the utility classes from Tailwind CSS and components from Flowbite."}</p>
// </div>
// </div>
// <h2 id="accordion-open-heading-3">
// <button type="button" class="flex items-center justify-between w-full p-5 font-medium rtl:text-right text-gray-500 border border-gray-200 focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-800 dark:border-gray-700 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 gap-3" data-accordion-target="#accordion-open-body-3" aria-expanded="false" aria-controls="accordion-open-body-3">
//     <span class="flex items-center"><svg class="w-5 h-5 me-2 shrink-0" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"></path></svg> {"What are the differences between Flowbite and Tailwind UI?"}</span>
//     <svg class="w-3 h-3 rotate-180 shrink-0" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
//     <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5 5 1 1 5"/>
//     </svg>
// </button>
// </h2>
// <div id="accordion-open-body-3" class="hidden" aria-labelledby="accordion-open-heading-3">
// <div class="p-5 border border-t-0 border-gray-200 dark:border-gray-700">
//     <p class="mb-2 text-gray-500 dark:text-gray-400">{"Learn more about these technologies:"}</p>
//     <ul class="ps-5 text-gray-500 list-disc dark:text-gray-400">
//     <li><a href="https://flowbite.com/pro/" class="text-blue-600 dark:text-blue-500 hover:underline">{"Flowbite Pro"}</a></li>
//     <li><a href="https://tailwindui.com/" rel="nofollow" class="text-blue-600 dark:text-blue-500 hover:underline">{"Tailwind UI"}</a></li>
//     </ul>
// </div>
// </div>
// </div>

// <div id="accordion-open" data-accordion="open">
// <h2 id="accordion-open-heading-1">
//     <button type="button" class="flex items-center justify-between w-full p-5 font-medium rtl:text-right text-gray-500 border border-b-0 border-gray-200 rounded-t-xl focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-800 dark:border-gray-700 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 gap-3" data-accordion-target="#accordion-open-body-1" aria-expanded="true" aria-controls="accordion-open-body-1">
//         <span class="flex items-center">
//             <svg class="w-5 h-5 me-2 shrink-0" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
//                 <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"></path>
//             </svg>
//             {"What is Flowbite?"}
//         </span>
//         <svg class="w-3 h-3 rotate-180 shrink-0" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
//             <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5 5 1 1 5"/>
//         </svg>
//     </button>
// </h2>
// <div id="accordion-open-body-1" class="hidden" aria-labelledby="accordion-open-heading-1">
//     <div class="p-5 border border-b-0 border-gray-200 dark:border-gray-700 dark:bg-gray-900">
//         <p class="mb-2 text-gray-500 dark:text-gray-400">{"Flowbite is an open-source library of interactive components built on top of Tailwind CSS including buttons, dropdowns, modals, navbars, and more."}</p>
//     </div>
// </div>
// <h2 id="accordion-open-heading-2">
//     <button type="button" class="flex items-center justify-between w-full p-5 font-medium rtl:text-right text-gray-500 border border-b-0 border-gray-200 focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-800 dark:border-gray-700 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 gap-3" data-accordion-target="#accordion-open-body-2" aria-expanded="false" aria-controls="accordion-open-body-2">
//         <span class="flex items-center">

// </div>
