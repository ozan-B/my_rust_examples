fn main() {
    let array:[i8;16] = [1,5,6,7,9,0,15,-25,4,-45,8,-9,12,98,-8,-1];

    array_kucuk_buyuk(&array)

}

fn array_kucuk_buyuk(arr:&[i8]){

    let counter = arr.len();
    let mut buyuk=arr[0];
    let mut _i=0;
    let mut kucuk=arr[1];
    
    for _i in 0..counter{
        if arr[_i] > buyuk{
            buyuk = arr[_i];
        }if arr[_i]<= kucuk {
            kucuk =arr[_i];
        }
    }

    println!("en büyük değer :{}", buyuk);
    
    println!("en küçük değer :{}", kucuk);

}