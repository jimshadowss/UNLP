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
public class InstEntyJug {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        double sueldo, antiguedad;
        int partidos, goles, camps;
        String nombre;
        System.out.println("Definiremos el jugador");
        System.out.println("Ingrese nombre: ");
        nombre = Lector.leerString();
        System.out.println("Ingrese antiguedad: ");
        antiguedad = Lector.leerDouble();
        System.out.println("Ingrese sueldo basico: ");
        sueldo = Lector.leerDouble();
        System.out.println("Ingrese goles hechos");
        goles = Lector.leerInt();
        System.out.println("Ingrese partidos jugados");
        partidos = Lector.leerInt();
        Jugador messi = new Jugador(nombre, sueldo, antiguedad, partidos, goles);
        System.out.println(messi.toString());
        System.out.println("Ahora definiremos el entrenador");
        System.out.println("Ingrese el nombre: ");
        nombre = Lector.leerString();
        System.out.println("Ingrese antiguedad: ");
        antiguedad = Lector.leerDouble();
        System.out.println("Ingrese sueldo basico: ");
        sueldo = Lector.leerDouble();
        System.out.println("Ingrese campeonatos ganados: ");
        camps = Lector.leerInt();
        Entrenador uno = new Entrenador(nombre, sueldo, antiguedad, camps);
        System.out.println(uno.toString());
    }

}
