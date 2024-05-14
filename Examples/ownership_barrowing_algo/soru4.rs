//3x3 İki matrisin toplamını bulma




fn main(){

    let matris_1 :[[i32;3];3] = [

    [1,2,3],
    [4,5,6],
    [7,8,9],

    ];

    let matris_2 :[[i32;3];3] = [

    [1,2,3],
    [4,5,6],
    [7,8,9],

    ];
    
    println!("ilk matris ->{:?}",matris_1);
    println!("");
    println!("ikinci matris ->{:?}",matris_2);
    println!("");

    //matrislerin toplamının sonucu
    let sonuc =matris_toplam(&matris_1, &matris_2);

    

    println!("sonuç ->{:?}",sonuc);

}

fn matris_toplam(m1:&[[i32;3];3],m2:&[[i32;3];3]) -> [[i32;3];3] {

    let mut toplam_matris :[[i32;3];3]=[[0;3];3];

    for i in 0..3{
        for j in 0..3{
            toplam_matris[i][j] = m1[i][j] + m2[i][j];

        }
    }
    toplam_matris
}