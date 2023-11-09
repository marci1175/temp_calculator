#[derive(Debug)]
struct FTemp {
    val: f64,
}

impl FTemp {

    pub fn wrapval(val : f64) -> FTemp {
        FTemp { val }
    }

    pub fn convertval(val : FTemp) -> CTemp {
        CTemp { val: val.val / 1.8 - 32. } 
    }


}

#[derive(Debug)]
struct CTemp {
    val: f64,
}

impl CTemp {
    pub fn wrapval(val : f64) -> CTemp {
        CTemp { val }
    }

    pub fn convertval(val : CTemp) -> FTemp {
        FTemp { val: val.val * 1.8 + 32. } 
    }
}

fn main() {
    let base : f64 = 30.;

    println!("{}", &base);

    let C_base = CTemp::wrapval(base.clone());

    println!("{:?}", &C_base);

    let F_base = CTemp::convertval(C_base);

    println!("{:?}", &F_base);
}
