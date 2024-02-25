use serde_json::Value;
use tokio::io::Result;
use xpress::middleware::Middleware;
use xpress::router::request::Request;

enum DataTypes {
    Json,
    // UrlEncoded,
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
            // DataTypes::UrlEncoded => self._url_encoded(req),
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
        if let Some(ref mut body) = req.body {
            let json_value: Option<Value>;

            if let Some(ref data) = body.raw {
                json_value = Some(serde_json::from_str(&data)?);
            } else {
                json_value = None;
            }

            if json_value.is_none() {
                body.json = json_value;
            }
        }

        Ok(())
    }

    // pub fn url_encoded() -> Self {
    //     Self {
    //         data_type: DataTypes::UrlEncoded,
    //     }
    // }

    // fn _url_encoded(&self, req: Request) -> Request {}

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
