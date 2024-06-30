//Given a sorted array of integers, implement a function that returns the median of the array.
fn find_median(arr:&[i32])->Option<f64>{
    
    let len = arr.len();
    
    if(len==0){
        return None;
    }
    
    let mid = len/2;
    if len%2==0 {
        let median = (arr[mid-1] as f64 + arr[mid] as f64)/2;
        return Some(median);
    }
    else{
        return Some(arr[mid] as f64);
    }
}
