package tp02.ejercicio1;

import java.util.Scanner;

public class TestEj6 {
    public static void main(String[] args) {
        ListaDeEnterosEnlazada l = new ListaDeEnterosEnlazada();
        Scanner s = new Scanner(System.in);
        Ej6.sucesion(s.nextInt(), l);
        l.comenzar();
        for (int i = 0; i < l.tamanio(); i++) {
            System.out.println(l.elemento(i + 1));
        }
    }

}
