package tp04.ejercicio3;

import tp02.ejercicio2.*;
import tp02.ejercicio3.ColaGenerica;
import tp04.ejercicio1.*;

public class RecorridosAG {

    public static ListaGenerica<Integer> numerosImparesMayoresQuePreOrden(ArbolGeneral<Integer> a, Integer n) {
        ListaGenerica<Integer> l = new ListaEnlazadaGenerica<>();
        a.getHijos().comenzar();
        if (a.getDato() % 2 != 0 && a.getDato() > n) {// si es impar la raiz, agrego a la lista
            l.agregarFinal(a.getDato());
        }
        while (!a.getHijos().fin()) {// recorro la lista de hijos del arbol actual
            ListaGenerica<Integer> aux = new ListaEnlazadaGenerica<>();
            aux = numerosImparesMayoresQuePreOrden(a.getHijos().proximo(), n);
            while (!aux.fin()) { // avanza en la lista recibida y agrega si es impar
                l.agregarFinal(aux.proximo());
            }
        }
        return l;
    }

    public static ListaGenerica<Integer> numerosImparesMayoresQueInOrden(ArbolGeneral<Integer> a, Integer n) {
        ListaGenerica<Integer> l = new ListaEnlazadaGenerica<>();
        a.getHijos().comenzar();
        ListaGenerica<Integer> aux = new ListaEnlazadaGenerica<Integer>();
        if (!a.getHijos().fin()) {
            aux = numerosImparesMayoresQueInOrden(a.getHijos().proximo(), n);
            while (!aux.fin()) {
                l.agregarFinal(aux.proximo());
            }
        }
        if (a.getDato() % 2 != 0 && a.getDato() > n) {// si es impar la raiz, agrego a la lista
            l.agregarFinal(a.getDato());
        }
        while (!a.getHijos().fin() && !a.getHijos().esVacia()) {// recorro la lista de hijos del arbol actual
            aux = numerosImparesMayoresQueInOrden(a.getHijos().proximo(), n);
            while (!aux.fin()) { // avanza en la lista recibida y agrega si es impar
                l.agregarFinal(aux.proximo());
            }
        }
        return l;
    }

    public static ListaGenerica<Integer> numerosImparesMayoresQuePosOrden(ArbolGeneral<Integer> a, Integer n) {
        ListaGenerica<Integer> l = new ListaEnlazadaGenerica<>();
        a.getHijos().comenzar();

        while (!a.getHijos().fin()) {// recorro la lista de hijos del arbol actual
            ListaGenerica<Integer> aux = numerosImparesMayoresQuePosOrden(a.getHijos().proximo(), n);
            while (!aux.fin()) { // avanza en la lista recibida y agrega si es impar
                l.agregarFinal(aux.proximo());
            }
        }
        if (a.getDato() % 2 != 0 && a.getDato() > n) {// si es impar la raiz, agrego a la lista
            l.agregarFinal(a.getDato());
        }
        return l;
    }

    public static ListaGenerica<Integer> numerosImparesMayoresQuePorNiveles(ArbolGeneral<Integer> a, Integer n) {
        ColaGenerica<ArbolGeneral<Integer>> cola = new ColaGenerica<>();
        cola.encolar(a);
        ListaGenerica<Integer> l = new ListaEnlazadaGenerica<Integer>();
        while (!cola.esVacia()) {
            ArbolGeneral<Integer> aux = cola.desencolar();
            if (aux.getDato() % 2 != 0 && aux.getDato() > n) {
                l.agregarFinal(aux.getDato());
            }
            if (aux.tieneHijos()) {
                aux.getHijos().comenzar();
                while (!aux.getHijos().fin()) {
                    cola.encolar(aux.getHijos().proximo());
                }
            }

        }
        return l;
    }
}
