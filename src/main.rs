# [macro_use] extern crate rocket;

mod tera;

// Adding ability to return raw html and the ability to use templates
//  use rocket::response::content::RawHtml;                                                 //  Adds the option to return raw HTML
use rocket_dyn_templates::Template;

//  Original web pages

# [get ("/")]
fn index () -> &'static str {
    "Homepage"
}

/*
# [get ("/list")]
fn list () -> &'static str {
    "list the records"
}

# [get ("/create")]
fn create () -> &'static str {
    "Create a new record"
}

# [get ("/read")]
fn read () -> &'static str {
    "Read a current record"
}

# [get ("/update")]
fn update () -> &'static str {
    "Update a current record"
}

# [get ("/delete")]
fn delete () -> &'static str {
    "Delete a current record"
}
*/

# [launch]
fn rocket () -> _ {
    rocket::build().mount("/", routes![index])
        .mount("/list", routes![tera::create, tera::read, tera::update, tera::delete])
        .register("/list", catchers![tera::not_found])
        .attach(Template::custom(|engines| {
            tera::customize(&mut engines.tera);
        }))
}