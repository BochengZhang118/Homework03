fn main() {
 
    let mut list = vec![19, 5, 34, 29, 67, 16, 59, 199];
    println!("Original :{:?}  ", list);
    bubble_sort(&mut list);
    println!("After sorting :{:?}  ", list);

    let mut list = vec!['a','d', 'N','e', 'A', 'Z', 'a', 'W'];
    println!("Original :{:?}  ", list);
    bubble_sort(&mut list);
    println!("After sorting :{:?}  ", list);
 
}
 

 
fn bubble_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) -> &Vec<T> {
    for _i in 0..list.len() {
        for j in 0..list.len() - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
    list
}
