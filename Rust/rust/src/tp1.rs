use std::{
    fmt,
    io::{stdin, Read},
};

/*1- Escribir un programa que defina una variable de tipo flotante con algún valor, y luego
permita al usuario ingresar un número decimal por teclado para multiplicar, dividir, sumar y
restar su valor. Se deben imprimir los resultados.
 */

pub fn ej1() {
    let num1: f32 = 3.5;
    let mut buf: String = String::new();
    println!("Ingrese numero a operar contra 3.5");
    std::io::stdin().read_line(&mut buf).expect("Error");
    let num2: f32 = buf.trim().parse().expect("No es i32");
    println!("Suma: {}", num1 + num2);
    println!("Resta: {}", num1 - num2);
    println!("Multipilcacion: {}", num1 * num2);
    println!("Division: {}", num1 / num2);
}

/*2- Escribir un programa que defina una variable de tipo entero sin signo, y luego imprima su
valor en hexadecimal.
 */

pub fn ej2() {
    let mut buf: String = String::new();
    println!("Ingrese el numero a convertir:");
    std::io::stdin().read_line(&mut buf).expect("Error");
    let num2: i32 = buf.trim().parse().expect("No es u32");
    assert_eq!(format!("{:x}", num2), "2a");
}

/*3- Escribir un programa que defina una variable de tipo booleano, y luego permita al usuario
ingresar un valor booleano por teclado para actualizar su valor haciendo las operaciones
and y or. Se deben imprimir ambos resultados.
 */

pub fn ej3() {
    let bool = true;
    let mut buf: String = String::new();
    println!("Ingrese su valor de verdad para analizar:");
    std::io::stdin().read_line(&mut buf).expect("Error");
    let var: bool = buf.trim().parse().expect("No es boolean");
    let bool1 = bool && var;
    let bool2 = bool || var;
    println!("El valor del AND es {}", bool1);
    println!("El valor del OR es {}", bool2);
}

/*4- Escribir un programa que defina una tupla que contenga una cadena, un número entero
con signo y un valor booleano, y luego imprima cada valor de la tupla */

pub fn ej4() {
    let tupla: (String, i32, bool) = ("Hola".to_string(), -45, false);
    println!(
        "{}, es {} que la temperatura exterior sea de {}°C.",
        tupla.0, tupla.2, tupla.1
    );
}

/*5- Escribir un programa que defina una variable de tipo cadena, y luego permita al usuario
ingresar una cadena por teclado para concatenar su valor. El programa debe imprimir la
cadena en mayúsculas.
 */

pub fn ej5() {
    let mut buf: String = String::new();
    println!("Ingrese su texto:");
    std::io::stdin().read_line(&mut buf).expect("Error");
    let mut cad: String = buf.to_uppercase();
    buf = String::from("");
    std::io::stdin().read_line(&mut buf).expect("Error");
    cad = cad + buf.to_uppercase().trim();
    println!("{}", cad);
}

/*6- Escribir un programa que defina una variable de tipo entero sin signo, y luego permita al
usuario ingresar un número entero por teclado para sumarse con la variable definida. El
programa debe imprimir el valor del número elevado al cuadrado. */

pub fn ej6() {
    let mut mi_num: u32 = 3;
    let mut buf = String::new();
    println!("Ingrese su numero");
    std::io::stdin().read_line(&mut buf).expect("Error");
    let num: u32 = buf.trim().parse().expect("No es entero sin signo");
    println!("{}+{}={}", mi_num, num, mi_num + num);
    mi_num += num;
    println!("{} al cuadrado igual a {}", mi_num, mi_num.pow(2));
}

/*7- Escribir un programa que defina una variable de tipo arreglo que contenga seis números
enteros, y luego multiplique cada valor del arreglo por un valor constante definido,
modificando el contenido del arreglo. */

pub fn ej7() {
    let arr: [i32; 6] = [3, 5, 2, 7, 6, 4];
    let mut arr2: [i32; 6] = [0, 0, 0, 0, 0, 0];
    let num = 3;
    println!("El arreglo es {:?}", arr);
    println!("El numero fijo es {}", num);
    for i in 0..arr.len() {
        arr2[i] = arr[i] * num;
    }
    println!("El arreglo ahora es {:?}", arr2);
}

/*8- Escribir un programa que defina una constante de tipo cadena, y luego imprima el
número de veces que un caracter específico ingresado por el usuario aparece en la cadena.
Se debe imprimir el resultado */

pub fn ej8() {
    let arr: &str = "55i481i5i31i5isieidjeksg4f8f84";
    let mut cant: i16 = 0;
    let mut buf = String::new();
    println!("Ingrese el caracter a buscar");
    std::io::stdin().read_line(&mut buf).expect("Error");
    let dato: char = buf.trim().parse().expect("No es char");

    for i in 1..arr.len() + 1 {
        println!("{}", i);
        // println!("{}", arr.len());
        if arr.chars().nth(i - 1).unwrap() == dato {
            cant += 1;
        }
    }
    println!("La cantidad de repeticiones de ese caracter es: {}", cant);
}

/* 9- Escribir un programa que defina un arreglo de 5 números enteros, y luego imprima la
suma de los valores del  arreglo */
pub fn ej9() {
    let arr: [i32; 5] = [4, 5, 6, 4, 7];
    let mut aux = 0;
    for i in 1..arr.len() + 1 {
        println!("{}", arr[i - 1]);
        aux += arr[i - 1];
    }
    println!("La suma es: {}", aux);
}

/*10- Escribir un programa que defina dos arreglos de 5 números enteros cada uno, y luego
cree un tercer arreglo que contenga la suma de los elementos de los dos arreglos
originales. */

pub fn ej10() {
    let arr1: [i32; 5] = [54, 5, 65, 12, 3];
    let arr2: [i32; 5] = [1, 50, -10, 43, 52];
    let mut res: [i32; 5] = [0, 0, 0, 0, 0];
    for i in 0..arr1.len() {
        res[i] = arr1[i] + arr2[i];
    }
    println!("{:?}", res);
}
/* 11- Escribir un programa que defina un arreglo de 5 cadenas, y luego permita al usuario
ingresar una cadena por teclado. El programa debe imprimir un mensaje si la cadena
ingresada por el usuario se encuentra en el arreglo.  */

pub fn ej11() {
    let arr: [&str; 5] = ["abc", "599", "liauhfbg", "kshbgvf", "ksfvhjb@"];
    let mut es: bool = false;
    let cadena: &str;
    let mut buf = String::new();
    println!("Ingrese su cadena");
    std::io::stdin().read_line(&mut buf).expect("Error");
    cadena = buf.trim();
    // buf = buf.trim().to_string();
    for i in 0..arr.len() {
        if arr[i] == cadena {
            es = true;
        }
    }
    if es {
        println!("Correcto");
    } else {
        println!("Incorrecto")
    }
}

/* 12- Escribir un programa que defina una tupla que contenga una cadena y un arreglo de
enteros, y luego imprima la cadena y la suma de los valores en el arreglo. */

pub fn ej12() {
    let tupla = ("La suma del arreglo es: ", [4, 65, 48, 7, 98]);
    let mut suma: i32 = 0;
    for i in 0..tupla.1.len() {
        suma += tupla.1[i];
    }
    print!("{}", tupla.0);
    println!("{}", suma);
}
