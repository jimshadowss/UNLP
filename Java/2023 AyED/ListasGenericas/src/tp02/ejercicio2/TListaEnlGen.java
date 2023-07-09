package tp02.ejercicio2;

import java.util.Scanner;
import practica1b.ej3.*;

public class TListaEnlGen {
    public static void main(String[] args) {
        Scanner s = new Scanner(System.in);
        System.out.println("Nombre:");
        String nombre = s.nextLine();
        System.out.println("Apellido:");
        String apellido = s.nextLine();
        System.out.println("Comision:");
        int comision = s.nextInt();
        s.nextLine();
        System.out.println("Email:");
        String email = s.nextLine();
        System.out.println("Direccion:");
        String direccion = s.nextLine();
        ListaEnlazadaGenerica<Estudiante> l = new ListaEnlazadaGenerica<Estudiante>();
        Estudiante[] ests = new Estudiante[4];
        for (int i = 0; i < 4; i++) {
            ests[i] = new Estudiante(nombre, apellido, comision, email, direccion);
            l.agregarFinal(ests[i]);
            System.out.println(ests[i].tusDatos());
        }
    }
}
