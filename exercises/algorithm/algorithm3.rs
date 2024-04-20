/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort1<T: PartialOrd + Clone>(array: &mut [T]) {
    for i in 0..array.len() {
        for j in 0..array.len() - i {
            if j + 1 >= array.len() - i {
                continue;
            } 
            if &array[j] > &array[j + 1] { 
                let tmp = (&array[j + 1]).clone();
                array[j + 1] = (&array[j]).clone();
                array[j] = tmp;
            }
        }
    }
}
fn sort<T: PartialOrd + Clone>(array: &mut [T]) {
    for i in 1..array.len() {
        let tmp = (&array[i]).clone();
        let mut index :usize=0;
        for j in 0..=i -1{
                index = i-1-j;
            if &array[index] > &tmp { 
                array[index + 1] = (&array[index]).clone();
            }else{
                index=index+1;
                break;
            }
        }
        array[index] = tmp;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_0() {
        let mut vec = vec![9, 81];
        sort(&mut vec);
        println!("{:?}", vec);
    }
    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
