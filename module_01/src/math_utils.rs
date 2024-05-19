pub fn square(x:f64)->f64{
     x*x
     }
pub fn cube(x:f64)->f64{
     x*x*x
     }
pub fn square_root(x:f64)->f64{
     x.sqrt()
     }
pub fn cube_root(x:f64)->f64{
     x.cbrt()
     }
pub fn factorial(x:f64)->f64{
     let mut fact1:f64=1.0;
     for i in 1..=x as usize{
         fact1*=i as f64;
    } 
     fact1

}
pub fn sum(x:f64,y:f64)->f64{
     x+y
     }
pub fn sub(x:f64,y:f64)->f64{
     x-y
     }
pub fn mul(x:f64,y:f64)->f64{
     x*y
     }
pub fn div(x:f64,y:f64)->f64{
     x/y
     }
pub fn mod_(x:f64,y:f64)->f64{
     x%y
     }
pub fn pow(x:f64,y:f64)->f64{
     x.powf(y)
     }

pub fn permutation(n:f64,r:f64)->f64{
     let mut fact1:f64=1.0;
     for i in 1..=r as usize{
         fact1*=i as f64;   
    } 
     let mut fact2:f64=1.0;
     for i in 1..=(n-r) as usize{
         fact2*=i as f64;   
    }   
}

// pub fn abs(x:f64)->f64{
//      x.abs()
//      }
// pub fn round(x:f64)->f64{
//      x.round()
//      }
// pub fn floor(x:f64)->f64{
//      x.floor()
//      }
// pub fn ceil(x:f64)->f64{
//      x.ceil()
//      }
// pub fn exp(x:f64)->f64{
//      x.exp()
//      }
// pub fn log(x:f64)->f64{
//      x.log10()
//      }
// pub fn log2(x:f64)->f64{
//      x.log2()
//      }
// pub fn log10(x:f64)->f64{
//      x.log10()
//      }
// pub fn summation(x:f64)->f64{
//      (x*x+1.0)/2.0
//      }