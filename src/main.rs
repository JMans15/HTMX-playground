use anyhow::Result;

use actix_files::NamedFile;
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use leptos::*;
use std::process::Command;

#[get("/")]
async fn index(_req: HttpRequest) -> HttpResponse {
    let html = leptos::ssr::render_to_string(move |cx| {
        view! { cx,
            <head>
                <style>"
                    #myimage {
                      animation: fadeIn 1s;
                    }
                    @keyframes fadeIn {
                        0% {opacity: 0;}
                        100% {opacity: 1;}
                    }
                    .btn {
                      display: inline-block;
                      padding: 10px 20px;
                      font-size: 16px;
                      font-weight: bold;
                      text-align: center;
                      text-decoration: none;
                      border: none;
                      border-radius: 5px;
                      cursor: pointer;
                      background-color: #313131;
                      color: #fff;
                      transition: background-color 0.3s ease, transform 0.3s ease;
                    }

                    .btn:hover {
                      background-color: #313131;
                      transform: scale(1.05);
                    }

                    .btn:active {
                      transform: scale(0.95);
                    }
                "</style>
                <script src="https://unpkg.com/htmx.org@1.9.2" integrity="sha384-L6OqL9pRWyyFU3+/bjdSri+iIphTN/bvYyM37tICVyOJkWZLpP2vGn6VUEXgzg6h" crossorigin="anonymous"></script>
            </head>
            <body>
            <button class="btn" hx-swap="outerHTML" hx-get="/loadimg" hx-trigger="click"> Lazy Load Image </button>
            </body>
        }
    });

    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html);
}

#[get("/loadimg")]
async fn loading_text(_req: HttpRequest) -> HttpResponse {
    let html = leptos::ssr::render_to_string(move |cx| {
        view! { cx,
            <div hx-get="/img" hx-trigger="load">
                <button class="btn">Lazy Loading Image</button>
            </div>
        }
    });
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/default")]
async fn default_view(_req: HttpRequest) -> HttpResponse {
    let html = leptos::ssr::render_to_string(move |cx| {
        view! { cx,
            <button class="btn" hx-swap="outerHTML" hx-get="/loadimg" hx-trigger="click"> Lazy Load Image </button>
        }
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/img")]
async fn load_image(_req: HttpRequest) -> HttpResponse {
    let html = leptos::ssr::render_to_string(move |cx| {
        view! { cx,
            <div id="parent-div">
            <img id="myimage" src="./static/image.png" width=800/><br/><br/>
            <text> Image lazy loaded without page being refreshed </text><br/><br/>
            <button class="btn" hx-get="/default" hx-trigger="click" hx-target="#parent-div"> Reset </button>
            </div>
        }
    });

    let mut child = Command::new("sleep").arg("1").spawn().unwrap();
    let _result = child.wait().unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(index)
            .service(loading_text)
            .service(default_view)
            .service(load_image)
            .route("static/image.png", web::get().to(get_image))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

async fn get_image(_req_: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/bigimage.png")?)
}
