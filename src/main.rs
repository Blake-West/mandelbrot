use num_complex::Complex;

#[inline]
fn compute_mandelbrot(z: Complex<f64>, c : Complex<f64>) -> Complex<f64> {

  return z * z + c;
}
fn main() {
  let complex_constant = Complex::new(-0.2,0.1);

//    c.to_string();
  println!("My complex constant: {}", complex_constant);
  let mut z = Complex::new(0.0,0.0);
  let iterations = 1000;
  println!("Z(0) = {}", z);
  for x in 1..iterations {
    z = compute_mandelbrot(z, complex_constant);
    println!("Z({}) = {}", x, z);
    if f64::sqrt(z.norm_sqr()) > 2.0 {
      println!("Mandelbrot set diverges...");
      break;
    }
  }
  // println!("Hello, world!");
  // compute_mandelbrot()

}
