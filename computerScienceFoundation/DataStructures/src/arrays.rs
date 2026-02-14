fn slices(arr: &[i32]){
    for value in arr {
        println!("{}", value)
    }
}


fn array() {
    //Array en rust definicion y comportamiento

    // Definicion de un array con elementos -> [Type;size]
    let arr: [i32; 4] = [1,2,3,4];

    //Atajo de inicializacion de un array
    let zeros = [0;10]; // crea ->  [0,0,0,0,0,0,0,0,0,0]


    //Como trabaja la indexacion en los ARRAYS ej: 
    let value = arr[2];

    /*sucede internamente este proceso: 

        memory_address + (index * size_of_element)

        si: 
         la direccion de memoria base es 0x1000
         indice a la cual queremos usar es 2
         y el tamaño del array es de 4 elementos 

         siguiendo la formula el elemento en esa posicion estaria en la direccion de memoria

         0x1000 + (2 x 4) = 0x1008 -> en esta posicion de memoria se enncuentra el dato con el indice especificado
    */

    //BOUNDs CHECKINGS

    /* que pasa si hacemos esto: arr[10] -> panic en runtime 

        esto prevee acceso a memoria que no existe o corrupcion de los mismos

        en C podrias acceder a memoria aleatoria PELIGROSO
        pero en rust te protege de este tipo de corrupcion en su modo seguro
    */
    
    //ARRAY VS VECTORS

    let arr2 = [1,2,3]; //tamaño fijo , layout de memoria simple
    let vec  = vec![1,2,3]; // tamaño dinamico , guardado en el heap

}
