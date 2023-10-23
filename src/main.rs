use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
extern "system" {
	// i8 es la c_char = 0-255
	// SOCKADDR = {
	//     sa_family: u16
	//     sa_data: [i8; 14] -- IP: 123.567.9ab.cde -> 14
	// }
	fn getnameinfo ( psockaddr : *const SOCKADDR ,
					 sockaddrlength : i32 ,
					 pnodebuffer : i8 ,
					 nodebuffersize : u32 ,
					 pservicebuffer : i8 ,
					 servicebuffersize : u32 ,
					 flags : i32 ) -> i32 )
}
fn reverse_dns_lookup(ip_addr: &str) -> Result<String,String> {
	let ip: IpAddr = match FromStr::from_str(ip_addr) {
		Ok(r) => r,
		Err(_) => return Err("Direccion ip no valida".to_string())
	};
	match ip {
		IpAddr::V4(ipv4) => {
			match ipv4 {
				Ipv4Addr::UNSPECIFIED => return Err("Direccion IP no valida".to_string()),
				_ => {
					match dns_lookup::lookup_addr(&ip) {
						Ok(hostnames) => {
							if hostnames.is_empty() {
								return Ok("No se encontro un nombre de host adecuado".to_string());
							} else {
								return Ok(format!("Nombre del host: {}", hostnames))
							}

						}
						Err(_) => return Err("Error al buscar la busqueda inversa del DNS".to_string())
					}
				}
			}
		}
		IpAddr::V6(_) => {
			return Err("La busqueda del DNS no es compatible con Ipv6".to_string())
		}
	}
}

fn main() {
	let arp_output = std::process::Command::new("arp")
		.arg("-a")
		.output()
		.expect("Error al ejecutar el comando arp");
	let arp_output_str = String::from_utf8_lossy(&arp_output.stdout);
	let arp_lines: Vec<&str> = arp_output_str.lines().collect();

	for line in arp_lines.iter().skip(3) {
		let parts: Vec<&str> = line.split_whitespace().collect();
		if parts.len() >= 2 {
			let ip = parts[0];
			let mac = parts[1];
			if let Ok(name) = reverse_dns_lookup(ip) {
				println!("Direccion IP: {}, Direccion MAC: {}, Nombre del host: {}", ip, mac, name)
			} else {
				println!("Direccion IP: {}, Direccion MAC: {}, Nombre del host: {}", ip, mac, "No se encontro")
			}
		}
	}
}
