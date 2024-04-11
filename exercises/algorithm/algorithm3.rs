/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn partition<T>(array: &mut [T], low: isize, high: isize) -> isize
where
    T: Ord + Clone,
{
    let pivot = array[high as usize].clone();
    let mut i = low - 1;
    for j in low..high {
        if array[j as usize] < pivot {
            i += 1;
            array.swap(i as usize, j as usize);
        }
    }
    array.swap((i + 1) as usize, high as usize);
    i + 1
}

fn qsort<T>(array: &mut [T], low: isize, high: isize)
where
    T: Ord + Clone,
{
    if low < high {
        let pivot = partition(array, low, high);
        qsort(array, low, pivot - 1);
        qsort(array, pivot + 1, high);
    }
}

fn sort<T: std::cmp::Ord + Clone>(array: &mut [T]){
    qsort(array, 0, array.len() as isize - 1);
}
#[cfg(test)]
mod tests {
    use super::*;

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