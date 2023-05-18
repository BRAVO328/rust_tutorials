use rocket::Request;
use rocket::response::Redirect;

use rocket_dyn_templates::{Template, tera::Tera, context};

//  Templated pages
#[get("/")]
pub fn items() -> Template {
    Template::render("list/items.html", context! {
        title: "items",
    })
}

#[get("/create")]
pub fn create() -> Template {
    Template::render("list/create.html", context! {
        title: "create",
    })
}
#[get("/read")]
pub fn read() -> Template {
    Template::render("list/read.html", context! {
        title: "read",
    })
}
#[get("/update")]
pub fn update() -> Template {
    Template::render("list/update.html", context! {
        title: "update",
    })
}
#[get("/delete")]
pub fn delete() -> Template {
    Template::render("list/delete.html", context! {
        title: "delete",
    })
}


//  Error messages
#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("/error", context! {
        uri: req.uri()
    })
}

pub fn customize(tera: &mut Tera) {
    tera.add_raw_template("tera/about.html", r#"
        {% block content %}
            <section id="about">
              <h1>About - Here's another page!</h1>
            </section>
        {% endblock content %}
    "#).expect("valid Tera template");
}