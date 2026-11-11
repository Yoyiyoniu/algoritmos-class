use std::cmp::{Ordering, max};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Membresia {
    Vip,
    Platino,
    Oro,
    Basica,
    Inicial,
}

impl Membresia {
    pub const TODAS: [Membresia; 5] = [
        Membresia::Vip,
        Membresia::Platino,
        Membresia::Oro,
        Membresia::Basica,
        Membresia::Inicial,
    ];

    pub fn nombre(&self) -> &'static str {
        match self {
            Membresia::Vip => "VIP",
            Membresia::Platino => "Platino",
            Membresia::Oro => "Oro",
            Membresia::Basica => "Básica",
            Membresia::Inicial => "Inicial",
        }
    }

    pub fn desde_texto(texto: &str) -> Option<Membresia> {
        Membresia::TODAS
            .into_iter()
            .find(|m| m.nombre().eq_ignore_ascii_case(texto.trim()))
    }
}

#[derive(Clone, Debug)]
pub struct Cliente {
    pub nombre: String,
    pub edad: u32,
    pub telefono: String,
    pub correo: Option<String>,
    pub mensualidades_atrasadas: u32,
    pub membresia: Membresia,
}

impl Cliente {
    pub fn nuevo(
        nombre: &str,
        edad: u32,
        telefono: &str,
        correo: Option<&str>,
        mensualidades_atrasadas: u32,
        membresia: Membresia,
    ) -> Self {
        Self {
            nombre: nombre.trim().to_owned(),
            edad,
            telefono: telefono.trim().to_owned(),
            correo: correo
                .map(str::trim)
                .filter(|c| !c.is_empty())
                .map(str::to_owned),
            mensualidades_atrasadas,
            membresia,
        }
    }

    fn clave(&self) -> String {
        self.nombre.to_lowercase()
    }
}

struct Nodo {
    cliente: Cliente,
    altura: i32,
    izq: Option<Box<Nodo>>,
    der: Option<Box<Nodo>>,
}

fn altura(nodo: &Option<Box<Nodo>>) -> i32 {
    nodo.as_ref().map_or(0, |n| n.altura)
}

fn factor_balance(nodo: &Nodo) -> i32 {
    altura(&nodo.izq) - altura(&nodo.der)
}

fn actualizar_altura(nodo: &mut Nodo) {
    let h = max(altura(&nodo.izq), altura(&nodo.der)) + 1;
    nodo.altura = h;
}

pub struct Arbol {
    raiz: Option<Box<Nodo>>,
    total: usize,
}

impl Default for Arbol {
    fn default() -> Self {
        Self::new()
    }
}

impl Arbol {
    pub fn new() -> Self {
        Self {
            raiz: None,
            total: 0,
        }
    }

    pub fn total(&self) -> usize {
        self.total
    }

    pub fn insertar(&mut self, cliente: Cliente) -> bool {
        let existed = self.buscar(&cliente.nombre).is_some();
        self.raiz = insertar(self.raiz.take(), cliente);
        if !existed {
            self.total += 1;
        }
        !existed
    }

    pub fn buscar(&self, nombre: &str) -> Option<&Cliente> {
        let clave = nombre.trim().to_lowercase();
        let mut actual = self.raiz.as_ref();
        while let Some(nodo) = actual {
            match clave.cmp(&nodo.cliente.clave()) {
                Ordering::Equal => return Some(&nodo.cliente),
                Ordering::Less => actual = nodo.izq.as_ref(),
                Ordering::Greater => actual = nodo.der.as_ref(),
            }
        }
        None
    }

    pub fn eliminar(&mut self, nombre: &str) -> bool {
        let clave = nombre.trim().to_lowercase();
        let encontrado = self.buscar(nombre).is_some();
        self.raiz = eliminar(self.raiz.take(), &clave);
        if encontrado {
            self.total -= 1;
        }
        encontrado
    }

    pub fn listar(&self) -> Vec<Cliente> {
        let mut salida = Vec::with_capacity(self.total);
        in_order(&self.raiz, &mut salida);
        salida
    }

    pub fn filtrar_por_membresia(&self, membresia: Membresia) -> Vec<Cliente> {
        self.listar()
            .into_iter()
            .filter(|c| c.membresia == membresia)
            .collect()
    }

    pub fn deudores(&self) -> Vec<Cliente> {
        self.listar()
            .into_iter()
            .filter(|c| c.mensualidades_atrasadas > 0)
            .collect()
    }
}

fn insertar(nodo: Option<Box<Nodo>>, cliente: Cliente) -> Option<Box<Nodo>> {
    let Some(mut actual) = nodo else {
        return Some(Box::new(Nodo {
            cliente,
            altura: 1,
            izq: None,
            der: None,
        }));
    };

    match cliente.clave().cmp(&actual.cliente.clave()) {
        Ordering::Equal => Some(actual),
        Ordering::Less => {
            actual.izq = insertar(actual.izq.take(), cliente);
            Some(actual)
        }
        Ordering::Greater => {
            actual.der = insertar(actual.der.take(), cliente);
            Some(actual)
        }
    }
    .map(|mut n| {
        actualizar_altura(&mut n);
        balancear(n)
    })
}

fn balancear(mut nodo: Box<Nodo>) -> Box<Nodo> {
    let desbalance = factor_balance(&nodo);

    if desbalance > 1 {
        let hijo_izq = nodo.izq.take().expect("hijo izquierdo");
        nodo.izq = Some(if factor_balance(&hijo_izq) < 0 {
            rotar_izquierda(hijo_izq)
        } else {
            hijo_izq
        });
        return rotar_derecha(nodo);
    }

    if desbalance < -1 {
        let hijo_der = nodo.der.take().expect("hijo derecho");
        nodo.der = Some(if factor_balance(&hijo_der) > 0 {
            rotar_derecha(hijo_der)
        } else {
            hijo_der
        });
        return rotar_izquierda(nodo);
    }

    nodo
}

fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
    let mut x = y.izq.take().expect("hijo izquierdo");
    actualizar_altura(&mut y);
    y.izq = x.der.take();
    x.der = Some(y);
    actualizar_altura(&mut x);
    x
}

fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
    let mut y = x.der.take().expect("hijo derecho");
    actualizar_altura(&mut x);
    x.der = y.izq.take();
    y.izq = Some(x);
    actualizar_altura(&mut y);
    y
}

fn eliminar(nodo: Option<Box<Nodo>>, clave: &str) -> Option<Box<Nodo>> {
    let mut actual = nodo?;

    match clave.cmp(&actual.cliente.clave()) {
        Ordering::Less => {
            actual.izq = eliminar(actual.izq.take(), clave);
        }
        Ordering::Greater => {
            actual.der = eliminar(actual.der.take(), clave);
        }
        Ordering::Equal => {
            if actual.izq.is_none() {
                return actual.der.take();
            }
            if actual.der.is_none() {
                return actual.izq.take();
            }
            let max_izq = maximo(actual.izq.as_ref().expect("subárbol izquierdo"));
            actual.cliente = max_izq.cliente.clone();
            let clave_max = max_izq.cliente.clave();
            actual.izq = eliminar(actual.izq.take(), &clave_max);
        }
    }

    actualizar_altura(&mut actual);
    Some(balancear(actual))
}

fn maximo(nodo: &Nodo) -> &Nodo {
    let mut actual = nodo;
    while let Some(der) = actual.der.as_ref() {
        actual = der;
    }
    actual
}

fn in_order(nodo: &Option<Box<Nodo>>, salida: &mut Vec<Cliente>) {
    if let Some(actual) = nodo {
        in_order(&actual.izq, salida);
        salida.push(actual.cliente.clone());
        in_order(&actual.der, salida);
    }
}

pub fn datos_iniciales() -> Vec<Cliente> {
    let datos: [(&str, u32, &str, &str, u32, Membresia); 15] = [
        (
            "Aquiles Baeza",
            24,
            "6641234501",
            "aquiles.baeza@correo.com",
            0,
            Membresia::Vip,
        ),
        (
            "Elsa Pato",
            31,
            "6641234502",
            "elsa.pato@correo.com",
            2,
            Membresia::Vip,
        ),
        (
            "Susana Oria",
            45,
            "6641234503",
            "susana.oria@correo.com",
            0,
            Membresia::Vip,
        ),
        (
            "Aitor Tilla",
            19,
            "6641234504",
            "aitor.tilla@correo.com",
            1,
            Membresia::Vip,
        ),
        (
            "Debora Melo",
            52,
            "6641234505",
            "debora.melo@correo.com",
            3,
            Membresia::Platino,
        ),
        (
            "Armando Casas",
            27,
            "6641234506",
            "armando.casas@correo.com",
            0,
            Membresia::Platino,
        ),
        (
            "Elba Lazo",
            38,
            "6641234507",
            "elba.lazo@correo.com",
            1,
            Membresia::Oro,
        ),
        ("Benito Camelo", 22, "6641234508", "", 0, Membresia::Oro),
        ("Zoila Vaca", 41, "6641234509", "", 2, Membresia::Basica),
        ("Lola Mento", 60, "6641234510", "", 0, Membresia::Basica),
        ("Esteban Dido", 17, "6641234511", "", 1, Membresia::Basica),
        (
            "Elba Surita",
            35,
            "6641234512",
            "elba.surita@correo.com",
            0,
            Membresia::Basica,
        ),
        ("Marcia Ana", 29, "6641234513", "", 0, Membresia::Basica),
        (
            "Inés Tornillo",
            44,
            "6641234514",
            "ines.tornillo@correo.com",
            0,
            Membresia::Inicial,
        ),
        (
            "Zacarías Flores",
            26,
            "6641234515",
            "zacarias.flores@correo.com",
            1,
            Membresia::Inicial,
        ),
    ];

    datos
        .iter()
        .map(|(nombre, edad, tel, correo, atrasadas, membresia)| {
            Cliente::nuevo(nombre, *edad, tel, Some(correo), *atrasadas, *membresia)
        })
        .collect()
}

