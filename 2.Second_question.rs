//Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.


fn find_first(arr:&[i32],target:i32)->Option<usize>{
    
    let mut low =0;
    let mut high = arr.len();
    
    while low<high{
        let  mid = (high+low)/2;
        
        if arr[mid]==target {
            return Some(mid);
            
        }
        else if arr[mid] <target{
            low = mid +1;
        }else{
            high = mid-1;
        }
    }
    None
}
