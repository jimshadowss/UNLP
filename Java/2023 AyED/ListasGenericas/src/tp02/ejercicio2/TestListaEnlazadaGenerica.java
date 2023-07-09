package tp02.ejercicio2;

import java.util.Scanner;
import practica1b.ej3.*;

public class TestListaEnlazadaGenerica {
    public static void main(String[] args) {
        Scanner s = new Scanner(System.in);
        String nombre = s.nextLine();
        String apellido = s.nextLine();
        int comision = s.nextInt();
        String email = s.nextLine();
        String direccion = s.nextLine();
        ListaEnlazadaGenerica l = new ListaEnlazadaGenerica();
        Estudiante[] ests = new Estudiante[4];
        for (int i = 0; i < 4; i++) {
            ests[i] = new Estudiante(nombre, apellido, comision, email, direccion);
            l.agregarFinal(ests[i]);
            ests[i].tusDatos();
        }

    }
}
