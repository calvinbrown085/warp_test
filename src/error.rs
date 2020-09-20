
use prometheus::Counter;

quick_error! {
    #[derive(Debug)]
    pub enum HttpError {
        ClientError {}
        ServerError {}
    }
}

fn handle_error(error: HttpError) -> String {
    match error {
        HttpError::ClientError => String::from("test"),
        HttpError::ServerError => String::from("test")
    }
}