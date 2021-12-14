use actix_web::{HttpServer, App, web, HttpResponse, Responder};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use std::fs;
use std::io::prelude::*;
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
struct Group {
    username: String,
    email: String,
    group: String,
}
#[derive(Debug, Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}
#[derive(Debug, Deserialize)]
struct Submission {
    title: String,
    link: Option<String>,
}

async fn submission(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Submit a Post");

    let rendered = tera.render("submission.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn process_submission(data: web::Form<Submission>) -> impl Responder {
    println!("{:?}", data);
    let arg1 = &data.link;
    println!("{:?}", &arg1);
    
    let mut file = File::create("title");
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
    let mut msg = "Successsfully saved user: ";
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
    if  String::from_utf8_lossy(&foo.stdout).len() == 0 
    {
        msg = "user might be in database error saving user:";
    }
    HttpResponse::Ok().body(format!("{} {}",msg, data.username))
}
async fn process_group(data: web::Form<User>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}
async fn group(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
 
    let posts = [
        Post {
            title: String::from("make a new group"),
            link: String::from("/newGroup"),
            author: String::from("")
        },
        Post {
            title: String::from("enter a group"),
            link: String::from("/joinGroup"),
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

        Post{
            title: String::from("logout"),
            link: String::from("/logout"),
            author: String::from("")
        },
        Post{
            title: String::from("Group"),
            link: String::from("/group"),
            author: String::from("")
        },    
        Post{
            title: String::from("Submission"),
            link: String::from("/submission"),
            author: String::from("")
        },




 
    ];

    data.insert("posts", &posts);

    data.insert("title", "Web_App");
    data.insert("name","Preston");

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn login(tera: web::Data<Tera>, id: Identity) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Login");
    if let Some(_id) = id.identity()
    {
        return HttpResponse::Ok().body("already logged in.");
    }
    
    let rendered = tera.render("login.html", &data).unwrap();
    
    HttpResponse::Ok().body(rendered)
}

async fn process_login(data: web::Form<LoginUser>,id: Identity) -> impl Responder {
    println!("{:?}", data);
    let  arg1 = &data.username;
    let  arg2 = &data.password;
    let foo = Command::new("python")
                      .arg("login.py")
                      .arg(arg1)
                      .arg(arg2)
                      .output().unwrap();
    println!("python: {}", String::from_utf8_lossy(&foo.stdout));
    
    let filename = "login.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut _user = String::with_capacity(80);
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        println!("{} {}",index, line);
        _user = line;
    }
    let foo2 = Command::new("python")
                      .arg("clear.py")
                      .output().unwrap();
    
    println!("python: {}", String::from_utf8_lossy(&foo2.stdout));
    println!("Debug: {}",_user );
    let foo3 = Command::new("python")
                      .arg("strcmp.py")
                      .arg(arg1)
                      .arg(arg1 )
                      .output().unwrap();
    println!("python: {}", String::from_utf8_lossy(&foo3.stdout));
    if _user  != ""
    {
        let session_token = String::from(arg1);
        id.remember(session_token);

        HttpResponse::Ok().body(format!("Logged in: {}", data.username))
    }
    else
    {
        HttpResponse::Ok().body(format!("User {} not valid", data.username))
    }
   
     
}

async fn logout(id: Identity) -> impl Responder
{
    id.forget();
    HttpResponse::Ok().body("Logged out.")
}

async fn new_group(tera: web::Data<Tera>,id: Identity) -> impl Responder
{
    let mut data = Context::new();
    data.insert("title", "Login");
    
    let rendered = tera.render("newGroup.html", &data).unwrap();
    
    if let Some(_id) =  id.identity()
    {
       return  HttpResponse::Ok().body(rendered)
    }
    
        return HttpResponse::Ok().body("login before you can make a new group");

}

async fn join_group(tera: web::Data<Tera>,id: Identity) -> impl Responder
{
    let mut data = Context::new();
    data.insert("title", "Login");
    
    let rendered = tera.render("joinGroup.html", &data).unwrap();
    
    if let Some(_id) =  id.identity()
    {
       return  HttpResponse::Ok().body(rendered)
    }
    
        return HttpResponse::Ok().body("login before you can join a new group");

}
async fn process_join_group(tera: web::Data<Tera>,data: web::Form<Group>) -> impl Responder 
{

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

        Post{
            title: String::from("logout"),
            link: String::from("/logout"),
            author: String::from("")
        },
        Post{
            title: String::from("Group"),
            link: String::from("/group"),
            author: String::from("")
        },    
        Post{
            title: String::from("Submission"),
            link: String::from("/submission"),
            author: String::from("")
        },




 
    ];

    data.insert("posts", &posts);

    data.insert("title", "Web_App");
    data.insert("name","Preston");

    let rendered = tera.render("base.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)


}

async fn process_new_group(tera: web::Data<Tera>,data: web::Form<Group>) -> impl Responder {
    let mut data = Context::new();
 
    let posts = [
        Post {
            title: String::from("File"),
            link: String::from("hello"),
            author: String::from("Preston")
        },
        Post {
            title: String::from("File"),
            link: String::from("hello"),
            author: String::from("Preston")
        },

       Post {
            title: String::from("File"),
            link: String::from("hello"),
            author: String::from("Preston")
        },

       Post {
            title: String::from("File"),
            link: String::from("hello"),
            author: String::from("Preston")
        },

       Post {
            title: String::from("File"),
            link: String::from("hello"),
            author: String::from("Preston")
        },



 
    ];

    data.insert("posts", &posts);

    data.insert("title", "Web_App");
    data.insert("name","Preston");

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
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
            .route("/newGroup", web::post().to(process_new_group))
            .route("/submission", web::get().to(submission))
            .route("/submission", web::post().to(process_submission))
            .route("/group", web::get().to(group))
           // .route("/group", web::post().to(process_group))
            .route("/logout",web::to(logout))
            .route("/newGroup",web::get().to(new_group))
            .route("/joinGroup",web::get().to(join_group))
            .route("/joinGroup",web::post().to(process_join_group))

    })
    .bind("192.168.128.197:5765")?
    .run()
    .await
}

