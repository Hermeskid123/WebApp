use actix_web::{HttpServer, App, web, HttpResponse, Responder};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
use std::process::Command;

use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};

#[derive(Serialize)]
struct Post {
    title: String,
    link: String,
    author: String,
}


#[derive(Debug, Deserialize)]
struct User {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}
#[derive(Debug, Deserialize)]
struct Submission {
    title: String,
    link: String,
}

async fn submission(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Submit a Post");

    let rendered = tera.render("submission.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn process_submission(data: web::Form<Submission>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Posted submission: {}", data.title))
}

async fn signup(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Sign Up");

    let rendered = tera.render("signup.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn process_signup(data: web::Form<User>) -> impl Responder {
    println!("{:?}", data);
    let  arg1 = &data.username;
    let  arg2 = &data.email;
    let  arg3 = &data.password;
        let foo = Command::new("python")
                      .arg("signup.py")
                      .arg(arg1)
                      .arg(arg2)
                      .arg(arg3)
                      .output().unwrap();
    println!("python: {}", String::from_utf8_lossy(&foo.stdout));
    
    println!("length: {}",String::from_utf8_lossy(&foo.stdout).len());
    
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}
async fn process_group(data: web::Form<User>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}
async fn group(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
 
    let posts = [
        Post {
            title: String::from("Login"),
            link: String::from("/login"),
            author: String::from("")
        },
        Post {
            title: String::from("New User"),
            link: String::from("/signup"),
            author: String::from("")
        },
    ];

    data.insert("posts", &posts);

    data.insert("title", "Web_App");
    data.insert("name","Preston");

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}
async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
 
    let posts = [
        Post {
            title: String::from("Login"),
            link: String::from("/login"),
            author: String::from("")
        },
        Post {
            title: String::from("New User"),
            link: String::from("/signup"),
            author: String::from("")
        },
    ];

    data.insert("posts", &posts);

    data.insert("title", "Web_App");
    data.insert("name","Preston");

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn login(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Login");
    
    let rendered = tera.render("login.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn process_login(data: web::Form<LoginUser>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Logged in: {}", data.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("127.0.0.1:8000");
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
             .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(&[0;32])
                    .name("auth-cookie")
                    .secure(false)
                )
            )           
             .data(tera)
            .route("/", web::get().to(index))
            .route("/signup", web::get().to(signup))
            .route("/login", web::get().to(login))
            .route("/login", web::post().to(process_login))
            .route("/signup", web::post().to(process_signup))
            .route("/submission", web::get().to(submission))
            .route("/submission", web::post().to(process_submission))
            .route("/group", web::get().to(group))
            .route("/group", web::post().to(process_group))


    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

