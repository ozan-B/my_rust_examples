use crate::{
    common_ports::MOST_COMMON_PORTS_100,
    model::{Port, Subdomain},
};
use rayon::prelude::*;
use std::net::{SocketAddr, ToSocketAddrs};
use std::{net::TcpStream, time::Duration};

pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    if socket_addresses.is_empty() {
        return subdomain;
    }

    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_par_iter()
        .map(|port| scan_port(socket_addresses[0], *port))
        .filter(|port| port.is_open) // filter closed ports
        .collect();
    subdomain
}

fn scan_port(mut socket_address: SocketAddr, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    socket_address.set_port(port);

    let is_open = TcpStream::connect_timeout(&socket_address, timeout).is_ok();

    Port { port, is_open }
}


/*
Bu kod, belirli bir altdomain (subdomain) için yaygın olarak kullanılan 100 portu 
tarayarak hangi portların açık olduğunu belirler. scan_ports fonksiyonu, altdomain 
üzerindeki açık portları tespit eder ve sonuçları Subdomain yapısında döner. İşte kodun 
detaylı açıklaması:
*/


// KULLANILAN KÜTÜPHANELER

/*
MOST_COMMON_PORTS_100: En yaygın 100 port numarasını içeren sabit liste (common_ports modülünden gelir).
Port ve Subdomain: Port ve altdomain yapılarını tanımlayan model modülünden gelen yapılar.
rayon::prelude::*: rayon kütüphanesi, paralel iterasyon işlemleri için kullanılır.
std::net::{SocketAddr, ToSocketAddrs}: Socket adresleri ve dönüştürme işlemleri için kullanılır.
TcpStream: TCP bağlantısı kurmak için kullanılır.
Duration: Zaman aşımı süresini ayarlamak için kullanılır.
*/




// 2. scan_ports Fonksiyonu     
/*
scan_ports: Verilen altdomain üzerinde port taraması yapar.
subdomain: Tarama yapılacak altdomain.
format!("{}:1024", subdomain.domain): Altdomain adı ile birlikte geçici bir port numarası (1024) kullanarak socket adresi formatı oluşturur.
to_socket_addrs(): Domain adını socket adresine dönüştürür.
expect("port scanner: Creating socket address"): Socket adresi oluşturma başarısız olursa hata mesajı verir.
collect(): Dönüştürülen socket adreslerini bir Vec<SocketAddr> olarak toplar.
*/



//3. Socket Adreslerinin Kontrolü ve Port Taraması
/*
if socket_addresses.is_empty(): Eğer socket adresi oluşturulamazsa (boşsa), tarama yapılmadan altdomain döner.
MOST_COMMON_PORTS_100.into_par_iter(): En yaygın 100 portu paralel iterasyona tabi tutar (rayon kullanarak).
map(|port| scan_port(socket_addresses[0], *port)): Her port için scan_port fonksiyonunu çağırır ve portun açık olup olmadığını kontrol eder.
filter(|port| port.is_open): Sadece açık portları filtreler.
collect(): Açık portları Vec<Port> olarak toplar ve subdomain.open_ports'a atar.

*/

// 4. scan_port Fonksiyonu
/*
scan_port: Belirli bir portun açık olup olmadığını kontrol eder.
mut socket_address: SocketAddr: Taranacak socket adresi (altdomain ve port).
port: u16: Tarama yapılacak port numarası.
timeout = Duration::from_secs(3): Port tarama işlemi için 3 saniyelik zaman aşımı süresi.
socket_address.set_port(port): Socket adresinin portunu ayarlar.
TcpStream::connect_timeout(&socket_address, timeout).is_ok(): Verilen socket adresine belirtilen zaman aşımında TCP bağlantısı kurar ve başarılı olup olmadığını kontrol eder.
Port { port, is_open }: Port numarasını ve bağlantının açık olup olmadığını içeren Port yapısını döner.

*/