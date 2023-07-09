/*
 * B- Realice un programa que cree un micro con patente “ABC123”, destino “Mar del Plata” y
hora de salida “5:00”. Lea 5 nros. de asiento, que corresponden a pedidos de personas.
Para cada nro. ingresado debe: validar el nro.; en caso que esté libre dicho asiento,
ocuparlo e informar el éxito de la operación; en caso que esté ocupado, mostrar el nro. del
primer asiento libre. Al finalizar, informe la cantidad de asientos ocupados del micro.
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.tema4;

import PaqueteLectura.Lector;
import PaqueteLectura.GeneradorAleatorio;

/**
 *
 * @author igarz
 */
public class Viaje {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        String patente = "ABC123";
        String destino = "Mar del Plata";
        String hSalida = "5:00";
        int[] nAsiento = new int[2];
        GeneradorAleatorio.iniciar();
        Micro micro = new Micro(patente, destino, hSalida);
        for (int i = 0; i < 5; i++) {
            System.out.println("Ingrese columnda de su asiento:");
            nAsiento[0] = Lector.leerInt();
            System.out.println("Ingrese fila de su asiento:");
            nAsiento[1] = Lector.leerInt();
            if (micro.getSiEnRango(nAsiento)) {
                if (!micro.getSiOcupado(nAsiento)) {
                    micro.setOcupado(nAsiento);
                    System.out.println("Asiento ocupado correctamente.");
                } else {
                    System.out.println(
                            "Se encuentra libre el asiento " + micro.getLibre()[0] + ", " + micro.getLibre()[1]);
                }
            }

        }
        System.out.println("Cantidad de asientos ocupados: " + micro.getOccupied());
    }
}
