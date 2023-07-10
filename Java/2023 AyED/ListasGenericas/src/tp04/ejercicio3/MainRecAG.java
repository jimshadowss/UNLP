package tp04.ejercicio3;

import java.util.Scanner;

import tp02.ejercicio2.ListaEnlazadaGenerica;
import tp02.ejercicio2.ListaGenerica;
import tp04.ejercicio1.ArbolGeneral;

public class MainRecAG {
    public static void main(String[] args) {
        Scanner s = new Scanner(System.in);
        Integer raiz = (Integer) s.nextInt();
        Integer h1 = (Integer) s.nextInt();
        Integer h2 = (Integer) s.nextInt();
        Integer h3 = (Integer) s.nextInt();
        Integer hh1 = (Integer) s.nextInt();
        System.out.println("----------------------------------------------");
        ArbolGeneral<Integer> aghh1 = new ArbolGeneral<Integer>(hh1);
        ListaGenerica<ArbolGeneral<Integer>> l3 = new ListaEnlazadaGenerica<>();
        l3.agregarFinal(aghh1);
        ArbolGeneral<Integer> agh1 = new ArbolGeneral<Integer>(h1, l3);
        ArbolGeneral<Integer> agh2 = new ArbolGeneral<Integer>(h2);
        ArbolGeneral<Integer> agh3 = new ArbolGeneral<Integer>(h3);

        ListaGenerica<ArbolGeneral<Integer>> l2 = new ListaEnlazadaGenerica<>();

        l2.agregarFinal(agh1);
        l2.agregarFinal(agh2);
        l2.agregarFinal(agh3);

        ArbolGeneral<Integer> agraiz = new ArbolGeneral<Integer>(raiz, l2);
        // ListaEnlazadaGenerica<Integer> result = (ListaEnlazadaGenerica) RecorridosAG
        // .numerosImparesMayoresQuePorNiveles(agraiz, 20);
        // result.comenzar();
        // while (!result.fin()) {
        // System.out.println(result.proximo());
        // }
        System.out.println(agraiz.ancho());
    }
}
