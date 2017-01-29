
use std::io::*;
use std::net::IpAddr;

#[derive(Debug)]
struct Profile {
    public_dns_list: Vec<IpAddr>,
    spec_dns_list: Vec<IpAddr>,
    spec_hosts: Vec<String>,
}

impl Profile {
    pub fn new() -> Profile {
        let profile = Profile {
            public_dns_list: Vec::new(),
            spec_dns_list: Vec::new(),
            spec_hosts: Vec::new(),
        };

        profile
    }

    pub fn read_pub_dns_list<T: Read>(&mut self, r: T) {

        let rdr = BufReader::new(r);

        self.public_dns_list.clear();
        for line in rdr.lines() {
            match line.unwrap().parse::<IpAddr>() {
                Ok(addr) => self.public_dns_list.push(addr),
                _ => continue,
            }
        }
    }

    pub fn read_spec_dns_list<T: Read>(&mut self, r: T) {

        let rdr = BufReader::new(r);

        self.spec_dns_list.clear();
        for line in rdr.lines() {
            match line.unwrap().parse::<IpAddr>() {
                Ok(addr) => self.spec_dns_list.push(addr),
                _ => continue,
            }
        }
    }

    pub fn read_spec_host_list<T: Read>(&mut self, r: T) {

        let rdr = BufReader::new(r);

        self.spec_hosts.clear();
        for line in rdr.lines() {
            self.spec_hosts.push(line.unwrap());
        }
    }

    pub fn write<T: Write>(&self, w: T) -> Result<()> {

        let mut wtr = BufWriter::new(w);

        writeln!(wtr,
                 "trust-anchor=.,19036,8,2,\
                  49AAC11D7B6F6446702E54A1607371607A1A41855200FD2CE1CDDE32F24E8FB5")?;
        writeln!(wtr, "dnssec")?;
        writeln!(wtr, "dnssec-check-unsigned")?;
        writeln!(wtr, "dnssec-no-timecheck")?;
        writeln!(wtr, "no-resolv")?;
        writeln!(wtr, "no-poll")?;

        // public dns server list
        for dns in &self.public_dns_list {
            writeln!(wtr, "server={}", dns)?;
        }

        // specificed hosts
        for host in &self.spec_hosts {
            for dns in &self.spec_dns_list {
                writeln!(wtr, "server=/{}/{}", host, dns)?;
            }
        }

        writeln!(wtr, "listen-address=::1, 127.0.0.1")?;
        writeln!(wtr, "cache-size=102400")?;
        writeln!(wtr, "local-ttl=3600")?;
        writeln!(wtr, "min-cache-ttl=3600")?;

        Ok(())
    }
}

fn main() {
    let mut profile = Profile::new();

    let pub_dns = b"8.8.8.8\n8.8.4.5\n";
    let spec_dns = b"114.114.114.114\n115.115.115.115\n";
    let spec_hosts = b"bilibili.com\nqq.com\n";

    profile.read_pub_dns_list(&pub_dns[..]);
    profile.read_spec_dns_list(&spec_dns[..]);
    profile.read_spec_host_list(&spec_hosts[..]);
    profile.write(stdout()).unwrap();
}
