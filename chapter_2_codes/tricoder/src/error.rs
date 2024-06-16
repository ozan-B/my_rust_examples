use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: tricoder <kerkour.com>")]
    CliUsage,
    #[error("Reqwest: {0}")]
    Reqwest(String),
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}

/*use thiserror::Error;
thiserror::Error: thiserror kütüphanesinden Error derive makrosunu kullanırız. 
Bu makro, hata türlerimizi tanımlamamıza ve biçimlendirmemize yardımcı olur.
*/


//Error Enum'ı

/*#[derive(Error, Debug, Clone)]: Error, Debug ve Clone traitlerini otomatik olarak implement eder. 
Error derive makrosu, bu enumın bir hata türü olarak kullanılmasını sağlar. */

/*pub enum Error: Bu, bir hata türü olarak kullanılacak bir enum tanımıdır.*/

/*CliUsage: Kullanıcıya komut satırı kullanım hatası ile ilgili bir mesaj veren bir hata türüdür.
 Hata mesajı "Usage: tricoder <kerkour.com>" olarak belirlenmiştir.
*/




//From Trait'inin Implementasyonu

/*impl std::convert::From<reqwest::Error> for Error: Bu blok, reqwest::Error türünden 
Error türüne bir dönüşüm (conversion) sağlar. Bu, From trait'inin bir implementasyonudur. */


/*fn from(err: reqwest::Error) -> Self: reqwest::Error tipinde bir hata aldığında, 
bu fonksiyon çalışır ve bu hatayı Error::Reqwest tipine dönüştürür. */

/*Error::Reqwest(err.to_string()): reqwest::Error nesnesini Stringe dönüştürür ve 
Error::Reqwest türünde bir hata döner.
*/




//Kısaca Ne Yapar?

/*Bu kod, iki farklı türde hata tanımlar (CliUsage ve Reqwest). 
Ayrıca, reqwest kütüphanesinden gelen hataların otomatik olarak Error 
türüne dönüştürülmesini sağlar. Bu sayede, reqwest hataları Error türüne sarılır ve 
daha homojen bir hata yönetimi sağlar.
*/

/*Bu kod, iki farklı türde hata tanımlar (CliUsage ve Reqwest). 
Ayrıca, reqwest kütüphanesinden gelen hataların otomatik olarak Error 
türüne dönüştürülmesini sağlar. Bu sayede, reqwest hataları Error türüne sarılır ve
daha homojen bir hata yönetimi sağlar.
*/