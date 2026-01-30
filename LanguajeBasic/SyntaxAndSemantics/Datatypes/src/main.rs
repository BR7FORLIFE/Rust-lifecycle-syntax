// Data Types

/**

Enteros con signo

| Tipo    | Bytes | Bits | Rango        | Detalles hardcore        |
| ------- | ----- | ---- | ------------ | ------------------------ |
| `i8`    | 1     | 8    | −128..127    | Usado para I/O binario   |
| `i16`   | 2     | 16   | −32K..32K    | Poco usado               |
| `i32`   | 4     | 32   | −2.1B..2.1B  | **Default para enteros** |
| `i64`   | 8     | 64   | ±9.22e18     | Registros nativos        |
| `i128`  | 16    | 128  | enorme       | Implementado en software |
| `isize` | 8     | 64   | depende arch | Tamaño de puntero        |

Enteros sin signo (unsigned)

| Tipo    | Bytes | Bits | Rango              | Notas        |
| ------- | ----- | ---- | ------------------ | ------------ |
| `u8`    | 1     | 8    | 0..255             | Bytes reales |
| `u16`   | 2     | 16   |                    |              |
| `u32`   | 4     | 32   |                    |              |
| `u64`   | 8     | 64   |                    |              |
| `u128`  | 16    | 128  |                    |              |
| `usize` | 8     | 64   | índices / punteros |              |

Floating point (IEEE 754)

| Tipo  | Bytes | Bits | Precisión     | Detalles      |
| ----- | ----- | ---- | ------------- | ------------- |
| `f32` | 4     | 32   | ~7 decimales  | SIMD friendly |
| `f64` | 8     | 64   | ~15 decimales | Default       |

Boolean

| Tipo   | Bytes | Bits |
| ------ | ----- | ---- |
| `bool` | 1     | 8    |

Char y Strings

| Tipo   | Bytes | Bits |
| ------ | ----- | ---- |
| `char` | 4     | 32   |

&str (string slice)

| Tipo   | Bytes | Bits |
| ------ | ----- | ---- |
| `char` | 4     | 32   |

String

| Ubicación | Contenido                  |
| --------- | -------------------------- |
| stack     | ptr + len + cap (24 bytes) |
| heap      | bytes UTF-8                |

Tipos compuestos

- Tuples (i32, f64, bool) -> layout secuencial , padding por alignment

- Struct

struct A {
    x: u8,
    y: u64
}

orden de campos

// MAL
u8 + u64 = 16 bytes

// BIEN
u64 + u8 = 9 → padding → 16


Arrays

[i32; 4]

Vectores

Vec<T>

| Parte | Bytes |
| ----- | ----- |
| ptr   | 8     |
| len   | 8     |
| cap   | 8     |

Enums

enum Option<T> {
    None,
    Some(T),
}

Null pointer optimization

Option<&T> == size_of<&T>()

*/
// Glosario

/*
alignment (alineacion) -> indica en que direcciones de memoria puede empezar un dato.

Ejemplo:
    un u32 (4bytes) suele requerir alineacion de 4 bytes
    eso significa que debe empezar en una direccion multiplo de 4

Direcciones:
    0x1000 ✅
    0x1004 ✅
    0x1008 ✅
    0x1002 ❌

Padding -> bytes "vacios" que el compilador inserta para respetar la alineación

 Ejemplo:
    struct A {
        a: u8,   // 1 byte
        b: u32,  // 4 bytes
    }
 Layout Real:
    a: 1 byte
    padding: 3 bytes
    b: 4 bytes
    TOTAL: 8 bytes

*/
use std::mem::{align_of, size_of};

struct Alltypes {
    a: i8,
    b: i32,
    c: u64,
    d: f32,
    e: bool,
    f: char,
}

/**
 * ---------------------------------------------------------------------------------------------------------
 * embedded value -> es un valor que vive directamente dentro de otra estructura no a través de un puntero
 * 
 * Ejemplo: 
 * 
 *  struct A { x: i32 // valor embebido! }  
 * en memoria:[A.x] 
 *     - no hay puntero 
 *     - no hay heap
 *     - no hay indireccion
 *  
 * Comparación con puntero
 * 
 * struct B { x: Box<i32> // no embebido }
 * en memoria: [B.x] -> heap -> [i32]
 * 
 * ---------------------------------------------------------------------------------------------------------
 * 
 * Direccion real -> es la dirección fisica o virtuañ donde el dato vive en memoria.
 * 
 * Ejemplo: 
 *  
 *  let x = 5;
 *  let p = &x
 *  
 *  x -> valor
 *  &x -> direccion en memoria de ese valor!
 * 
 * ---------------------------------------------------------------------------------------------------------
 * 
 *  En Rust: 
 * 
 * Stack -> direcciones temporales
 * Heap -> direcciones dinamicas
 * Static -> direcciones fijas
 * 
 * Lifetime global -> Un valor con lifetime 'static' vive durante todo el programa !(no se libera NUNCA)
 * 
 * ---------------------------------------------------------------------------------------------------------
 * 
 * const VS static
 * 
 * CONST 
 * 
 * const n: i32 = 12;
 * 
 *  - no tiene dirección fija
 *  - Se inserta directamente en el codigo
 *  - Puede copiarse infitamente
 *  - No ocupa memoria "real" única
 * 
 * STATIC
 * 
 * static x: i32 = 10;
 * 
 * - Tiene una direccion real 
 * - Vive en memoria global
 * - Lifetime 'static'
 * - Puede ser referenciado
 * 
 */

// STACK

/*
 Que es: 
    
    - Memoria automatica
    - Crece / decrece con llamadas
    - Muy rapida
    - Direcciones cambian

*/


// STACK -> LIFETIME

// fn foo() -> &i32 {
//     let x = 5;
//     &x // LifeTime invalido
// }
fn foo() -> i32{
    let x = 5;
    x
}

// Stack vs Static

/*
| Stack                | Static            |
| -------------------- | ----------------- |
| Temporal             | Permanente        |
| Dirección cambia     | Dirección fija    |
| Scoped               | Global            |
| Seguro por lifetimes | Seguro por diseño |
*/


fn main() {
    //Alignment - Size memory
    println!("Size_of::<Alltypes>() = {}", size_of::<Alltypes>());
    println!("align_of::<Alltypes>() = {}", align_of::<Alltypes>());

    let x: i8 = 5;
    let y: &i8 = &x;

    // {:p} -> direccion de memoria a la cual apunta la variable
    println!("valor real: {} - dirección en memoria: {:p}", x, y);
    println!("valor de la funcion: {}", foo());
}
