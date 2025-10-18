use std::ops::Index;
use rand::{Rng, distr::Alphanumeric};

const BASELINE_JITTER: [f64; 2] = [1.0, 1.456];

pub struct WebHookBlaster {
    pub mention_everyone: bool,
    pub random_junk: bool,
    pub junk_data: Option<String>,
    pub add_jitter: Option<bool>,
    pub jitter_base: Option<Vec<f64>>,
    pub success_responses: Option<usize>,
    pub error_responses: Option<usize>,
    pub total_responses: Option<usize>,
}

impl WebHookBlaster {
    pub fn new() -> Self {
        Self {
            mention_everyone: false,
            random_junk: false,
            junk_data: None,
            add_jitter: None,
            jitter_base: Some(Vec::from(BASELINE_JITTER)),
            success_responses: Some(0),
            error_responses: Some(0),
            total_responses: Some(0),
        }
    }

    fn generate_random_username(&mut self) -> String {
        let len = rand::rng().random_range(5..15);
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }

    pub fn set_mention_everyone(&mut self, mention_everyone: bool) {
        self.mention_everyone = mention_everyone;
    }

    pub fn set_random_junk(&mut self, random_junk: bool) {
        self.random_junk = random_junk;
    }

    pub fn generate_junk(&mut self) {
        let len = rand::rng().random_range(10..100);
        self.junk_data = Some(rand::rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect());
    }

    pub fn adjust_jitter(&mut self) {
        let mut min = self.jitter_base.clone().unwrap().index(0).clone();
        let mut max = self.jitter_base.clone().unwrap().index(1).clone();
        min = min + 0.12;
        max = max + 0.12;
        self.jitter_base = Some(vec![min.clone(), max.clone()])
    }

    pub fn get_jitter_base(&self) -> Option<Vec<f64>> {
        self.jitter_base.clone()
    }

    pub fn assemble_data(&mut self) -> String {
        let mut data = String::new();
        if self.mention_everyone {
            data.push_str("@everyone ");
        }
        if self.random_junk {
            self.generate_junk();
        }
        if self.junk_data.is_some() {
            data.push_str(&self.junk_data.clone().unwrap());
        }
        data
    }

    pub fn build_body(&mut self, message: Option<String>) -> String {
        let data = self.assemble_data();
        let mut body = String::new();
        if self.mention_everyone {
            body.push_str("@everyone");
        }
        if self.random_junk {
            body.push_str("```");
            body.push_str(&self.junk_data.clone().unwrap());
            body.push_str("```");
        } else {
            body.push_str("```");
            body.push_str(&message.unwrap());
            body.push_str("```");
        }
        let u_name = format!(",\"username\":\"{}\"", self.generate_random_username());
        body.push_str("{\"content\": \"");
        body.push_str(&data);
        body.push_str(", \"avatar_url\": \"https://redirecthost.online/googly_eye.gif\"");
        body.push_str(u_name.as_str());
        body.push_str(&self.generate_random_username());
        body.push_str("\"}");
        body
    }
}
