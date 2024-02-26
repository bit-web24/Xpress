use serde_urlencoded;
use tokio::io::Result;
use xpress::middleware::Middleware;
use xpress::router::request::Request;

enum DataTypes {
    Json,
    UrlEncoded,
    // Raw,
    // Buffer,
}

pub struct BodyParser {
    data_type: DataTypes,
}

impl Middleware for BodyParser {
    fn handle(&self, req: &mut Request) -> Result<()> {
        match self.data_type {
            DataTypes::Json => self._json(req),
            DataTypes::UrlEncoded => self._url_encoded(req),
            // DataTypes::Raw => self._raw(req),
            // DataTypes::Buffer => self._buffer(req),
        }
    }
}

impl BodyParser {
    pub fn json() -> Self {
        Self {
            data_type: DataTypes::Json,
        }
    }

    fn _json(&self, req: &mut Request) -> Result<()> {
        if req.headers.get("Content-Type") != Some(&"application/json".to_string()) {
            return Ok(());
        }

        if let Some(body) = &mut req.body {
            if body.json.is_some() {
                return Ok(());
            }

            if let Some(data) = &body.raw {
                let json_value = serde_json::from_str(data)?;
                body.json = Some(json_value);
            }
        }

        Ok(())
    }

    pub fn url_encoded() -> Self {
        Self {
            data_type: DataTypes::UrlEncoded,
        }
    }

    fn _url_encoded(&self, req: &mut Request) -> Result<()> {
        if let Some(content_type) = req.headers.get("Content-Type") {
            let expected = "application/x-www-form-urlencoded".to_string();

            if content_type == &expected {
                if let Some(body) = &mut req.body {
                    if body.url_encoded.is_some() {
                        return Ok(());
                    }

                    if let Some(data) = &body.raw {
                        match serde_urlencoded::from_str::<Vec<(String, String)>>(data) {
                            Ok(value) => {
                                body.url_encoded = Some(value);
                            }
                            Err(err) => {
                                println!("ERROR: {:?}", err);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    // pub fn raw() -> Self {
    //     Self {
    //         data_type: DataTypes::Raw,
    //     }
    // }

    // fn _raw(&self, req: Request) -> Request {}

    // pub fn buffer() -> Self {
    //     Self {
    //         data_type: DataTypes::Buffer,
    //     }
    // }

    // fn _buffer(&self, req: Request) -> Request {}
}
