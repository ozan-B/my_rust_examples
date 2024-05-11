fn main() {
    let  array : [i32;8] = [1,2,3,4,5,6,8,9];
    
    fonk(&array);
}

fn fonk(arr:&[i32]){

    let mut counter: usize = arr.len();
    let counter2: usize = arr.len();

    let mut array2: Vec<i32> = vec![0; counter]; // Tüm elemanları 0 olan bir vektör oluşturur

    let mut _i=0;

    while  counter>0 {

        while _i<counter2{
        
            array2[_i]=arr[counter-1];// Son elemanın indeksi: boyut - 1

            break;
        }
        _i =_i+1;
        counter=counter-1;
    }

    print!("yeni dizin :{:?}",array2)
}