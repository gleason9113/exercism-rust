pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut size: u32 = 2;
    let mut pool: u32 = n * size; 
    let mut primes = Vec::new();
    let mut total_primes: u32 = primes.len() as u32;

    while total_primes <= n {
        primes = get_prime(pool);
        total_primes = primes.len() as u32;
        size += 1;
        pool = n * size;
    }
   
    let result = &primes[n as usize];
    return *result; 
       
}


//Find primes using Sieve of Eratosthenes

pub fn get_prime(size: u32) -> Vec<u32> {
    let mut primes = vec![1; size as usize]; //Vector initialized with ones
    let mut result = Vec::new();
    for i in 2..size { //Iterate through number list
        if primes[i as usize] == 1 { 
            result.push(i);
            for j in i..size {
                let x: u32 = i.wrapping_mul(j);
                if x < size {
                    primes[x as usize] = 0; //Mark composite numbers
                }
                else { 
                    break;
                }
            }
        }
    }
    return result; //Indices marked with 1 represent prime numbers
}    
    

    
