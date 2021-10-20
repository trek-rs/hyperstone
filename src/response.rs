use crate::{
    anyhow::{Error, Result},
    header::{HeaderValue, CONTENT_LOCATION, CONTENT_TYPE, LOCATION},
    Body, Response, StatusCode,
};

pub trait ResponseExt {
    /// Responds TEXT
    fn text(data: impl Into<Body>) -> Response<Body> {
        Self::with(data, mime::TEXT_PLAIN.as_ref())
    }

    /// Responds HTML
    fn html(data: impl Into<Body>) -> Response<Body> {
        Self::with(data, mime::TEXT_HTML.as_ref())
    }

    #[cfg(feature = "json")]
    /// Responds JSON
    fn json<T>(data: T) -> Result<Response<Body>>
    where
        T: serde::Serialize,
    {
        serde_json::to_vec(&data)
            .map(|v| Self::with(v, mime::APPLICATION_JSON.as_ref()))
            .map_err(Error::new)
    }

    /// Responds body with `Content-Type`
    fn with(data: impl Into<Body>, ct: &'static str) -> Response<Body> {
        let mut res = Response::new(data.into());
        res.headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static(ct));
        res
    }

    /// Sets the `Content-Location` header
    fn location(location: &'static str) -> Response<Body> {
        let mut res = Response::default();
        res.headers_mut()
            .insert(CONTENT_LOCATION, HeaderValue::from_static(location));
        res
    }

    /// Redirects to the URL derived from the specified path
    fn redirect(location: &'static str, status: StatusCode) -> Response<Body> {
        let mut res = Response::default();
        *res.status_mut() = status;
        res.headers_mut()
            .insert(LOCATION, HeaderValue::from_static(location));
        res
    }
}

impl ResponseExt for Response<Body> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response() -> Result<()> {
        let res = Response::text("hello world");
        assert_eq!(
            res.headers().get(CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::TEXT_PLAIN.as_ref()))
        );

        let res = Response::html("hello world");
        assert_eq!(
            res.headers().get(CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::TEXT_HTML.as_ref()))
        );

        let res = Response::json(0)?;
        assert_eq!(
            res.headers().get(CONTENT_TYPE),
            Some(&HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()))
        );

        Ok(())
    }
}
