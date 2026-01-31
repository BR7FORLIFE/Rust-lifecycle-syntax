/**
 * Controles de flujo IF / ELSE / MATCH
 * 
 * Funciones -> fn
 * 
 * 
 * 
 */

fn prove_value(value: i32) -> i32{
    let mut result: i32 = 10;
    if value > 0 { result+=value } else { result -= result}
    result 
}

fn main(){
    println!("Valor de la funcion: {}", prove_value(32));
}