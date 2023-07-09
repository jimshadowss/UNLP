package tp02.ejercicio1;

import java.util.Scanner;

public class TestListaDeEnterosConArreglos {
    public static void main(String[] args) {
        int i;
        ListaDeEnterosConArreglos l = new ListaDeEnterosConArreglos();
        Scanner s = new Scanner(System.in);
        for (i = 0; i < 3; i++) {
            System.out.println("Ingrese el dato " + (i + 1));
            l.agregarFinal(s.nextInt());
        }
        for (i = 0; i < 3; i++) {
            l.comenzar();
            System.out.println(l.elemento(i + 1));
        }
    }
}