package tp03.ejercicio4;

import java.util.Scanner;
import tp03.ejercicio1y2.*;

public class MainRed {
    public static void main(String[] args) {
        Scanner s = new Scanner(System.in);
        ArbolBinario<Integer> ab = new ArbolBinario<>();
        int num = s.nextInt();
        ab.setDato(num);
        ab.agregarHijoIzquierdo(new ArbolBinario<Integer>());
        num = s.nextInt();
        ab.getHijoIzquierdo().setDato(num);
        ab.agregarHijoDerecho(new ArbolBinario<Integer>());
        num = s.nextInt();
        ab.getHijoDerecho().setDato(num);
        ab.getHijoIzquierdo().agregarHijoIzquierdo(new ArbolBinario<Integer>());
        num = s.nextInt();
        ab.getHijoIzquierdo().getHijoIzquierdo().setDato(num);
        ab.getHijoIzquierdo().agregarHijoDerecho(new ArbolBinario<Integer>());
        num = s.nextInt();
        ab.getHijoIzquierdo().getHijoDerecho().setDato(num);
        System.out.println("----------------------------------------");
        RedBinariaLlena red = new RedBinariaLlena();
        red.setAb(ab);
        System.out.println(red.retardoReenvio());
    }
}
