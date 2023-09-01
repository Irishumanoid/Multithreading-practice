use std::{
    iter,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use hello::ThreadPool;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();


    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}


use messaging_thread_pool::{*, samples::*};


pub fn test_messaging_threads_pool(listener: TcpListener) {
    let pool = messaging_thread_pool::ThreadPool::<Randoms>::new(4);
    for stream in listener.incoming().take(4) {
        let stream = stream.unwrap();
        let stream_clone = stream.try_clone().expect("stream cloning failed");
        iter::once(move || {
            let cloned_stream = stream_clone.try_clone().expect("stream cloning failed");
            move || {
                handle_connection(cloned_stream);
            }
        });

        pool.send_and_receive((0..4usize).map(|i| RandomsAddRequest(i)))
            .expect("thread pool to be available")
            .for_each(|response: AddResponse| assert!(response.result().is_ok()));
    }
    }


fn test_threads_pool(stream: TcpStream) {
    let pool = threads_pool::ThreadPool::new(4);
    
    pool.execute(|| {
        handle_connection(stream);
    });
}


