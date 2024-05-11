fn main() {
    let array:[i8;16] = [1,5,6,7,9,0,15,-25,4,-45,8,-9,12,98,-8,-1];

    array_kucuk_buyuk(&array)

}

fn array_kucuk_buyuk(arr:&[i8]){

    let counter =  arr.len() ;
    let mut sum=0;
    let mut _i=0;
    
    
    for _i in 0..counter{
        sum += arr[_i];
    }
    
    let ort:f64 = sum as f64 / counter as f64;

    println!("Dizi elemanlarının toplam değeri :{}", sum);
    
    println!("Dizi elemanlarının ortalaması  :{:.2}", ort);

}