
use std::net::{Ipv4Addr, SocketAddrV4};

#[allow(dead_code)]
pub fn parse_host(input: &str) -> Result<SocketAddrV4, ()> {
    // TODO documentation. Converts a string into a SocketAddrV4.
    // The string is of the format IP:HOST (e.g. 1.2.3.4:22)

    let tokens : Vec<&str> = input.split(":").collect();

    if tokens.len() > 2 {
        return Err(());
    }

    let ip = try!(parse_ip(tokens[0]));

    if tokens.len() == 1 {
        Ok(SocketAddrV4::new(ip, 0))
    }
    else {
        let port_result = tokens[1].parse::<u16>();
        if port_result.is_err() {
            return Err(());
        }
        Ok(SocketAddrV4::new(ip, port_result.unwrap()))
    }
}

#[allow(dead_code)]
pub fn parse_ip(input: &str) -> Result<Ipv4Addr, ()> {
    // TODO Documentation. Parse an IP address string into an IPv4Addr. 
    //Same as the C inet_aton function.

    let tokens: Vec<&str> = input.split(".").collect();
    let mut octets : Vec<u8> = vec![];

    for token in tokens {

        let numval_result = token.parse::<u8>();
        if numval_result.is_err() {
            return Err(());
        }
        octets.push(numval_result.unwrap());
    }

    if octets.len() > 4 {
        return Err(());
    }
    else if octets.len() == 1 {
        octets.insert(0, 0);
    }

    while octets.len() != 4 {
        octets.insert(1, 0);
    }

    Ok(Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]))
}

// Tests

#[cfg(test)]
mod test {
    
    use std::net::{Ipv4Addr, SocketAddrV4};
    use netparse::{parse_ip, parse_host};

    fn construct_expected(ok: bool,
                          a: u8,
                          b: u8,
                          c: u8,
                          d: u8,
                          port: u16) -> Result<SocketAddrV4, ()> {

        if ok {
            Ok(SocketAddrV4::new(
                    Ipv4Addr::new(a, b, c, d),
                    port))
        }
        else {
            Err(())
        }
    }

    // Tests for IP parsing

    #[test]
    fn test_ip_parse_basic() {
        assert_eq!(parse_ip("1.2.3.4"), Ok(Ipv4Addr::new(1, 2, 3, 4)));
    }

    #[test]
    fn test_ip_parse_fails() {
        assert_eq!(parse_ip("1..3.4"), Err(()));
    }

    #[test]
    fn test_ip_parse_single_octet() {
        assert_eq!(parse_ip("1"), Ok(Ipv4Addr::new(0, 0, 0, 1)));
    }

    #[test]
    fn test_ip_parse_two_octets() {
        assert_eq!(parse_ip("1.2"), Ok(Ipv4Addr::new(1, 0, 0, 2)));
    }

    #[test]
    fn test_ip_parse_three_octets() {
        assert_eq!(parse_ip("1.2.3"), Ok(Ipv4Addr::new(1, 0, 2, 3)));
    }

    // Tests for Host Parsing (IP:Port)

    #[test]
    fn test_host_parse_basic() {
        let expect = construct_expected(true, 1, 2, 3, 4, 5678);
        assert_eq!(parse_host("1.2.3.4:5678"), expect);
    }

    #[test]
    fn test_host_parse_no_port_and_colon() {
        let expect = construct_expected(true, 1, 2, 3, 4, 0);
        assert_eq!(parse_host("1.2.3.4"), expect);
    }

    #[test]
    fn test_host_parse_bad_ip() {
        let expect = construct_expected(false, 0, 0, 0, 0, 0);
        assert_eq!(parse_host("asdf:5678"), expect);
    }

    #[test]
    fn test_host_parse_bad_ip_overflow() {
        let expect = construct_expected(false, 0, 0, 0, 0, 0);
        assert_eq!(parse_host("365.1.2.3:5678"), expect);
    }

    #[test]
    fn test_host_parse_bad_port() {
        let expect = construct_expected(false, 0, 0, 0, 0, 0);
        assert_eq!(parse_host("365.1.2.3:asdf"), expect);
    }

    #[test]
    fn test_host_parse_bad_port_overflow() {
        let expect = construct_expected(false, 0, 0, 0, 0, 0);
        assert_eq!(parse_host("365.1.2.3:88888"), expect);
    }
}
