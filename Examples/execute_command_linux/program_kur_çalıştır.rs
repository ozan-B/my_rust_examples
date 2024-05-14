use std::process::Command;
use std::thread;
use std::time::Duration;

//github reposunu indirir ve içindeki python dosyasını çalıştırır
fn main() {

    // 3 saniye bekleyin
    let duration = Duration::from_secs(3);
    


    // Git reposunu klonla
    let output = Command::new("sh")
        .arg("-c")
        .arg("git clone https://github.com/ozan-B/find_file_true.git")
        .output()
        .expect("Failed to execute command");

    thread::sleep(duration);
    
    // Python scriptini çalıştır
    let output_py = Command::new("python3")
        .arg("find_file_true/true_file.py") // Python scriptinin mutlak yolu
        .output()
        .expect("Failed to run");

    if output_py.status.success() {
        println!("Çalıştırma işlemi başarılı");
    } else {
        println!("Çalıştırma işlemi başarısız");
    }
}


