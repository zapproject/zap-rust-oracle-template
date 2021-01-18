# zap-rust-oracle-template
A template for create a Oracle on the Zap platform written in Rust

Web3 Server

Usage
    Configuration:
        Add a config entry for each event being watched
        create abi file for abi_path
        examples:
            {
            "name":"FulfillQuery",
            "event_hash":"d78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822",
            "address":"0d4a11d5eeaac28ec3f61d100daf4d40471f1852",
            "abi_path":"FulfillQuery.abi",
            "response_data":"http://localhost:3007",
            "response_type:"http_post"
            }
            {
            "name":"Incoming",
            "event_hash":"3a1b1d57ba9424f0feefcaa9c18f21c62f171592092617c442a3059181b9e32c",
            "address":"9c0b906ec58c44938ae4836d2afe17962d421347",
            "abi_path":"eventsABI/Incoming.abi",
            "response_type":"web_socket",
            "response_data":"none",
            }
        create a env file with the following keys
            ## defines websocket etheruem endpoint. can be any web3 provider
            INFURA=""
            ## defines 
            PORT=""

        Build with docker:
           from project run: docker build -t rust-web3 -f ./Dockerfile .
        Build with cargo:
            cargo build --release
    Running:
        TCP server must first be running for the service. 
        To start test server use npm run start-websocket in testserver folder.

Running with docker :
    docker run -it --net=host rust-web3
Running with Cargo:
    cargo run --release