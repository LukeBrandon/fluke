use yew::prelude::*;

pub struct Navbar;

impl Component for Navbar {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <nav class="flex justify-around py-4 bg-white/80
            backdrop-blur-md shadow-md w-full
            fixed top-0 left-0 right-0 z-10">
            <div class="flex items-center">
                <a class="cursor-pointer">
                    <h3 class="text-2xl font-medium text-blue-500">
                        <img class="h-10 object-cover"
                            src="logo_transparent.png" alt="Logo"/>
                    </h3>
                </a>
            </div>
            <div class="items-center hidden space-x-8 lg:flex">
                <a class="flex text-gray-600 hover:text-blue-500
                    cursor-pointer transition-colors duration-300">
                    {"Home"}
                </a>

                <a class="flex text-gray-600 hover:text-blue-500
                    cursor-pointer transition-colors duration-300">
                    {"Developers"}
                </a>

                <a class="flex text-gray-600 hover:text-blue-500
                    cursor-pointer transition-colors duration-300">
                   {"About Us"}
                </a>
            </div>

            <div class="flex items-center space-x-5">
                <a class="flex text-gray-600 hover:text-blue-500
                    cursor-pointer transition-colors duration-300">
                    {"Register"}
                </a>
                <a class="flex text-gray-600
                    cursor-pointer transition-colors duration-300
                    font-semibold text-blue-600">

                    {"Login"}
                </a>
            </div>
        </nav>

        }
    }
}
