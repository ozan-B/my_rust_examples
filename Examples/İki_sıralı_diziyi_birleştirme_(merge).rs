/*İki sıralı diziyi birleştirmek için kullanılan "merge" işlemi,
 sıralı iki diziye sahip olduğunuzda, bu dizileri birleştirerek tek bir 
 sıralı dizi oluşturma işlemidir. 
Merge işlemi, sıralı dizileri birleştirirken sıralı düzenini korur. */


/*işte iki sıralı diziyi birleştiren basit bir merge işlemi örneği:

Dizi 1: [2, 4, 6, 8]
Dizi 2: [1, 3, 5, 7]

Birleştirilmiş Dizi: [1, 2, 3, 4, 5, 6, 7, 8]
 */

 fn main() {
    // İlk dizi ve ikinci dizi tanımlanıyor
    let _array: [i32; 5] = [2, 4, 6, 8, 9];
    let _array2: [i32; 4] = [1, 3, 5, 7];

    // İndisler için değişkenler tanımlanıyor
    let (mut i, mut j): (usize, usize) = (0, 0);

    // Sıralı diziyi tutacak olan vektör ve dizilerin uzunlukları tanımlanıyor
    let (mut sorted_arr, len1, len2): (Vec<i32>, usize, usize) = (Vec::new(), _array.len(), _array2.len());

    // İki dizi de tamamlanana kadar devam eden birleştirme işlemi
    while i < len1 && j < len2 {
        // İki dizi arasında karşılaştırma yapılıyor, küçük olan öğe vektöre ekleniyor ve ilgili indis arttırılıyor
        if _array[i] < _array2[j] {
            sorted_arr.push(_array[i]);
            i += 1;
        } else {
            sorted_arr.push(_array2[j]);
            j += 1;
        }
    }

    //iknici dizinin elemanları tükendiyse, ilk dizinin kalan elemanlarını ekle

    while i < len1 {
        sorted_arr.push(_array[i]);
        i += 1;
    }

    // ilk dizinin elemanları tükendiyse, ikinci dizinin kalan elemanlarını ekle
    while j < len2 {
        sorted_arr.push(_array2[j]);
        j += 1;
    }

    // Birleştirilmiş ve sıralanmış vektör yazdırılıyor
    print!("sonuc:{:?}", sorted_arr);
}
