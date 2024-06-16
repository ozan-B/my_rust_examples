use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    };

const SHA1_HEX_STRING_LENGTH :usize= 40;

fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();  /*env::args() fonksiyonu, programın komut satırı argümanlarını içeren bir iterator döndürür. Bu iterator'ü collect() fonksiyonuyla kullanarak, bu argümanları bir vektör haline getirebiliriz. */

    if args.len() != 3{ /*kullanıcı 3 argüman girmediyse if claus'a gir */
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>"); /*programın doğru kullanımını açıkla */
        return Ok(());
    /*return Ok(()) ifadesi, Rust'ta bir işlevin başarıyla sona erdiğini belirtir
    
     Ancak, bu işlevin herhangi bir değer döndürmediğini belirtmek için () kullanılır. Yani, Ok(()) 
     ifadesi, işlevin başarıyla tamamlandığını, ancak bir değer döndürmediğini belirtir. */    
    }


    let hash_to_crack = args[2].trim(); /*trim argümanı gereksiz boşluklardan temizlemek için kullanılır */  
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH{

            return Err("sha1 hash is not valid".into())
    /*into(): Bu, bir tür dönüşüm işlemidir. String bir ifadeyi Error türüne dönüştürmek için kullanılır. 
    Rust'ta, genellikle into() metodları tür dönüşümlerini gerçekleştirmek için kullanılır. */
    }

    let wordlist_file = File::open(&args[1])?;
    /*? operatörü: Eğer dosya başarılı bir şekilde açılırsa, 
    program normal şekilde devam eder. Ancak, dosya açılamazsa
    program bu hatayı işler. ? operatörü, hata durumunu işleyerek programın hata 
    olmadan devam etmesini sağlar. Eğer File::open()
    başarısız olursa, program derhal bu bloğun dışına çıkar ve Err durumunu döndürür. 
    
     
     Özetle, ? operatörü, hata durumlarını kısa ve anlaşılır bir şekilde işlemek için kullanılır 
     ve genellikle Rust'ta hata işleme sürecini kolaylaştırmak için tercih edilir.*/


    let reader=BufReader::new(&wordlist_file); /* Bu dosyanın içeriği BufReader ile okunacaktır. */

    /*BufReader, bir giriş akışını tamponlayarak daha verimli okuma işlemleri yapmamızı sağlar. 
    Yani, bu yapı, bir giriş akışını bellekte 
    bir tampon(BUFFER) üzerinde tutar ve daha verimli okuma işlemleri gerçekleştirmemize olanak tanır.
     */    
    
    for line in reader.lines(){
        let line = line?;
        let common_password = line.trim();
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())){
            println!("Password found: {}",&common_password);
            return Ok(());
        }
    }

    println!("Password not found in wordlist :(");
    Ok(())

    /*common_password.as_bytes() -> common_password değişkenindeki stringi   byte dizisine dönüştürür. Bu, SHA-1 hash fonksiyonunun kullanabilmesi için gereklidir çünkü hash fonksiyonları byte dizileri üzerinde çalışır. */

    /*sha1::Sha1::digest(common_password.as_bytes()) -> common_password'ın byte dizisi üzerinde SHA-1 hash fonksiyonunu çalıştırır ve bu parolanın SHA-1 hash'ini hesaplar. */

    /*hex::encode(...) -> SHA-1 hash'inin byte dizisini alır ve bunu hexadecimal (onaltılık) bir dizgeye dönüştürür. Hash'ler genellikle hexadecimal formatında temsil edilir. */

    /*&hex::encode(...) -> Bu dönüşüm sonucunda elde edilen hexadecimal dizgeye referans oluşturur. */

    /*Kullanıcı tarafından sağlanan hash (hash_to_crack) ile hesaplanan hash'i karşılaştırır.
     Eğer iki hash eşitse, bu, common_password parolasının,
     kullanıcı tarafından verilen hash'in orijinal parolası olduğu anlamına gelir. */

    /*lines() metodu, BufReader nesnesi üzerinde çağrılan bir metottur ve bu metot, 
    bir satırı okur ve dize içerisinde bir sonraki yeni satıra kadar olan karakterleri döndürür. 
    Bu metot, bir satırın sonuna kadar olan karakterleri bir Result türünden döndürür. 
    Eğer bir satır başarıyla okunursa, Ok durumu 
    ile satırın içeriği döndürülür. Eğer bir hata meydana gelirse, Err durumu ile hata döndürülür. */



}