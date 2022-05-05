use std::fs::File;
use std::io::Read;
use ws::{listen, Handler, Message, Request, Response, Result, Sender};

fn immut_from_file(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).expect("no file found");
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).expect("broken read");
    return buffer;
}

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_request(&mut self, req: &Request) -> Result<Response> {
        match req.resource() {
            "/" => Ok(Response::new(
                200,
                "OK",
                immut_from_file("/root/static/index.html"),
            )),
            "/aes_gcm.js" => {
                let mut response = Response::new(
                    200,
                    "OK",
                    immut_from_file("/root/static/aes_gcm.js"),
                );
                let headers = response.headers_mut();
                headers.push((String::from("Content-Type"), Vec::<u8>::from("application/javascript".as_bytes())));
                return Ok(response);
            }
            "/aes_gcm_bg.wasm" => {
                let mut response = Response::new(
                    200,
                    "OK",
                    immut_from_file("/root/static/aes_gcm_bg.wasm"),
                );
                let headers = response.headers_mut();
                headers.push((String::from("Content-Type"), Vec::<u8>::from("application/wasm".as_bytes())));
                return Ok(response);
            },
            _ => Response::from_request(req),
        }
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.out.broadcast(msg)
    }
}

fn main() {
    listen("127.0.0.1:8000", |out| Server { out }).unwrap()
}
