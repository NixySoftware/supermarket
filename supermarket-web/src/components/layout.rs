use leptos::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <header class="fixed inset-x-0">
            <Topbar />
        </header>

        <main class="container h-screen w-screen pt-16">
            {children()}
        </main>

        <footer />
    }
}

#[component]
fn Topbar() -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <div class="border-b">
                <div class="flex h-16 items-center px-4">
                    <div class="flex items-center space-x-2">
                        <a href="#" class="hover:text-primary text-md font-medium transition-colors">
                            Supermarket
                        </a>
                    </div>

                    <div class="ml-auto flex items-center space-x-4">
                        <a href="#" class="text-muted-foreground hover:text-primary text-sm font-medium transition-colors">
                            Sign in
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
