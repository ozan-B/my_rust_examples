//Bir String'i tersine çevirme
//Verilen bir String'i tersine çevirerek 
//yeni bir String oluşturun. Orijinal String'in sahipliği değişmemelidir.



fn main(){
    let kelime="bozkur tozan";
    let ters_kelime=tersine_cevir(&kelime);
    print!("Ters kelime :{:?}",ters_kelime);

}

fn tersine_cevir(s:&str)->String{

    //chars fonksiyonu, bir stringin karakterlerini tek tek döndüren bir iterator döndürür.
    //rev fonksiyonu, bir iteratorün elemanlarını tersine çeviren bir başka iterator döndürür.


    //collect fonksiyonu, bir iteratorün elemanlarını toplu bir şekilde toplayarak belirli bir veri yapısına dönüştürmek için kullanılır. 
    // Örneğin, bir vektör elde etmek için collect fonksiyonunu kullanabiliriz.
    
    
    let  vektor:Vec<char> = s.chars().rev().collect();
    let mut ters_kelime =String::new();
    
    for i in vektor{

        ters_kelime.push(i);
        //ters_kelime.push(i);

    }     
    ters_kelime
 
}