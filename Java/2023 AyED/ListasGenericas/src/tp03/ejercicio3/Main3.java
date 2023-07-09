package tp03.ejercicio3;

import java.util.Scanner;

import tp02.ejercicio3.ColaGenerica;
import tp03.ejercicio1y2.*;
import tp03.ejercicio4.RedBinariaLlena;

public class Main3 {
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
        ContadorArbol cont = new ContadorArbol(ab);
        ColaGenerica<Integer> cola = cont.numerosPares();
        while (!cola.esVacia()) {
            System.out.println(cola.desencolar());
        }
    }
}
