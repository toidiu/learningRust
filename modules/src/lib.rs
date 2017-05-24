pub mod client;

pub mod network;

#[cfg(test)]
mod tests {

    use super::client;

    #[test]
    fn it_works() {
        //start from root module
        ::client::connect();

        //one up module
        super::client::connect();

        //uses `use`
        client::connect();
    }
}
