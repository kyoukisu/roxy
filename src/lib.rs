use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum RoxyType {
    Http,
    Https,
    Socks4,
    Socks5,
}

#[derive(Error, Debug)]
pub enum RoxyError<'a> {
    #[error("RoxyError: {0}")]
    FormatError(&'a str)
}

impl RoxyType {
    fn from_str(input: &str) -> RoxyType {
        match input {
            "https://" => RoxyType::Https,
            "socks4://" => RoxyType::Socks4,
            "socks5://" => RoxyType::Socks5,
            _ => RoxyType::Http, // by default all converts to "http://"
        }
    }
}

impl fmt::Display for RoxyType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RoxyType::Http => fmt.write_str("http://")?,
            RoxyType::Https => fmt.write_str("https://")?,
            RoxyType::Socks4 => fmt.write_str("socks4://")?,
            RoxyType::Socks5 => fmt.write_str("socks5://")?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Roxy {
    pub protocol: RoxyType,
    pub ip: String,
    pub port: String,
    pub login: Option<String>,
    pub password: Option<String>,
}

pub fn new(proxy: &str) -> Result<Roxy,RoxyError> {
    Roxy::new(proxy)
}

// Changes `protocol://aaa:aaa@bbb:bbb` to `protocol://bbb:bbb@aaa:aaa`
pub fn change_proxy_format(proxy: &str) -> String {
    let mut parts: Vec<&str> = proxy.split('@').collect();
    let auth = parts.remove(0).trim_start_matches("http://");
    let mut auth_parts: Vec<&str> = auth.split(':').collect();
    let login = auth_parts.remove(0);
    let password = auth_parts.remove(0);
    let mut endpoint_parts: Vec<&str> = parts[0].split(':').collect();
    let ip = endpoint_parts.remove(0);
    let port = endpoint_parts.remove(0);
    format!("http://{}:{}@{}:{}", ip, port, login, password)
}

impl Roxy {
    // protocol://login:password@ip:port
    pub fn lpip(&self) -> String{
        if let (Some(login),Some(password))=(self.login.clone(),self.password.clone()){
            format!("{}{}:{}@{}:{}",self.protocol,login,password,self.ip,self.port)
        } else {
            format!("{}{}:{}",self.protocol,self.ip,self.port)
        }
    }
    // protocol://ip:port@login:password
    pub fn iplp(&self) -> String{
        if let (Some(login),Some(password))=(self.login.clone(),self.password.clone()){
            format!("{}{}:{}@{}:{}",self.protocol,self.ip,self.port,login,password)
        } else {
            format!("{}{}:{}",self.protocol,self.ip,self.port)
        }
    }
    pub fn new(proxy: &str) -> Result<Self,RoxyError> {
        let protocol = RoxyType::from_str(match proxy.find("://") {
            Some(index) => &proxy[..index + 3],
            None => "http://",
        });
        let str_protocol = protocol.to_string();
        let str_protocol = str_protocol.as_str();
        let remaining: &str = match proxy.strip_prefix(str_protocol){
            Some(a)=>a,
            None=>proxy
        };
        let sobaka_count = remaining.matches('@').count();
        match sobaka_count {
            0 => {
                let sep_count = remaining.matches(':').count();
                match sep_count {
                    0 => {
                        // http://ip
                        Err(RoxyError::FormatError("No : found in proxy string"))
                    },
                    1 => {
                        // http://ip:port
                        let (ip, port) =  match remaining.split_once(':') {
                            Some((_ip,_port)) => (_ip.to_string(), _port.to_string()),
                            None => ("".to_string(),"".to_string())
                        };
                        if ip.eq("") || port.eq("") {
                            return Err(RoxyError::FormatError("Empty login passowrd found in proxy string"));
                        }
                        Ok(Self{protocol,ip,port,login:None,password:None})
                    },
                    3 => {
                        let parts: Vec<&str> = remaining.split(':').collect();
                        let f_a = parts[0];
                        let f_b = parts[1];
                        let s_a = parts[2];
                        let s_b = parts[3];
                        if f_a.is_empty() || f_b.is_empty() || s_a.is_empty() || s_b.is_empty() {
                            return Err(RoxyError::FormatError("Bad proxy string"));
                        }
                        if f_a.contains('.') {
                            Ok(Self{protocol,ip:f_a.to_string(),port:f_b.to_string(),login:Some(s_a.to_owned()),password:Some(s_b.to_owned())})
                        } else {
                            Ok(Self{protocol,ip:s_a.to_owned(),port:s_b.to_owned(),login:Some(f_a.to_string()),password:Some(f_b.to_string())})
                        }
                        
                    }
                    _ => {
                        // http://ip:port:something
                        Err(RoxyError::FormatError("Multiple : found in proxy string"))
                    },
                }
            }
            1 => {
                // http://aaa:aaa@aaa:aaa
                let parts: Vec<&str> = remaining.split('@').collect();
                let first_part = parts[0];
                let second_part = parts[1];
                if first_part.matches(':').count() != 1 || second_part.matches(':').count() != 1{
                    return Err(RoxyError::FormatError("Wrong amount of : found in proxy string"))
                }
                if let (Some((f_a, f_b)),Some((s_a,s_b))) = (first_part.split_once(':'),second_part.split_once(':')){
                    if f_a.is_empty() || f_b.is_empty() || s_a.is_empty() || s_b.is_empty() {
                        return Err(RoxyError::FormatError("Bad proxy string"));
                    }
                    if f_a.contains('.') {
                        Ok(Self{protocol,ip:f_a.to_string(),port:f_b.to_string(),login:Some(s_a.to_owned()),password:Some(s_b.to_owned())})
                    } else {
                        Ok(Self{protocol,ip:s_a.to_owned(),port:s_b.to_owned(),login:Some(f_a.to_string()),password:Some(f_b.to_string())})
                    }
                }
                else {
                    return  Err(RoxyError::FormatError("Wrong amount of : found in proxy string"));
                }
            }
            _ => {
                // http://aaa@aaa@aaa
                Err(RoxyError::FormatError("Multiple @ found in proxy string"))
            }
        }
    }
}
