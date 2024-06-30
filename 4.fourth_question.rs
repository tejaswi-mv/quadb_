fn is_prime(n :u32)->bool{
    if n<=1{
        return false;
    }
    
    let mut i=2;
    let mut ans = true;
    
    while i*i <=n{
        if n%i{
            ans = false;
            break;
        }
        i +=1;
    }
    
    ans
}
