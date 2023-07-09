package tp04.ejercicio1;

import tp02.ejercicio2.ListaEnlazadaGenerica;

public class main {
    public static void main(String[] args) {

        ArbolGeneral<Integer> a2 = new ArbolGeneral<Integer>(45);
        ArbolGeneral<Integer> a3 = new ArbolGeneral<Integer>(46);
        ArbolGeneral<Integer> a4 = new ArbolGeneral<Integer>(77, null);
        ListaEnlazadaGenerica<ArbolGeneral<Integer>> lista = new ListaEnlazadaGenerica<>();
        lista.agregarFinal(a4);
        lista.agregarFinal(a3);
        lista.agregarFinal(a2);
        ArbolGeneral<Integer> a1 = new ArbolGeneral<Integer>(2, lista);
        Practica pr = new Practica();
        pr.setAg(a1);
        ListaEnlazadaGenerica<ArbolGeneral<Integer>> l2 = pr.resp(2);
        l2.comenzar();
        while (!l2.fin()) {
            System.out.println(l2.proximo().getDato());
        }
    }
}
