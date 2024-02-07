use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a>{ //如果成员触及到引用类型就要加生命周期
    version:&'a str,
    status_code: &'a str,
    status_text:&'a str,
    headers:Option<HashMap<&'a str, &'a str>>,
    body:Option<String>
}

impl<'a> Default for HttpResponse<'a>{
    fn default() -> Self {
        Self {
            version:"HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl <'a> From<HttpResponse<'a>> for String{
    fn from(value: HttpResponse<'a>) -> Self {
        let res = value.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res.version(),
            &res.status_code(),
            &res.status_text(),
            &res.headers(),
            &value.body.unwrap().len(), //这里
            &res.body()
        )
    }
}

impl <'a>  HttpResponse<'a>{
    pub fn new(
        status_code:&'a str,
        headers:Option<HashMap<&'a str, &'a str>>,
        body:Option<String>
    ) -> HttpResponse<'a >{
        let mut response:HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        };
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
        };
        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into()
        };
        response.body = body;
        response
    }

    pub fn send_response(&self, write_stream:&mut impl Write) -> Result<(), ()>{
        let res = self.clone();
        let response_string:String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }

    fn version(&self) -> &str{
        self.version
    }

    fn status_code(&self) -> &str{
        self.status_code
    }

    fn status_text(&self) ->&str{
        self.status_text
    }

    fn headers(&self) ->String{
        let map:HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string:String = "".into();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v)
        }
        header_string
    }

    pub fn body(&self) ->&str{
        match &self.body {
            None => "",
            Some(b) => b.as_str()
        }
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_200_response(){
        let response_actual = HttpResponse::new(
            "200",
            None,
            Some("xxx".into()),
        );
        let response_exp = HttpResponse{
            version: "HTTP/1.1",
            status_text: "OK",
            status_code:"200",
            headers:{
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body:Some("xxx".into()),
        };

        assert_eq!(response_actual, response_exp);

    }

    #[test]
    fn test_404_response(){
        let response_actual = HttpResponse::new(
            "404",
            None,
            Some("xxx".into()),
        );
        let response_exp = HttpResponse{
            version: "HTTP/1.1",
            status_text: "Not Found",
            status_code:"404",
            headers:{
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body:Some("xxx".into()),
        };

        assert_eq!(response_actual, response_exp);
    }

    #[test]
    fn test_response_creation(){
        let response_ex = HttpResponse{
            version:"HTTP/1.1",
            status_code:"404",
            status_text:"Not Found",
            headers:{
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body:Some("xxx".into()),
        };
        let http_string:String = response_ex.into();
        let actual_string = "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 3\r\n\r\nxxx";
        assert_eq!(http_string, actual_string);
    }
}