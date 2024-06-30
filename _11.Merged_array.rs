//Merge two sorted arrays in Rust



fn merge_sorted(arr1 : &[i32] , arr2 :&[i32])->Vec<i32>{
    
    let mut nums = Vec::with_capacity(arr1.len()+arr2.len());
    
    let mut i = 0;
    let mut j=0;
    while i<arr1.len() && j<arr2.len(){
        if arr1[i]<arr2[j]{
            nums.push(arr1[i]);
            i+=1;
        }
        else{
            nums.push(arr2[j]);
            j+=1;
        }
    }
    
    nums.extend_from_slice(&arr1[i..]);
    nums.extend_from_slice(&arr2[j..]);
    nums;
}
