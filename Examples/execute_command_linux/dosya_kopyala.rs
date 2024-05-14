use std::process::Command;

//txt dosyasını denemecopy.txt adıyla /home/boewolf dizinine kopyalar
fn main() {
    // ls komutunu çalıştır ve çıktısını al
    let output = Command::new("cp")
        .arg("/home/boewolf/deneme.txt")
        .arg("/home/boewolf/denemecopy.txt")
        .output()
        .expect("Failed to execute command");





    if output.status.success(){
        println!("İşlem başarılı");
    }else{
        println!("İşlem başarısız");
    }
}
