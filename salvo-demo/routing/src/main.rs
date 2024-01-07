use salvo::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let debug_mode = true;
    let admin_mode = true;
    let router = Router::new()
        .get(index)
        .push(
            Router::with_path("users")
                .hoop(auth)
                .post(create_user)
                .push(
                    Router::with_path(r"<id:num>")
                        .post(update_user)
                        .delete(delete_user),
                ),
        )
        .push(
            Router::with_path("users")
                .get(list_users)
                .push(Router::with_path(r"<id:num>").get(show_user)),
        )
        .then(|router| {
            if debug_mode {
                router.push(Router::with_path("debug").get(debug))
            } else {
                router
            }
        })
        .then(|router| {
            if admin_mode {
                router.push(Router::with_path("admin").get(admin))
            } else {
                router
            }
        });
    println!("{router:#?}");

    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}

// curl "http://127.0.0.1:5800/"

// curl -X POST "http://127.0.0.1:5800/users" -d '{}' -H "Content-Type: application/json"
// curl -X POST "http://127.0.0.1:5800/users/1" -d '{}' -H "Content-Type: application/json"
// curl -X DELETE "http://127.0.0.1:5800/users/1" -d '{}' -H "Content-Type: application/json"

// curl "http://127.0.0.1:5800/users"
// curl "http://127.0.0.1:5800/users/1"

// curl "http://127.0.0.1:5800/debug"
// curl "http://127.0.0.1:5800/admin"

#[handler]
async fn admin(res: &mut Response) {
    res.render(Text::Plain("Admin page"));
}

#[handler]
async fn debug(res: &mut Response) {
    res.render(Text::Plain("Debug page"));
}

#[handler]
async fn index(res: &mut Response) {
    res.render(Text::Plain("Hello World!"));
}

#[handler]
async fn auth(res: &mut Response) {
    res.render(Text::Plain("user has authed\n\n"));
}

#[handler]
async fn list_users(res: &mut Response) {
    res.render(Text::Plain("list users"));
}

#[handler]
async fn show_user(res: &mut Response) {
    res.render(Text::Plain("show user"));
}

#[handler]
async fn create_user(res: &mut Response) {
    res.render(Text::Plain("user created"));
}

#[handler]
async fn update_user(res: &mut Response) {
    res.render(Text::Plain("user updated"));
}

#[handler]
async fn delete_user(res: &mut Response) {
    res.render(Text::Plain("user deleted"));
}
