use std::array;

pub struct Velas {
    velas: [String; 26],
    count: usize,
}

impl Velas {
    pub fn new() -> Self {
        Velas {
            velas: array::from_fn(|_| String::new()),
            count: 0,
        }
    }

    pub fn agregar(&mut self, vela: String) -> Result<(), String> {
        self.velas[self.count] = vela;
        self.count += 1;
        Ok(())
    }

    pub fn eliminar(&mut self, i: usize) -> Result<(), String> {
        for j in i..self.count - 1 {
            self.velas[i] = self.velas[j + 1].clone();
        }

        self.velas[self.count - 1] = String::new();
        self.count -= 1;
        Ok(())
    }

    pub fn buscar(&self, q: &str) -> Option<usize> {
        self.velas[..self.count].iter().position(|slot| slot == q)
    }

    pub fn sustituir(&mut self, q: &str, r: &str) -> Result<(), String> {
        match self.buscar(q) {
            Some(i) => {
                self.velas[i] = r.to_string();
                Ok(())
            }
            None => Err(format!("no se encontró '{}'", q)),
        }
    }

    pub fn mostrar(&self) -> Result<&[String], String> {
        if self.count == 0 {
            return Err("no data :V".to_string());
        }
        Ok(&self.velas[..self.count])
    }
}
