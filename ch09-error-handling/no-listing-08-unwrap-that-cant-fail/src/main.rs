fn main() {
    // ANCHOR: here
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()  // K_22713 Parses this string slice into another type.
        .expect("Hardcoded IP address should be valid");
    // ANCHOR_END: here
    println!("{:?}",home)
}
