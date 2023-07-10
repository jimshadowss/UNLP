/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Main.java to edit this template
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.tema5;

import PaqueteLectura.Lector;

/**
 *
 * @author igarz
 */
public class InstTRyCR {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        double lado1, lado2, lado3, radio;
        String CL, CR;
        System.out.println("Definiremos el triangulo");
        System.out.println("Ingrese lado 1");
        lado1 = (Lector.leerDouble());
        System.out.println("Ingrese lado 2");
        lado2 = (Lector.leerDouble());
        System.out.println("Ingrese lado 3");
        lado3 = (Lector.leerDouble());
        System.out.println("Ingrese color de linea");
        CL = (Lector.leerString());
        System.out.println("Ingrese color de relleno");
        CR = (Lector.leerString());
        Triangulo tr = new Triangulo(lado1, lado2, lado3, CL, CR);
        System.out.println(tr.toString());
        System.out.println("***********DESPINTANDO***********");
        tr.despintar();
        System.out.println(tr.toString());

        System.out.println("Ahora definiremos el ciruclo");
        System.out.println("Ingrese el radio");
        radio = Lector.leerDouble();
        System.out.println("Ingrese color de linea");
        CL = Lector.leerString();
        System.out.println("Ingrese color de relleno");
        CR = Lector.leerString();
        Circulo cr = new Circulo(radio, CR, CL);
        System.out.println(cr.toString());
        System.out.println("***********DESPINTANDO***********");
        cr.despintar();
        System.out.println(cr.toString());
    }

}
