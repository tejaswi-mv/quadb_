//Implement a function that returns the kth smallest element in a given array.

fn kth_smallest(arr: &[i32], k:usize() )->Option<i32>{
  if arr.is_empty() || k==0 || k> arr.len(){
      return None;
  }
    let n = arr.len();
   let mut sorted = arr.to_owned();
   
   sorted.sort();
    
    Some(sorted[k-1])
}
