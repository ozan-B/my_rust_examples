use std::process::Command;

/*std::process::Output, şu alanlardan oluşur:

status: Harici komutun çıkış durumunu temsil eder. ExitStatus türündedir.
stdout: Harici komutun standart çıkışını (stdout) temsil eder. Vec<u8> türündedir.
stderr: Harici komutun standart hata çıkışını (stderr) temsil eder. Vec<u8> türündedir. */

//txt dosyasını okur daha sonra dosyayı siler .
fn main() {
    // cat komutunu çalıştır ve çıktısını al
    let mut output = Command::new("cat")
        .arg("/home/boewolf/deneme.txt")
        .output()
        .expect("Failed to execute command");

    // Çıktıyı konsola yazdır
   
    println!("içerik :{}", String::from_utf8_lossy(&output.stdout));
   
    // rm komutunu çalıştır ve çıktısını al
    output = Command::new("rm")
        .arg("/home/boewolf/deneme.txt")
        .output()
        .expect("Failed to execute command");

    if output.status.success(){
        println!("İşlem başarılı");
    }else{
        println!("İşlem başarısız");
    }
}
