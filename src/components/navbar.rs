use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let show_menu = use_state(|| false);
    
    let toggle_menu = {
        let show_menu = show_menu.clone();
        Callback::from(move |_| {
            show_menu.set(!*show_menu);
        })
    };

    html! {
        <nav class="bg-gray-800 p-4 relative">
            <div class="container mx-auto">
                <div class="flex items-center justify-between">
                    // Left side - Name and title
                    <div class="flex-1">
                        <p class="text-white font-bold text-lg md:text-xl lg:text-2xl">
                            {"Oleg Tchernov / Fullstack Java Developer"}
                        </p>
                    </div>

                    // Desktop menu (visible on large screens)
                    <div class="hidden lg:flex lg:items-center lg:space-x-6">
                        <Link<Route> 
                            classes="text-white hover:text-gray-300 text-lg" 
                            to={Route::Home}
                        >
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> 
                            classes="text-white hover:text-gray-300 text-lg" 
                            to={Route::About}
                        >
                            { "About Me" }
                        </Link<Route>>
                        <Link<Route> 
                            classes="text-white hover:text-gray-300 text-lg" 
                            to={Route::Resume}
                        >
                            { "Resume" }
                        </Link<Route>>
                        <Link<Route> 
                            classes="text-white hover:text-gray-300 text-lg" 
                            to={Route::Contact}
                        >
                            { "Contact" }
                        </Link<Route>>
                    </div>

                    // Hamburger button (visible on smaller screens)
                    <div class="lg:hidden flex items-center">
                        <button 
                            class="text-white hover:text-gray-300 p-2 rounded-md"
                            onclick={toggle_menu}
                        >
                            <div class="space-y-2">
                                <div class="w-8 h-0.5 bg-white"></div>
                                <div class="w-8 h-0.5 bg-white"></div>
                                <div class="w-8 h-0.5 bg-white"></div>
                            </div>
                        </button>
                    </div>
                </div>

                // Mobile dropdown menu
                <div class={classes!(
                    "lg:hidden",
                    "absolute",
                    "right-0",
                    "mt-2",
                    "py-4",
                    "w-64",
                    "bg-gray-800",
                    "rounded-md",
                    "shadow-xl",
                    if *show_menu { "block" } else { "hidden" },
                    "z-50"
                )}>
                    <Link<Route> 
                        classes="block px-6 py-3 text-white hover:bg-gray-700 w-full text-left text-lg" 
                        to={Route::Home}
                    >
                        { "Home" }
                    </Link<Route>>
                    <Link<Route> 
                        classes="block px-6 py-3 text-white hover:bg-gray-700 w-full text-left text-lg" 
                        to={Route::About}
                    >
                        { "About Me" }
                    </Link<Route>>
                    <Link<Route> 
                        classes="block px-6 py-3 text-white hover:bg-gray-700 w-full text-left text-lg" 
                        to={Route::Resume}
                    >
                        { "Resume" }
                    </Link<Route>>
                    <Link<Route> 
                        classes="block px-6 py-3 text-white hover:bg-gray-700 w-full text-left text-lg" 
                        to={Route::Contact}
                    >
                        { "Contact" }
                    </Link<Route>>
                </div>
            </div>
        </nav>
    }
}