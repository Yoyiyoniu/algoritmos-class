#[derive(Debug)]
pub(crate) enum Token {
    Num(String),
    Car(char),
}

pub(crate) fn tokenizar(expresion: &str) -> Result<Vec<Token>, ()> {
    let mut tokens = Vec::new();
    let mut num = String::new();
    for c in expresion.chars().map(|c| match c {
        '–' | '−' => '-',
        c => c,
    }) {
        if c.is_whitespace() {
            if !num.is_empty() {
                tokens.push(Token::Num(std::mem::take(&mut num)));
            }
        } else if c.is_ascii_digit() {
            num.push(c);
        } else if "+-*/^!()".contains(c) {
            if !num.is_empty() {
                tokens.push(Token::Num(std::mem::take(&mut num)));
            }
            tokens.push(Token::Car(c));
        } else {
            return Err(());
        }
    }
    if !num.is_empty() {
        tokens.push(Token::Num(num));
    }
    Ok(tokens)
}

pub(crate) fn formatear(tokens: &[Token]) -> Result<String, ()> {
    let mut fmt = Formateador { tokens, pos: 0 };
    let s = fmt.expr()?;
    if fmt.pos != tokens.len() {
        return Err(());
    }
    Ok(s)
}

struct Formateador<'t> {
    tokens: &'t [Token],
    pos: usize,
}

impl Formateador<'_> {
    fn mirar(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn mirar_car(&self) -> Option<char> {
        match self.mirar() {
            Some(Token::Car(c)) => Some(*c),
            _ => None,
        }
    }

    fn atender(&mut self) {
        self.pos += 1;
    }

    fn expr(&mut self) -> Result<String, ()> {
        let mut s = self.termino()?;
        while let Some(op @ ('+' | '-')) = self.mirar_car() {
            self.atender();
            let t = self.termino()?;
            s = format!("({s} {op} {t})");
        }
        Ok(s)
    }

    fn termino(&mut self) -> Result<String, ()> {
        let mut s = self.factor()?;
        while let Some(op @ ('*' | '/')) = self.mirar_car() {
            self.atender();
            let t = self.factor()?;
            s = format!("({s} {op} {t})");
        }
        Ok(s)
    }

    fn factor(&mut self) -> Result<String, ()> {
        let mut s = self.primario()?;
        if self.mirar_car() == Some('^') {
            self.atender();
            let f = self.factor()?;
            s = format!("({s} ^ {f})");
        }
        while self.mirar_car() == Some('!') {
            self.atender();
            s = format!("({s} !)");
        }
        Ok(s)
    }

    fn primario(&mut self) -> Result<String, ()> {
        match self.mirar() {
            Some(Token::Num(n)) => {
                let n = n.clone();
                self.atender();
                Ok(n)
            }
            Some(Token::Car('(')) => {
                self.atender();
                let e = self.expr()?;
                if self.mirar_car() != Some(')') {
                    return Err(());
                }
                self.atender();
                Ok(e)
            }
            _ => Err(()),
        }
    }
}

#[derive(Default, Debug)]
struct Nodo {
    dato: Option<char>,
    valor: Option<String>,
    izq: Option<usize>,
    der: Option<usize>,
}

impl Nodo {
    fn num(valor: &str) -> Self {
        Self {
            valor: Some(valor.to_owned()),
            ..Default::default()
        }
    }
}

struct ArbolEx {
    nodos: Vec<Nodo>,
    raiz: usize,
}

impl ArbolEx {
    fn construir(formateada: &str) -> Result<Self, ()> {
        let tokens = tokenizar(formateada)?;
        let mut nodos: Vec<Nodo> = Vec::new();
        let mut pila: Vec<usize> = Vec::new();
        let mut actual: Option<usize> = None;
        let mut raiz: usize = 0;

        for token in tokens {
            let t = nodos.len();
            match token {
                Token::Car('(') => {
                    nodos.push(Nodo::default());
                    if actual.is_none() && pila.is_empty() {
                        raiz = t;
                    } else {
                        let padre = actual.or(pila.last().copied()).ok_or(())?;
                        let nodo = &mut nodos[padre];
                        if nodo.izq.is_none() {
                            nodo.izq = Some(t);
                        } else if nodo.der.is_none() {
                            nodo.der = Some(t);
                        } else {
                            return Err(());
                        }
                    }
                    pila.push(t);
                    actual = Some(t);
                }
                Token::Car(')') => {}
                Token::Car(op) => {
                    let a = pila.pop().ok_or(())?;
                    actual = Some(a);
                    let nodo = &mut nodos[a];
                    if nodo.dato.is_some() || nodo.valor.is_some() {
                        return Err(());
                    }
                    nodo.dato = Some(op);
                }
                Token::Num(v) => {
                    if t == 0 && actual.is_none() && pila.is_empty() {
                        return Ok(ArbolEx {
                            nodos: vec![Nodo::num(&v)],
                            raiz: 0,
                        });
                    }
                    nodos.push(Nodo::num(&v));
                    let destino = actual.ok_or(())?;
                    let nodo = &mut nodos[destino];
                    if nodo.izq.is_none() {
                        nodo.izq = Some(t);
                    } else if nodo.der.is_none() {
                        nodo.der = Some(t);
                    } else {
                        return Err(());
                    }
                }
            }
        }

        if !pila.is_empty() || nodos.is_empty() {
            return Err(());
        }

        let mut r = raiz;
        while nodos[r].dato.is_none() && nodos[r].valor.is_none() {
            r = nodos[r].izq.ok_or(())?;
        }
        Ok(ArbolEx { nodos, raiz: r })
    }

    fn postorden(&self) -> String {
        fn recorrer(nodos: &[Nodo], n: usize) -> String {
            let nodo = &nodos[n];
            if let Some(v) = &nodo.valor {
                return v.clone();
            }
            let izq = nodo.izq.map(|i| recorrer(nodos, i)).unwrap_or_default();
            let der = nodo.der.map(|d| recorrer(nodos, d)).unwrap_or_default();
            match nodo.dato {
                None => izq,
                Some('!') => format!("{izq} !"),
                Some(op) => format!("{izq} {der} {op}"),
            }
        }
        recorrer(&self.nodos, self.raiz)
    }
}

fn factorial(n: f64) -> Result<f64, ()> {
    if n < 0.0 || n.fract() != 0.0 || n > 170.0 {
        return Err(());
    }
    let mut r = 1.0;
    let mut i = 2.0;
    while i <= n {
        r *= i;
        i += 1.0;
    }
    Ok(r)
}

fn evaluar(postorden: &str) -> Result<f64, ()> {
    let mut pila: Vec<f64> = Vec::new();
    for token in postorden.split_whitespace() {
        if let Ok(n) = token.parse::<f64>() {
            pila.push(n);
            continue;
        }
        let r = match token {
            "!" => factorial(pila.pop().ok_or(())?),
            _ => {
                let der = pila.pop().ok_or(())?;
                let izq = pila.pop().ok_or(())?;
                match token {
                    "+" => Ok(izq + der),
                    "-" => Ok(izq - der),
                    "*" => Ok(izq * der),
                    "/" => Ok(izq / der),
                    "^" => Ok(izq.powf(der)),
                    _ => Err(()),
                }
            }
        }?;
        pila.push(r);
    }
    if pila.len() != 1 {
        return Err(());
    }
    Ok(pila[0])
}

pub(crate) fn calcular(expresion: &str) -> Result<(f64, String), ()> {
    let tokens = tokenizar(expresion)?;
    if tokens.is_empty() {
        return Err(());
    }
    let formateada = formatear(&tokens)?;
    let arbol = ArbolEx::construir(&formateada)?;
    let valor = evaluar(&arbol.postorden())?;
    Ok((valor, formateada))
}
