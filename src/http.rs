use reqwest::blocking::Client;

static LOGIN_URL: &str =
    "https://uic-gw.uzinfocom.uz/internal/dologin.php?NTLM=0&hash=myAccount_status_all";

#[derive(Debug)]
pub struct Instance {
    /// Blocking client to make requests with.
    pub(crate) client: Client,
}

impl Instance {
    /// Construct a new instance of Instance struct with a new client.
    pub fn new() -> Instance {
        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        Instance { client }
    }

    /// Login to the kerio control panel and update the cookie store.
    pub fn login(&mut self, username: String, password: String) {
        self.client
            .post(LOGIN_URL)
            .form(&[("kerio_username", username), ("kerio_password", password)])
            .send()
            .unwrap();
    }
}
