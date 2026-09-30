use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Nodo<T> {
    pub dato: T,
    pub siguiente: Option<Rc<RefCell<Nodo<T>>>>,
    pub anterior: Option<Weak<RefCell<Nodo<T>>>>,
}

impl<T> Nodo<T> {
    pub fn new(dato: T) -> Self {
        Self {
            dato,
            siguiente: None,
            anterior: None,
        }
    }
}

pub struct ListaDoble<T> {
    cabeza: Option<Rc<RefCell<Nodo<T>>>>,
    cola: Option<Weak<RefCell<Nodo<T>>>>,
    tamano: usize,
}

impl<T> ListaDoble<T> {
    pub fn new() -> Self {
        Self {
            cabeza: None,
            cola: None,
            tamano: 0,
        }
    }

    pub fn tamano(&self) -> usize {
        self.tamano
    }

    pub fn agregar_inicio(&mut self, dato: T) {
        let nuevo_nodo = Rc::new(RefCell::new(Nodo::new(dato)));

        match self.cabeza.take() {
            Some(vieja_cabeza) => {
                vieja_cabeza.borrow_mut().anterior = Some(Rc::downgrade(&nuevo_nodo));
                nuevo_nodo.borrow_mut().siguiente = Some(vieja_cabeza);
                self.cabeza = Some(nuevo_nodo);
            }
            None => {
                self.cola = Some(Rc::downgrade(&nuevo_nodo));
                self.cabeza = Some(nuevo_nodo);
            }
        }
        self.tamano += 1;
    }

    // Navegación por puntero sucesor desde la cabeza
    pub fn obtener_nodo_por_indice(&self, indice: usize) -> Option<Rc<RefCell<Nodo<T>>>> {
        if indice >= self.tamano {
            return None;
        }
        let mut actual = self.cabeza.clone()?;
        for _ in 0..indice {
            let siguiente = actual.borrow().siguiente.clone()?;
            actual = siguiente;
        }
        Some(actual)
    }

    pub fn mover_al_inicio(&mut self, indice: usize) {
        if indice == 0 || indice >= self.tamano {
            return;
        }

        let target_nodo = match self.obtener_nodo_por_indice(indice) {
            Some(nodo) => nodo,
            None => return,
        };

        let prev_weak = target_nodo.borrow().anterior.clone();
        let next_opt = target_nodo.borrow().siguiente.clone();

        // Desconectar el nodo de su posición previa
        if let Some(ref prev_weak) = prev_weak {
            if let Some(prev_rc) = prev_weak.upgrade() {
                prev_rc.borrow_mut().siguiente = next_opt.clone();
            }
        }

        if let Some(ref next_rc) = next_opt {
            next_rc.borrow_mut().anterior = prev_weak.clone();
        } else {
            self.cola = prev_weak;
        }

        // Reinsertar como nueva cabeza
        let vieja_cabeza = self.cabeza.take();
        if let Some(ref vieja) = vieja_cabeza {
            vieja.borrow_mut().anterior = Some(Rc::downgrade(&target_nodo));
        }

        target_nodo.borrow_mut().anterior = None;
        target_nodo.borrow_mut().siguiente = vieja_cabeza;

        self.cabeza = Some(target_nodo);
    }
}
