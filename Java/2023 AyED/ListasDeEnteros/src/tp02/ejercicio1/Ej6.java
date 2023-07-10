package tp02.ejercicio1;

public class Ej6 {
    public static void sucesion(int n, ListaDeEnterosEnlazada l) {
        if (n != 1) {
            if (n % 2 == 0) {
                n = n / 2;
                System.out.println(n);
                l.agregarFinal(n);
                sucesion(n, l);
            } else {
                n = 3 * n + 1;
                System.out.println(n);
                l.agregarFinal(n);
                sucesion(n, l);
            }
        }
    }
}
