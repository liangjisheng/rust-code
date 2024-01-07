async fn do_stuff_async() -> Result<&'static str, &'static str> {
    // async work
    Ok("do_stuff_async")
}

async fn more_async_work() -> Result<&'static str, &'static str> {
    // more here
    Ok("more_async_work")
}

#[tokio::main]
async fn main() {
    let res = tokio::try_join!(do_stuff_async(), more_async_work());

    match res {
        Ok((first, second)) => {
            // do something with the values
            println!("first {}, second {}", first, second);
        }
        Err(err) => {
            println!("processing failed; error = {}", err);
        }
    }
}
