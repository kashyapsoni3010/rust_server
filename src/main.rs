use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    //create listener and binding the listener
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //for loop on stream data received by the server
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        //function to handle the connection and requests
        handle_connection(stream);
    }
    
}


fn handle_connection(mut stream: TcpStream, ){
    //store the incoming request
    let mut request=[0;1024];
    //read input stream contents and store them in request 
    stream.read(&mut request).unwrap();
    let reqstr = String::from_utf8_lossy(&request[..]).to_string();
    let lines: Vec<&str> = reqstr.split("\n").collect();
    // println!("{}", lines[0]);
    // println!("{}", lines.len());
    let req = String::from(lines[0]);
    let filename: String = request_action(&req).to_string();
    send_response(stream, &filename);
}

fn send_response(mut stream: TcpStream, filename: &str){
    let contents = fs::read_to_string(filename).unwrap();
    let header = "HTTP/1.1 200 OK";
    let len = contents.len();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",header, len, contents);
    //write response to the tcpstream
    stream.write(response.as_bytes()).unwrap();
    //send the response
    stream.flush().unwrap();
}

fn request_action(req: &str) -> String{
    let mut filename: String = "".to_string();
    match  &req[..]{
        "GET / HTTP/1.1\r" => filename = "index.html".to_string(),
        "GET /action HTTP/1.1\r" => filename = "action.html".to_string(),
        "GET /fiction HTTP/1.1\r" => filename = "fiction.html".to_string(),
        "GET /drama HTTP/1.1\r" => filename = "drama.html".to_string(),
        "GET /comedy HTTP/1.1\r" => filename = "comedy.html".to_string(),
        "GET /games HTTP/1.1\r" => filename = "games.html".to_string(),
        _ => filename = "notfound.html".to_string(),
    }
    return filename
}
