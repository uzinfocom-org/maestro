use reqwest::blocking::Client;

#[derive(Debug)]
pub struct Instance {
    /// Blocking client to make requests with.
    pub(crate) client: Client,
}

impl Instance {
    /// Construct a new instance of Instance struct with a new client.
    pub fn new() -> Instance {
        let client = Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        Instance { client }
    }

    /// Login to the kerio control panel and update the cookie store.
    pub fn login(&mut self, url: String, username: String, password: String) {
        self.client
            .post(url)
            .form(&[("kerio_username", username), ("kerio_password", password)])
            .send()
            .unwrap();
    }
}
