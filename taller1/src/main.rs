mod velas;
use velas::Velas;

fn main() {
    let mut velas = Velas::new();
    velas.agregar("canela".to_string()).unwrap();
    velas.agregar("tropical".to_string()).unwrap();
    velas.agregar("maracuya".to_string()).unwrap();
    velas.agregar("stefano".to_string()).unwrap();

    print!("{:?}\n", velas.mostrar());

    let b = velas.buscar("maracuya");
    print!("{:?}\n", b);

    let _ = velas.eliminar(2);
    print!("{:?}\n", velas.mostrar());

    let _ = velas.sustituir("stefano", "Abedul");
    print!("{:?}\n", velas.mostrar());
}
