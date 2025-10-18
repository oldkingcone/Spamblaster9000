use std::ops::Index;
use rand::distr::Alphanumeric;
use rand::Rng;

const RANDOM_PROVIDERS: [&str; 9] = [
    "gmail.com",
    "yandex.ru",
    "yahoo.com",
    "outlook.com",
    "live.com",
    "globemail.com",
    "proton.me",
    "protonmail.com",
    "aol.com",
];

pub struct Telegram {
    pub target_url: String,
    pub jitter: Option<Vec<f64>>,
}

impl Telegram {
    pub fn new() -> Self {
        Self {
            target_url: String::new(),
            jitter: None,
        }
    }

    pub fn set_target_url(&mut self, target_url: String) {
        self.target_url = target_url;
    }

    pub fn set_jitter(&mut self, jitter: Option<Vec<f64>>) {
        self.jitter = jitter;
    }
    
    pub fn build_body(&mut self, target_url: &str) -> String {
        let mut target_url = target_url.to_string();
        if target_url.ends_with("SendMessage"){
            target_url.push_str("?text=");
        } else {
            target_url.push_str("SendMessage?text=");
        }
        let a = format!("{target_url}YOU+SMELL+LIKE+TURDS%0D%0AIP:+{fake_ip}+Page+name:+https://facebook.com/{fake_name}+Email/Mobile:+{fake_tel}+Full+name:+{fake_name}+Additional+info:++Password:{fake_pw}+2Fac:+{fake_2fac}",
                        target_url=target_url,
                        fake_ip=self.generate_random_ip(),
                        fake_name=self.generate_random_name(),
                        fake_tel=self.generate_random_phone(),
                        fake_2fac=self.generate_random_2fac(),
                        fake_pw=self.generate_random_password()
        );
        a
        
    }

    fn generate_random_username(&mut self) -> String {
        let len = rand::rng().random_range(5..15);
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
    
    fn generate_random_password(&mut self) -> String {
        let len = rand::rng().random_range(5..99);
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
    
    fn generate_random_email(&mut self) -> String {
        let len = rand::rng().random_range(5..15);
        let mut r = rand::rng();
        let mut email = String::new();
        let r_base: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();
        let r_index = r.random_range(0..9);
        email.push_str(&r_base);
        email.push_str("@");
        email.push_str(RANDOM_PROVIDERS.index(r_index));
        email
    }

    fn generate_random_phone(&mut self) -> String {
        let mut r = rand::rng();
        let phone = format!("{}-{}-{}", r.random_range(100..999), r.random_range(100..999), r.random_range(1000..9999));
        phone
    }

    fn generate_random_2fac(&mut self) -> String {
        let mut r = rand::rng();
        let mut code = String::new();
        for _ in 0..6 {
            code.push_str(&r.random_range(0..9).to_string());
        }
        code
    }

    fn generate_random_name(&mut self) -> String {
        let mut r = rand::rng();
        let mut name = String::new();
        for _ in 0..r.random_range(1..5) {
            name.push_str(&r.random_range(0..9).to_string());
        }
        name
    }

    fn generate_random_ip(&mut self) -> String {
        let mut r = rand::rng();
        let mut ip = String::new();
        for _ in 0..4 {
            ip.push_str(&r.random_range(1..255).to_string());
            ip.push_str(".");
        }
        ip
    }
}