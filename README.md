# Rust Proxy IP Detection

This Rust application is designed to handle connections that may be proxied and correctly identify the original source IP address, even when the connection is proxied.

## How it works

The application listens for incoming connections and reads the data sent over each connection. It uses the `proxy_protocol` library to parse the data and extract the Proxy Protocol header, if one is present.

The Proxy Protocol header can be either version 1 or version 2. This application specifically handles version 2 headers, which include the command, transport protocol, and addresses (source and destination).

The addresses can be IPv4 or IPv6. The application extracts the source address, which represents the original IP address of the client before it was proxied.

After extracting the original source IP address, the application echoes back all the data it received. It then clears the buffer to ensure that data from previous connections does not interfere with the next connection.

## Why it's useful

This application is useful in scenarios where you want to know the original IP address of a client, but the connection may be proxied. By correctly parsing the Proxy Protocol header, the application can extract the original source IP address, even when the connection is proxied.

This can be useful for logging, analytics, or any other situation where you need to know the original source IP address of a connection.

## Using with HAProxy

This application includes a HAProxy configuration file that is set up to use the Proxy Protocol. You can use this configuration file to test the application with HAProxy.

To use the provided HAProxy configuration file, run the following command:

```bash
haproxy -d -f ./haproxy.cfg