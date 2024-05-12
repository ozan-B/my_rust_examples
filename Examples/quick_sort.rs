/*Quick sort, sıralama algoritmalarından 
biridir ve genellikle hızlı bir şekilde çalışır. 
Temel fikri, bir diziyi bölerek sıralamak ve ardından 
bu bölümleri birleştirmektir.


Quick sort algoritması genellikle aşağıdaki adımları izler:

1-Bölme (Partitioning): Diziden bir eleman seçilir ve bu elemanın solunda küçük, sağında büyük olan elemanlar olacak şekilde diziyi ikiye böleriz. Bu elemana "pivot" denir.

2-Parçaları Sıralama (Sorting Sub-arrays): Pivot elemanın sağında ve solunda kalan alt dizileri (parçaları) tekrar aynı şekilde bölüp sıralarız. Bu işlem rekürsif olarak devam eder.

3-Birleştirme (Merging): Sıralı alt dizileri birleştirerek tam sıralı diziyi elde ederiz.


Her adımda, bir elemanın seçilmesi ve diziyi bölme işlemi yapıldığı 
için algoritmanın hızı, pivot elemanın nasıl seçildiğine ve dizinin ne kadar
 iyi bölündüğüne bağlıdır. İyi bir pivot seçimi 
(örneğin, dizinin ortasındaki eleman gibi) algoritmanın hızını artırabilir.

 */

 /*kodun sözel açıklaması:

1-quick_sort_desc fonksiyonu, bir vektör alır ve sıralanmış bir vektör döndürür.
2-Eğer vektörün uzunluğu 2'den küçükse, vektörü doğrudan döndürürüz. Bu durumda, vektör zaten sıralıdır veya içinde bir eleman vardır.
3-Vektörün orta elemanını pivot olarak seçeriz. Bu, vektörün uzunluğunun yarısı olan indisli elemandır.
4-Vektördeki her elemanı pivot ile karşılaştırırız. Pivot'ten büyük olanlar less vektörüne, pivot'ten küçük olanlar ise greater vektörüne eklenir.
5-less ve greater vektörlerini sıralamak için aynı fonksiyonu rekürsif olarak çağırırız. Ancak bu sefer less vektörü küçükten büyüğe sıralanmış, greater vektörü ise büyükten küçüğe sıralanmış olur.
6-less, pivot ve greater vektörlerini birleştirerek tam sıralanmış bir vektör elde ederiz.
7-Bu sıralanmış vektörü döndürürüz.
 */


 /*fonksiyonları bir değer döndürmek yerine, dizinin içinde yapılan
  değişiklikleri doğrudan kendisi üzerinde gerçekleştirir.
  Bu nedenle, bu fonksiyonlar -> işareti ile bir değer döndürmezler. */


fn quicksort (arr:&mut [i32]){

    let length = arr.len();

    if length<2{
        return;
    } 

    let pivot_index=partition(arr);

    quicksort(&mut arr[0..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..length]);


}

fn partition(arr:&mut [i32]) ->usize{

    let length =arr.len();
    let pivot = length/2;
    arr.swap(pivot,length -1);

    let mut i = 0;
    
    for j in 0..length -1{

        if arr[j] <= arr[length -1]{
            arr.swap(i,j);
            i +=1;
        }
    }
    arr.swap(i, length-1);
    i

}

fn main(){
    let mut arr = [4, 2, 7, 1, 9, 5, 3, 8, 6];
    println!("Before sorting: {:?}", arr);
    quicksort(&mut arr);
    println!("After sorting: {:?}", arr);
}