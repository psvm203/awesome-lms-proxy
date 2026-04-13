use worker::*;

pub trait HeadersExt {
    fn with_urlencoded(self) -> Self;
    fn with_set_cookie(self, cookie: &str) -> Self;
    fn with_cookie(self, cookie: &str) -> Self;
}

impl HeadersExt for Headers {
    fn with_urlencoded(self) -> Self {
        self.set("Content-Type", "application/x-www-form-urlencoded")
            .expect("Invalid header name");

        self
    }

    fn with_set_cookie(self, cookie: &str) -> Self {
        self.set("Set-Cookie", cookie).expect("Invalid header name");

        self
    }

    fn with_cookie(self, cookie: &str) -> Self {
        self.set("Cookie", cookie).expect("Invalid header name");

        self
    }
}

pub trait ResponseExt {
    fn ok(&self) -> bool;
    fn redirect(&self) -> bool;
}

impl ResponseExt for Response {
    fn ok(&self) -> bool {
        (200..300).contains(&self.status_code())
    }

    fn redirect(&self) -> bool {
        (300..400).contains(&self.status_code())
    }
}
