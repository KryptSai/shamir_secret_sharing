/*In this scheme, 'n' pieces of a 'secret' is shared between 'n' number of people. Out of these 'n' people atleast if 'k' number of 
people comes together they will get the 'secret'. But k-1 people also can't be able to get the 'secret'.   */

pub use num_bigint;
use num_bigint::{BigInt,BigUint,RandBigInt};
use num_traits::{One, Zero,ToPrimitive};
use rand::{Rng};

fn polynomial_eval(polynomial:Vec<BigInt>,n:i32,prime:BigInt,k:i32)->Vec<BigInt>{
   let mut D:Vec<BigInt> = Vec::new();     /*Polynomial evaluation function takes Vec of polynomial coeffients, n-number of points
   at which we want the polynomial evaluation, 'k-1' is the degree of the polynomial,*/
   let mut temp:BigInt = Zero::zero();
   let mut x :BigInt = One::one();
   for i in 1..n+1{
    for j in 0..k{
        temp =(temp +(polynomial[j as usize].clone()*x.clone())%prime.clone())%prime.clone();
        x=(x*i)%prime.clone();

    }
    x +=1;
    D.push(temp.clone());
   }
   
return D //D is the vector which contains the evaluation of the polynomial at 1..n points, which is nothing but the 'n' pieces of the secret. 
}
fn choosing_k_out_of_n_x_cordinates(D:Vec<BigInt>,k:i32)->Vec<i32>{
    let n = D.len();
    let mut indexes:Vec<i32> = Vec::new();
    for i in 0..k{
    let mut r:i32 = rand::thread_rng().gen_range(1..n).try_into().unwrap();
    let mut a = indexes.contains(&r);
    while a { r = rand::thread_rng().gen_range(1..n).try_into().unwrap();
         a = indexes.contains(&r);
    }
    indexes.push(r.try_into().unwrap());
}
return indexes;
}
// let k_points:Vec<(usize,BigInt)> = indexes.clone().into_iter().map(|x| (x, D[x].clone())).collect(); /*This will contains
//  k points out of n points,where each point is a tuple (x_cordinate,y_cordinate)  */

// return k_points
fn y_cordinates_of_k_n_points(a:Vec<i32>,D:Vec<BigInt>)->(Vec<BigInt>){
    let mut y_cordinates:Vec<BigInt> = Vec::new();
    for i in 0..a.len(){
        let mut r = &D[a[i] as usize];
        y_cordinates.push(r.clone());
    }
    let k_points:Vec<BigInt> = (y_cordinates);
    
    
    return k_points;
}



fn lagrange_interpolation(x:Vec<i32>,y:Vec<BigInt>,x_evaluate:i32,p:BigInt)->BigInt{ /*lagrange_interpolation() fun takes 
x,y cordinates of k points,x-cordinate of the evaluation point(In our case it is 0) and returns the polynomial value at the given x-cordinate*/
let k = x.len();
let mut sum = Zero::zero();
for i in 0..k {
    let mut numer:BigInt = One::one();
    let mut denom :BigInt= One::one();
    for j in 0..k{
        if i ==j {continue};
        println!("{},{}",x[j],x_evaluate);
        let mut sub = (x_evaluate-x[j]);
        numer = (sub)*numer;
        denom = (x[i] - x[j])*denom;
    }
    sum = sum +(numer*inverse(denom,p.clone()))*y[i].clone();

}
return sum
}
// fn inverse(a:BigInt,n:BigInt) ->BigInt{
//    let mut remainder:BigInt = Zero::zero();
//    let mut 


// }

fn extended_gcd(n:BigInt,a:BigInt)->(BigInt,BigInt){
   
    if a==Zero::zero() {return (One::one(),Zero::zero());}
    let (mut x_2,mut y_2) = extended_gcd(a.clone(),n.clone()%a.clone());
    let mut x = y_2.clone();
    let mut y = x_2 - (n/a)*y_2;
    //if y<0 {y = n+y}
    // else if y<0 && -y>n{ let c = -y%n;
    //     y = n-c;
    // }else {y = y%n ;}
    
    return (x,y);


}
fn y_compute(mut x:BigInt,mut y:BigInt,mut n:BigInt)->BigInt{
    if y<Zero::zero(){ let c = y%n.clone();
        y = n.clone()+c;
    }else {y = y%n ;}
    return y
}
fn inverse(n:BigInt,a:BigInt)->BigInt{
    let mut x:BigInt;
    let mut y:BigInt;
    (x,y) = extended_gcd(n.clone(), a);
    y = y_compute(x, y, n.clone());
    if y<Zero::zero(){ let mut c = y%n.clone();
        y = n+c;
    }else {y = y%n ;}
    return y

 }
// fn mod_reverse(&self, num: BigInt) -> BigInt {
//     let num1 = if num < Zero::zero() {
//         num + &self.prime
//     } else {
//         num
//     };
//     let (_gcd, _, inv) = self.extend_euclid_algo(num1);
//     // println!("inv:{}", inv);
//     inv
// }

/**
 * https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
 *
 * a*s + b*t = gcd(a,b) a > b
 * r_0 = a*s_0 + b*t_0    s_0 = 1    t_0 = 0
 * r_1 = a*s_1 + b*t_1    s_1 = 0    t_1 = 1
 * r_2 = r_0 - r_1*q_1
 *     = a(s_0 - s_1*q_1) + b(t_0 - t_1*q_1)   s_2 = s_0 - s_1*q_1     t_2 = t_0 - t_1*q_1
 * ...
 * stop when r_k = 0
 */
// fn extend_euclid_algo(&self, num: BigInt) -> (BigInt, BigInt, BigInt) {
//     let (mut r, mut next_r, mut s, mut next_s, mut t, mut next_t) = (
//         self.prime.clone(),
//         num.clone(),
//         BigInt::from(1),
//         BigInt::from(0),
//         BigInt::from(0),
//         BigInt::from(1),
//     );
//     let mut quotient;
//     let mut tmp;
//     while next_r > Zero::zero() {
//         quotient = r.clone() / next_r.clone();
//         tmp = next_r.clone();
//         next_r = r.clone() - next_r.clone() * quotient.clone();
//         r = tmp.clone();
//         tmp = next_s.clone();
//         next_s = s - next_s.clone() * quotient.clone();
//         s = tmp;
//         tmp = next_t.clone();
//         next_t = t - next_t * quotient;
//         t = tmp;
//     }
//     // println!(
//     // "{} * {} + {} * {} = {} mod {}",
//     // num, t, &self.prime, s, r, &self.prime
//     // );
//     (r, s, t)
// }



fn main() {
let n = 5; // 'n' is the number of shares  
let k = 2; //'k' is the threshold 
let prime = BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",16).unwrap();
//Large prime number

let secret = BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffc2f", 16).unwrap();
//'secret' is a number which is less than the prime number
println!("{}",secret);
let mut polynomial:Vec<BigInt> = Vec::new(); 
polynomial.push(secret.clone()); //we are setting 'a_0' coefficient as secret, remaining k-1 coefficients are randomly generated.
for i in 1..k{
    let mut rng = rand::thread_rng(); //here we are generating k-1 coefficients randomly.
    //r.gen_range(0..=prime-1);
    let r:BigInt = rng.gen_bigint_range(&BigInt::from(0), &prime);
    polynomial.push(r);
}
//println!("{:?}",polynomial);                  //This polynomial vector will contain all the coefficients of the polynomial
let mut result:Vec<BigInt>= Vec::new();    
result = polynomial_eval(polynomial, n, prime.clone(),k);
println!("{:?}",result);  
let mut x:Vec<i32> = choosing_k_out_of_n_x_cordinates(result.clone(), k);
let mut y:Vec<BigInt> = y_cordinates_of_k_n_points(x.clone(), result);
let mut f_0:BigInt = lagrange_interpolation(x, y, 0, prime);
assert_eq!(secret,f_0);

}
