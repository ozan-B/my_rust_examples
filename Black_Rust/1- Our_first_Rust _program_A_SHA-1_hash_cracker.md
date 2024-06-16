# Hadi ilk Rust programımızı yazalım. 

Bu kitaptaki kod örneklerinin tamamını, ekteki Git dosyasında bulabilirsiniz.
depo: https://github.com/skerkour/black-hat-rust

![r7](/img_rust/r7.png)

sha1_cracker klasöründe yeni bir proje oluşturacaktır.

![r8](/img_rust/r8.png)

- SHA-1, birçok eski web sitesi tarafından kullanıcıların şifrelerini saklamak için kullanılan bir hash fonksiyonudur.

- İşte bu noktada bir **“hash kırıcı”** kullanışlıdır. Bir hash kırıcı, çok sayıda Orijinal şifreyi bulmak için farklı hash'ler deneyen bir programdır  .

Bunun çözümü , Örneğin, SHA-1 gibi eski ve güvenliği zayıf bir hash fonksiyonu yerine, **Argon2id** gibi modern ve daha güvenli bir **hash fonksiyonu** tercih edilmelidir. **Argon2id**, brute force saldırılarına karşı daha dayanıklıdır çünkü hesaplama açısından çok daha fazla kaynak gerektirir.

- Rust'ta,
Bu kütüphanelere ya da paketlere **Crates** diyoruz. Bunlara **https://crates.io** adresinden çevrimiçi olarak göz atılabilir.

![r9](/img_rust/r9.png)


---
# Kodun Açıklaması :

### use sha1::Digest;
- Bu satır, **sha1** kütüphanesinden **`Digest`** isimli öğeyi içe aktarır.

### use std::env
- Bu satır, **std** kütüphanesindeki **(Rust'ın standart kütüphanesi)** **`env`** modülünü içe aktarır .
- **env modülü**, çevre değişkenlerine erişim sağlamak ve **komut satırı argümanlarını almak** gibi görevler için kullanılır.

### use std::error::Error;
- Bu satır, std kütüphanesindeki error modülünden Error trait'ini içe aktarır. 

- **Error trait**'i, hata işleme için yaygın olarak kullanılır ve hataların türüne bakılmaksızın ortak bir arayüz sağlar. **Bu, fonksiyonların hata döndürebilmesini ve bu hataların uygun şekilde ele alınabilmesini sağlar.**

### use std::fs::File;

- Bu satır, **std** kütüphanesindeki **fs** modülünden **File struct**'ını içe aktarır. 

- File struct'ı, **dosya işlemleri** **(okuma, yazma, açma, kapama)** için kullanılır.

### use std::io::{BufRead, BufReader};

- **std** kütüphanesindeki **io** modülünden **BufRead** ve **BufReader** öğelerini içe aktarır. 

- **BufRead**, **tamponlu(buffer) okuma işlemleri** için bir **trait**'tir ve **BufReader, bir okuyucuya tamponlama(buffer) ekleyerek** daha verimli okuma işlemleri yapmayı sağlar.


---
**`const SHA1_HEX_STRING_LENGTH: usize = 40;`**

- Bu sabit, SHA-1 hash değerinin **40 karakter uzunluğundaki** hexadecimal temsilini belirtir. SHA1_HEX_STRING_LENGTH sabitinin **usize türünde tanımlanmasının nedeni, dizinleme ve bellek işlemleri gibi durumlarda uygun boyutta olmasını sağlamaktır.**


---
### Giriş ve Geri Dönüş Tipi
**`fn main() -> Result<(), Box<dyn Error>>`**

- **`Result<(), Box<dyn Error>>`** dönüş tipi, fonksiyonun hata durumunda bir Error döndürebileceğini belirtir. `Box<dyn Error>` dinamik olarak büyüklüğü belirlenen bir hata türünü temsil eder. 

---

### Komut Satırı Argümanlarını Toplama
>**`let args: Vec<String> = env::args().collect();`** 

- `env::args()`, Rust'ın standart kütüphanesindeki env modülünden args() fonksiyonunu çağırır. Bu fonksiyon, programın komut satırı argümanlarını içeren bir iterator döndürür.

- `collect()` .collect(): args() fonksiyonunun döndürdüğü iterator'u bir koleksiyona dönüştürmek için kullanılır. Burada collect() fonksiyonu, bir vektör oluşturmak için kullanılır. collect() fonksiyonu, çeşitli veri yapılarını oluşturmak için kullanılabilir, burada `Vec<String>` türünde bir vektör oluşturulur.

- Özetle , **let** anahtar kelimesi, bir değişkenin tanımlandığını belirtir. **args** **isimli değişken** `Vec<String>` **türünde bir vektör olacaktır.** `Vec<String>` türü, içinde **String** türündeki öğeleri barındıran **dinamik boyutlu bir vektörü ifade eder.**

---

### Argüman Sayısını Kontrol Etme

~~~
if args.len() != 3 {
    println!("Usage:");
    println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
    return Ok(());
}
~~~
- Argümanların sayısını kontrol eder. Eğer 3 argüman yoksa **(program adı + 2 argüman)**, kullanım talimatlarını yazdırır ve programdan çıkar.

---

### Hash Değerini Alma ve Uzunluğunu Kontrol Etme

~~~
    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }
~~~

- Üçüncü argüman **(args[2])** olan hash'i alır ve **baştaki/sondaki boşlukları temizler** **`(trim())`**.

- **Hash'in uzunluğunun** SHA1_HEX_STRING_LENGTH **(40)** olup olmadığını kontrol eder. **Eğer değilse, hata döner.**

---

### Kelime Listesi Dosyasını Açma

~~~
    let wordlist_file = File::open(&args[1])?;
~~~

- İkinci argüman **(args[1])** olan kelime listesi dosyasını açar. Eğer dosya açılamazsa, hata döner **(? operatörü)**.

---

### BufReader Kullanarak Dosyayı Okumaya Hazırlanma

~~~
    let reader = BufReader::new(&wordlist_file);

~~~

- **BufReader**, dosya okumayı daha verimli hale getirmek için dosyayı tamponlar.

---

### Satır Satır Kelime Listesini Okuma ve SHA-1 Hash Kontrolü

~~~
    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }

~~~

- **`reader.lines()`** ile dosyayı satır satır okur. Her satır `Result<String, std::io::Error>` döner, **line?** ifadesiyle hata kontrolü yapılır.

- **line** değişkeni içindeki satır başındaki/sonundaki boşlukları temizler ve common_password değişkenine atar.

- common_password için SHA-1 hash hesaplanır `(sha1::Sha1::digest(common_password.as_bytes()))` ve **hex olarak encode edilir.**

- **Hesaplanan hash, kırılmak istenen hash (hash_to_crack) ile karşılaştırılır. Eğer eşleşirse, parolanın bulunduğu yazdırılır ve programdan çıkılır.**

---

### Parola Bulunamazsa

~~~
    Ok(())
}
~~~

- Tüm kelimeler denenmiş ve hiçbiri hash ile eşleşmemişse, Ok(()) döner ve program normal şekilde sona erer.

---
----

