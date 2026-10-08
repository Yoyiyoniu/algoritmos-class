use std::fmt;

pub const CAPACIDAD_MAXIMA: usize = 3;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCola {
    Saturada,
    SinElementos,
}

impl fmt::Display for ErrorCola {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCola::Saturada => write!(
                f,
                "ERR_QUEUE_FULL: Buffer saturado (max {})",
                CAPACIDAD_MAXIMA
            ),
            ErrorCola::SinElementos => {
                write!(f, "ERR_QUEUE_EMPTY: No hay lotes pendientes en la linea")
            }
        }
    }
}

pub struct Queue {
    pub slots: [Option<String>; CAPACIDAD_MAXIMA],
    cabeza: usize,
    cola: usize,
    pub elementos: usize,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            slots: Default::default(),
            cabeza: 0,
            cola: 0,
            elementos: 0,
        }
    }

    #[inline]
    pub fn esta_llena(&self) -> bool {
        self.elementos >= CAPACIDAD_MAXIMA
    }

    pub fn encolar(&mut self, item: String) -> Result<(), ErrorCola> {
        if self.esta_llena() {
            return Err(ErrorCola::Saturada);
        }

        self.slots[self.cola] = Some(item);
        self.cola = (self.cola + 1) % CAPACIDAD_MAXIMA;
        self.elementos += 1;
        Ok(())
    }

    pub fn desencolar(&mut self) -> Result<String, ErrorCola> {
        if self.elementos == 0 {
            return Err(ErrorCola::SinElementos);
        }

        let item = self.slots[self.cabeza].take();
        self.cabeza = (self.cabeza + 1) % CAPACIDAD_MAXIMA;
        self.elementos -= 1;

        match item {
            Some(val) => Ok(val),
            None => Err(ErrorCola::SinElementos),
        }
    }

    pub fn obtener_identificadores(&self) -> Vec<String> {
        let mut ids = Vec::with_capacity(self.elementos);
        let mut pos = self.cabeza;
        let mut count = 0;

        while count < self.elementos {
            if let Some(ref val) = self.slots[pos] {
                ids.push(val.clone());
            }
            pos = (pos + 1) % CAPACIDAD_MAXIMA;
            count += 1;
        }

        ids
    }
}
