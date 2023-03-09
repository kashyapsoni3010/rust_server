use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
//static mut mymap: HashMap<String, String> = HashMap::new();
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
    println!("{}", lines[0]);
    println!("{}", lines.len());
    let req = String::from(lines[0]);
    //let filename: String = request_action(&req).to_string();
    
    match &req[..]{
        "POST / HTTP/1.1\r" =>handle_post(stream, lines[lines.len()-1].to_string()),
        _ =>send_response(stream, &request_action(&req).to_string()),
    }
    
}

fn handle_post(mut stream: TcpStream,postreq: String){
    let request: Vec<&str> = postreq.split("=").collect();
    let target = String::from(request[0]);
    let targetfile = [target, ".html".to_string()].join("");
    println!("{}", targetfile);
    println!("{}", request[1]);
    println!("{}", request.len());
    //let html = r#targetfile;
    //let mut dom = parse(targetfile).unwrap();
    let contents: Vec<&str> = fs::read_to_string(targetfile.clone()).unwrap().split("<br>").collect();
    //let mut file = File::OpenO(targetfile.clone()).unwrap();

//----
let file_result = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(targetfile.clone());

    match file_result {
        Ok(mut file) => {
            // Write data to the file
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            file.set_len(0).unwrap();
            let parts:Vec<&str> = contents.split("<br>").collect();
            let element: String = [parts[0],"<br>\n","<div><p><a href=\"",&request[1],"\">Link</a></p></div>", parts[1]].join("");
            file.write_all(&element.as_bytes()).unwrap();
            // send_response(stream, &targetfile);
            //let contents = fs::read_to_string(targetfile.clone()).unwrap();
            let header = "HTTP/1.1 200 OK";
            let len = element.len();
            let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",header, len, element);
            //write response to the tcpstream
            stream.write(response.as_bytes()).unwrap();
            //send the response
            stream.flush().unwrap();
        },
        Err(error) => {
            println!("Error: {}", error);
        }
    }

//---


    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // let parts:Vec<&str> = contents.split("<br>").collect();
    // let element: String = [parts[0],"<div><p><a href=\"",request[1],"\">HTML Images</a></p></div>", parts[1]].join("");
    // file.write_all(&element.as_bytes()).unwrap();
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
    println!("req flushed");
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
