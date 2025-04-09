#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{Template, context};
use serde::Serialize;

#[derive(Serialize)]
struct PageContent {
    title: String,
    meta_description: String,
}

#[get("/")]
fn index() -> Template {
    let content = PageContent {
        title: String::from("SCP: Secret Facility - Многопользовательская хоррор-игра"),
        meta_description: String::from("SCP: Secret Facility - захватывающая многопользовательская хоррор-игра, основанная на вселенной SCP. Погрузитесь в мир секретных исследований и аномальных объектов."),
    };
    
    Template::render("index", context! { 
        content: content,
    })
}

#[get("/about")]
fn about() -> Template {
    let content = PageContent {
        title: String::from("Об игре SCP: Secret Facility - Узнайте больше о хоррор-игре"),
        meta_description: String::from("Узнайте больше о многопользовательской хоррор-игре SCP: Secret Facility, основанной на вселенной SCP. Исследуйте мрачные коридоры секретного комплекса."),
    };
    
    Template::render("index", context! { 
        content: content,
        section: "about",
    })
}

#[get("/characters")]
fn characters() -> Template {
    let content = PageContent {
        title: String::from("Персонажи SCP: Secret Facility - Роли и классы"),
        meta_description: String::from("Познакомьтесь с персонажами игры SCP: Secret Facility - от ученых и сотрудников класса D до смертельно опасных SCP-объектов."),
    };
    
    Template::render("index", context! { 
        content: content,
        section: "characters",
    })
}

#[get("/zones")]
fn zones() -> Template {
    let content = PageContent {
        title: String::from("Зоны в SCP: Secret Facility - Исследуйте комплекс"),
        meta_description: String::from("Исследуйте опасные зоны комплекса в игре SCP: Secret Facility, включая зону повышенной опасности 15+."),
    };
    
    Template::render("index", context! { 
        content: content,
        section: "zones",
    })
}

#[get("/download")]
fn download() -> Template {
    let content = PageContent {
        title: String::from("Скачать SCP: Secret Facility - Присоединяйтесь к игре"),
        meta_description: String::from("Скачайте SCP: Secret Facility прямо сейчас и погрузитесь в мир секретной организации SCP и аномальных объектов."),
    };
    
    Template::render("index", context! { 
        content: content,
        section: "download",
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, about, characters, zones, download])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
