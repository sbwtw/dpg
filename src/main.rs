
extern crate clap;

use std::io::*;
use std::net::IpAddr;
use std::fs::OpenOptions;
use clap::{App, Arg};

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

    // pub fn read_pub_dns_list<T: Read>(&mut self, r: T) {

    //     let rdr = BufReader::new(r);

    //     self.public_dns_list.clear();
    //     for line in rdr.lines() {
    //         match line.unwrap().parse::<IpAddr>() {
    //             Ok(addr) => self.public_dns_list.push(addr),
    //             _ => continue,
    //         }
    //     }
    // }

    // pub fn read_spec_dns_list<T: Read>(&mut self, r: T) {

    //     let rdr = BufReader::new(r);

    //     self.spec_dns_list.clear();
    //     for line in rdr.lines() {
    //         match line.unwrap().parse::<IpAddr>() {
    //             Ok(addr) => self.spec_dns_list.push(addr),
    //             _ => continue,
    //         }
    //     }
    // }

    // pub fn read_spec_host_list<T: Read>(&mut self, r: T) {

    //     let rdr = BufReader::new(r);

    //     self.spec_hosts.clear();
    //     for line in rdr.lines() {
    //         self.spec_hosts.push(line.unwrap());
    //     }
    // }

    pub fn read_rules<T: Read>(&mut self, r: T) {

        let rdr = BufReader::new(r);
        let mut cursor = rdr.lines();

        while let Some(Ok(line)) = cursor.next() {
            if line.is_empty() {
                break;
            }

            if line.bytes().next() == Some(b'#') {
                continue;
            }

            self.public_dns_list.push(line.parse().unwrap());
            // println!("{}", line);
        }


        while let Some(Ok(line)) = cursor.next() {
            if line.is_empty() {
                break;
            }

            if line.bytes().next() == Some(b'#') {
                continue;
            }

            self.spec_dns_list.push(line.parse().unwrap());
        }

        while let Some(Ok(line)) = cursor.next() {
            if line.is_empty() {
                continue;
            }

            if line.bytes().next() == Some(b'#') {
                continue;
            }

            self.spec_hosts.push(line);
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

    let home_dir = std::env::home_dir().unwrap();
    let default_list = format!("{}/.dnsmasq.rules", home_dir.display());

    let matches = App::new("dnsmasq-profile-generater")
        .author("sbw <sbw@sbw.so>")
        .version("0.0.1")
        .about("dnsmasq profile generater")
        .arg(Arg::with_name("output")
            .short("o")
            .takes_value(true)
            .help("specificed output result file")
            .default_value("/etc/dnsmasq.conf"))
        .arg(Arg::with_name("list")
            .short("l")
            .takes_value(true)
            .help("rule list file")
            .default_value(&default_list))
        .get_matches();

    let output = matches.value_of("output").unwrap();
    let rules = matches.value_of("list").unwrap();

    // let pub_dns = b"8.8.8.8\n8.8.4.4\n";
    // let spec_dns = b"114.114.114.114\n115.115.115.115\n";
    // let spec_hosts = b"bilibili.com\nqq.com\n";

    let list_file = match OpenOptions::new().read(true).open(rules) {
        Ok(file) => file,
        Err(e) => panic!("Unable to open {}, {}", rules, e),
    };

    let mut profile = Profile::new();
    // profile.read_pub_dns_list(&pub_dns[..]);
    // profile.read_spec_dns_list(&spec_dns[..]);
    // profile.read_spec_host_list(&spec_hosts[..]);
    profile.read_rules(list_file);

    // write config file
    let config_file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(output) {
        Ok(file) => file,
        Err(e) => panic!("Unable to open {}, {}", output, e),
    };
    profile.write(config_file).unwrap();
}
