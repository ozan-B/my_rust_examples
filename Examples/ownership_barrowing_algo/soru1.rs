//İki vektörün içeriğini birleştirme daha sonra bu içeriği küçükten büyüğe sırala .

fn main(){

    let mut vektor1:Vec<i32> = vec![1,2,3,4,5];
    let  vektor2:Vec<i32> = vec![-5,31,30,40];

    for &item in &vektor2{
        vektor1.push(item);
    }
    println!("birleşmiş vektör:{:?}",vektor1);


    quicksort(&mut vektor1);
    println!("Sıralı vektör:{:?}",vektor1);

}

fn quicksort(arr:&mut[i32]){

    let length=arr.len();
    if length<2{
        return;
    }

    let pivot_index=partition(arr);

    quicksort(&mut arr[0..pivot_index]);
    quicksort(&mut arr[pivot_index+1..]);



}

fn partition(arr:&mut[i32])-> usize{

    let length=arr.len();
    let pivot_index=length/2;
    arr.swap(pivot_index, length-1);

    let mut i =0;

    for j in 0..length-1{
        if arr[j] <= arr[length-1]{
            arr.swap(i, j);
            i+=1;
        }
    }
    arr.swap(i, length-1);
    i

}