/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Main.java to edit this template
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.tema4;

import PaqueteLectura.Lector;

/**
 *
 * @author igarz
 */
public class CrearFlota {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        Flota flota = new Flota();
        String patente;
        String destino = "";
        String hSalida = "";

        System.out.println("Ingrese patente");
        patente = Lector.leerString();
        if (!patente.equals("ZZZ")) {
            System.out.println("Ingrese destino");
            destino = Lector.leerString();
            System.out.println("Ingrese hora de salida");
            hSalida = Lector.leerString();
        }

        while (!patente.equals("ZZZ")) {
            if (flota.getSiHayEspacio()) {
                flota.addMicro(patente, destino, hSalida);
            } else {
                break;
            }
            System.out.println("Ingrese patente");
            patente = Lector.leerString();
            if (!patente.equals("ZZZ")) {
                System.out.println("Ingrese destino");
                destino = Lector.leerString();
                System.out.println("Ingrese hora de salida");
                hSalida = Lector.leerString();
            }
        }

        System.out.println("Indique destino a verificar");
        destino = Lector.leerString();
        if (flota.getXDestino(destino) != null) {
            System.out.println(flota.getXDestino(destino).toString());
        } else {
            System.out.println("No se encuentra ese micro");
        }
    }

}
