
fn main() {

    let mut cac=Cacher::new(|a| a);
    let v1=cac.value(1);
    println!("{}", v1);
}

#[derive(Debug)]
struct Cacher<T>
where T: Fn(u32) -> u32
{
calculation: T,
value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32{

    fn new(calculation: T) -> Cacher<T> {
        Cacher {
        calculation,
        value: None,
        }

    }


    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
                None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
                },

                        }
    }
}

