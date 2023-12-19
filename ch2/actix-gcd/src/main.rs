/*
date:2023/12/19
书中代码比较旧，改为actix 4 后需要加上异步等操作
*/
use actix_web::{web,post,App,HttpResponse,HttpServer,Responder};

/*
serde::Deserialize 是 Rust 编程语言中 serde 库的一部分，用于实现反序列化。
serde 是一个序列化和反序列化框架，广泛用于高效地处理数据的转换，比如将 JSON、YAML、XML 等格式的数据转换为 Rust 中的数据结构，或者反过来。
Deserialize trait 允许自定义数据结构能夠从各种数据表示（如 JSON）中被反序列化（解析）出来。
*/
use serde::Deserialize;


#[actix_web::main]
async fn main()-> std::io::Result<()>{
    let server = HttpServer::new(||{
        App::new().route("/",web::get().to(get_index))
        .service(post_gcd) 
        
    });
    println!("Serving on http://localhost:3000");
    server
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}

async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                    <input type="text" name="n">
                    <input type="text" name="m">
                    <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
    }
    
    #[derive(Deserialize)]
    struct GcdParameters{
        n: u64,
        m: u64,
    }

#[post("/gcd")]
async fn post_gcd(form: web::Form<GcdParameters>)-> impl Responder{
    if form.n == 0 || form.m == 0{
        return HttpResponse::BadRequest()
        .content_type("text/html")
        .body("computing the GCD with 0 is boring");
    }
    let response = 
        format!("the greatest common divisor of the number {} and {} \
         is <b>{}<b>",
         form.n,form.m,gcd(form.n, form.m));
        
    HttpResponse::Ok()
    .content_type("text/html")
    .body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        let t = m;
        m = n % m;
        n = t;
    }
    n
}