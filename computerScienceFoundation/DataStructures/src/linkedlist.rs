/*
    Una lista enlazada es una colecccion de nodos donde cada nodo tiene

    - Un dato
    - un puntero al siguiente nodo

    a diferencia de un array: 

    - memoria no contigua (el OS no necesita ocupar un bloque fisico unico y consecutivo de memoria)
    - no compatible con cache
    - tamaño dinamico
    - facil insercion o eliminacion de nodos 

    ejemplo simplicado de como se ve una lista enlazada

    Node A (0x1000) -> Node B (0x3F20) -> Node C (0xAA10) -> null

    A tener en cuenta
    - memoria es dispersa ya que no es contigua como en los arrays ordinarios
    - cada nodo almacena una direccion
*/

struct Node{
    value: i32,
    next: Option<Box<Node>> //Box -> le decimos:  aloja el objeto en el HEAP y haces referencia en el stack
}

fn linkedlist(){

    //Este es el end o el final vemos que en el next no apuntamos a nada
    // al ser el ultimo elemento de la lista enlazada
    let node3 = Node{
        value: 30,
        next: None // <-- sera null o el ultimo elemento de la lista enlazada
    };

    let node2 = Node{
        value: 15,
        next: Some(Box::new(node3))
    };

    let node1  = Node{
        value: 7,
        next: Some(Box::new(node2))
    };
    
    // Como se ve por detras: 

    /*
        node1 (stack)
            ↓
        heap Node(15)
            ↓
        heap Node(30)
            ↓
            None
    */

    println!("Linked list created");
}