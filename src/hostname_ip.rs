use std::{env::consts, net::Ipv4Addr, process::Command, str::FromStr};

// 获取本机的局域网 ip 信息
pub fn get_lan_ip() -> Ipv4Addr {
    match consts::OS {
        "windows" => {
            let output = get_output_in("ipconfig");
            let (output, _, _) = encoding_rs::GBK.decode(&output);
            get_windows_lan_ip(&output)
        }
        "linux" => {
            let output = get_output_in("ip");
            let output = String::from_utf8(output).expect("decode utf-8 failed");
            get_linux_lan_ip(&output)
        }
        _ => panic!("This os is not supported!!"),
    }
}

// 获取命令执行的结果
fn get_output_in(name: &str) -> Vec<u8> {
    Command::new(name)
        .arg("address")
        .output()
        .expect(&format!("exec '{}' command error!!", name))
        .stdout
}

// 根据命令内容获取window lan ip
fn get_windows_lan_ip(output: &str) -> Ipv4Addr {
    output
        .lines()
        .map(|l: &str| l.trim_end())
        .filter_map(|l: &str| {
            if l.contains("IPv4 地址") {
                l.find(": ").map(|i| &l[i + 2..])
            } else {
                None
            }
        })
        .find_map(|l: &str| Ipv4Addr::from_str(l).ok())
        .expect("lan ip resolution failed")
}

fn get_linux_lan_ip(output: &str) -> Ipv4Addr {
    output
        .lines()
        .map(|l| l.trim_start())
        .filter_map(|l| l.strip_prefix("inet "))
        .filter_map(|l| l.find(' ').map(|x| &l[0..x]))
        .filter_map(|l| l.find("/").map(|x| &l[0..x]))
        .filter_map(|l| Ipv4Addr::from_str(l).ok())
        .find(|o| !o.is_loopback())
        .unwrap()
}
