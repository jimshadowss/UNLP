/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Main.java to edit this template
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.EJ3A;

import PaqueteLectura.Lector;

/**
 *
 * @author igarz
 */
public class InstPersyTrb {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        String nombre, tarea;
        int edad, dni;
        System.out.println("Definiremos a la persona");
        System.out.println("Ingrese nombre:");
        nombre = Lector.leerString();
        System.out.println("Ingrese DNI:");
        dni = Lector.leerInt();
        System.out.println("Ingrese edad:");
        edad = Lector.leerInt();
        Persona p = new Persona(nombre, dni, edad);
        System.out.println(p.toString());
        System.out.println("Definiremos al trabajador");
        System.out.println("Ingrese nombre:");
        nombre = Lector.leerString();
        System.out.println("Ingrese DNI:");
        dni = Lector.leerInt();
        System.out.println("Ingrese edad:");
        edad = Lector.leerInt();
        System.out.println("Ingrese tarea:");
        tarea = Lector.leerString();
        Trabajador t = new Trabajador(nombre, dni, edad, tarea);
        System.out.println(t.toString());
    }

}
