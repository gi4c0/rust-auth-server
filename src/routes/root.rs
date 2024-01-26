use maud::{html, Markup};

pub async fn root() -> Markup {
    html! {
        head {
            script src="https://unpkg.com/htmx.org@1.9.10" {}
            script src = "https://cdn.tailwindcss.com" {}
        }

        body {
            div class="flex flex-col min-h-screen" {
                header class="flex h-18 items-center border-b bg-gray-100/40 px-4 py-2" {
                    nav class="flex-1 flex items-center gap-4" {
                        a class="flex items-center gap-2 font-semibold" href="#" rel="ugc" {
                            svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4" {
                                path d="M3 9h18v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V9Z" {}
                                path d="m3 9 2.45-4.9A2 2 0 0 1 7.24 3h9.52a2 2 0 0 1 1.8 1.1L21 9" {}
                                path d="M12 3v6" {}
                            }
                            span class="sr-only" {"Acme Inc"}
                        }
                        a class="text-sm font-medium" href="#" rel="ugc" { "Home" }
                        a class="text-sm font-medium" href="#" rel="ugc" { "Shop" }
                        a class="text-sm font-medium" href="#" rel="ugc" { "About Us" }
                        a class="text-sm font-medium" href="#" rel="ugc" { "Contact" }
                    }
                    div class="flex items-center gap-4" {
                        form class="relative mb-0" {
                            div {
                                svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    class="absolute left-2.5 top-2.5 h-4 w-4 text-gray-500 dark:text-gray-400" {
                                        circle cx="11" cy="11" r="8" {}
                                        path d="m21 21-4.3-4.3" {}
                                    }

                                input
                                    class="flex h-10 rounded-md border border-input px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 w-full bg-white shadow-none appearance-none pl-8 md:w-1/2 lg:w-1/2"
                                    placeholder="Search products..."
                                    type="search" {}
                            }
                        }

                        button class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 hover:bg-accent hover:text-accent-foreground h-10 w-10" {
                            svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="h-4 w-4" {
                                    circle cx="8" cy="21" r="1" {}
                                    circle cx="19" cy="21" r="1" {}
                                    path d="M2.05 2.05h2l2.66 12.42a2 2 0 0 0 2 1.58h9.78a2 2 0 0 0 1.95-1.57l1.65-7.43H5.12" {}
                                }
                            span class="sr-only" { "Toggle shopping cart" }
                        }
                    }
                }
            }
        }
    }
}
