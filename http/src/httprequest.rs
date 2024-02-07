use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum  Method{
    POST,
    GET,
    Uninitialized //数据结构初始化
}

#[derive(Debug, PartialEq)]
pub enum Version{
    V1_1,
    V2_0,
    Uninitialized
}

#[derive(Debug, PartialEq)]
pub enum Resource{
    Path(String)
}


pub struct HttpRequest{
    pub method:Method,
    pub version:Version,
    pub resource:Resource,
    pub headers:HashMap<String, String>,
    pub msg_body:String
}
fn process_req_line(line:&str) -> (Method, Resource, Version){
    let mut words = line.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (method.into(), Resource::Path(resource.to_string()), version.into())

}

fn process_header_line(line:&str) -> (String, String){
    let mut header_item = line.split(":");
    let mut key = String::new();
    let mut value = String::new();
    if let Some(k) = header_item.next(){
        key = k.to_string()
    }
    if let Some(v) = header_item.next() {
        value = v.to_string()
    }
    (key, value)
}
impl From<String> for HttpRequest{
    fn from(value: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let parsed_msg_body = "".to_string();
        for line in value.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            }else if line.contains(":"){
                let (key, values) = process_header_line(line);
                parsed_headers.insert(key, values);
            }else if line.len() == 0 {

            }
        }
        HttpRequest {
            method:parsed_method,
            version:parsed_version,
            resource:parsed_resource,
            headers:parsed_headers,
            msg_body:parsed_msg_body
        }
    }
}

impl From<&str> for Version{
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized
        }
    }
}

impl From<&str> for Method{
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::Uninitialized
        }
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_method(){
        let m:Method = "GET".into();
        assert_eq!(m,Method::GET);
    }

    #[test]
    fn test_version(){
        let v:Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1)
    }

    #[test]
    fn test_http(){
        let s:String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.1.1\r\nAccept: */*\r\n\r\n");
        let mut headers = HashMap::new();
        headers.insert("Host".into(), " localhost".into());
        headers.insert("Accept".into(), " */*".into());
        headers.insert("User-Agent".into(), " curl/7.1.1".into());
        let req : HttpRequest = s.into();
        assert_eq!(Method::GET, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers, req.headers)
    }
}