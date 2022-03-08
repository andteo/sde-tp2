
  struct VectorSortat{
        lista: Vec<i32>,
    }

impl VectorSortat{
    fn add(&mut self, elem:i32){
        if self.lista.len()==0{
            self.lista.push(elem);
        }else{
            let mut poz=self.lista.len();
        for i in 0..self.lista.len() {
             if self.lista[i]>elem {
                poz=i;
                break;
             }
        }
    &mut self.lista.insert(poz, elem); 
        }
           
    }
    fn remove(&self, index:i32){

    }
    fn new()->VectorSortat{
        return VectorSortat{lista: Vec::new()};
    }
    
    
}

fn main() {
    let mut v:VectorSortat= VectorSortat::new();
    v.add(7);
    v.add(10);
    v.add(1);
    v.add(5);
    v.add(30);
    println!("{:?}", v.lista);
}