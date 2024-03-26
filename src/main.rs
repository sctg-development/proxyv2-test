// Copyright (c) 2024 Ronan LE MEILLAT
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use bytes::BytesMut;

use proxy_protocol::version2::ProxyAddresses;
use proxy_protocol::ProxyHeader;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:21122").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            println!("Accepted a connection");
            let mut buf_bytes = BytesMut::with_capacity(1024);
            let original_peer_addr = match socket.peer_addr() {
                Ok(addr) => addr.to_string(),
                Err(_) => "unknown".to_string(),
            };
            let mut real_peer_addr:String = original_peer_addr.clone();
            loop {
                match socket.read_buf(&mut buf_bytes).await {
                    Ok(0) => {
                        // socket closed
                        return;
                    }
                    Ok(n) => {

                        println!("Received a real connection from {}", original_peer_addr);
                        if let Ok(header) = proxy_protocol::parse(&mut buf_bytes) {
                            match header {
                                ProxyHeader::Version2 {
                                    command,
                                    transport_protocol,
                                    addresses,
                                } => {
                                    println!(
                                        "Received a proxyied connection with a HAProxy V2 header"
                                    );
                                    println!("Command: {:?}", command);
                                    println!("Transport Protocol: {:?}", transport_protocol);
                                    println!("Addresses: {:?}", addresses);
                                    match addresses {
                                        ProxyAddresses::Ipv4 { source, .. } => {
                                            real_peer_addr = source.to_string();
                                        }
                                        ProxyAddresses::Ipv6 { source, .. } => {
                                            real_peer_addr = source.to_string();
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        // Now we have the original peer address
                        println!("Apparent peer address: {}, Real peer address: {}", original_peer_addr, real_peer_addr);

                        // Echo everything received
                        if socket.write_all(buf_bytes.as_ref()).await.is_err() {
                            // Error occurred, stop processing
                            return;
                        }
                        // Clear the buffer
                        buf_bytes.clear();
                    }
                    Err(_) => {
                        // Error occurred, stop processing
                        return;
                    }
                }
            }
        });
    }
}
