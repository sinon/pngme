use maud::{html, Markup};

/// Renders the form using Maud
pub fn render_form() -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="stylesheet" href="https://yree.io/mold/assets/css/main.css";
                link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🤫</text></svg>";
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="";
                link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&amp;display=swap" rel="stylesheet";
                title { "Pngme: Hide secrets message in .png files 🤫" }
                script src="https://unpkg.com/htmx.org" {}
            }
            body a="auto" {
                main class="content" aria-label="Content" {
                    div class="w" id="markdown-view"  {
                        h1 { "Pngme 🕵️‍♀️" }
                        p { "Hide secret message in .png files 🤫" }
                        h2 { "Upload file" }
                        form hx-post="/submit" hx-target="#response" {
                            div class="grid" {
                                input
                                    type="text"
                                    id="name"
                                    name="name"
                                    placeholder="Upload your file"
                                    required;
                                input type="radio" id="encode" name="encode" value="encode";
                                br;
                                " "
                                label for="encode" { "Do you want to add a message to the file?" }
                                input type="radio" id="decode" name="decode" value="decode";
                                br;
                                " "
                                label for="decode" { "Do you want to read a message form the file?" }
                                input type="radio" id="clear" name="clear" value="clear";
                                br;
                                " "
                                label for="clear" { "Do you want to remove a hidden message from the file?" }
                                button type="submit" { "Submit" }
                            }
                        }
                        br;
                        div id="response" {
                            p { "Hello world!" }
                        }
                    }
                }
            }
            footer {
                div class="w" {
                    p { a href="https://github.com/sinon/pngme" { "pngme" } " 🤫" }
                }
            }
        }
    }
}

/// Renders the response when a form is submitted
pub fn render_response(name: &str) -> Markup {
    html! {
        p { "Hello, " (name) "!" }
    }
}
