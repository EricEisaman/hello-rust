fn main() {
    let mut a: f64 = 1.0;
    let mut b: f64 = 1.0 / (2.0_f64).sqrt();
    let mut t: f64 = 0.25;
    let mut x: f64 = 1.0;
    loop{
        let y: f64 = a;
        a = (a+b)/2.0;
        b = (b*y).sqrt();
        t = t - x * (y-a)*(y-a);
        x = 2.0 * x;
        println!("a - b = {}",a-b);
        if a-b<1e-5{
            println!("PI: {}",((a+b)*(a+b))/(4.0*t));
            return
        }
    }  
}
