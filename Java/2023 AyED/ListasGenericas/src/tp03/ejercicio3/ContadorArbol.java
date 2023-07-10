package tp03.ejercicio3;

import tp04.ejercicio1.*;
import tp02.ejercicio3.*;
import tp02.ejercicio2.*;
import tp03.ejercicio1y2.ArbolBinario;

public class ContadorArbol {
    private ArbolBinario<Integer> ab;

    public ContadorArbol(ArbolBinario<Integer> ab) {
        this.ab = ab;
    }

    public ColaGenerica<Integer> numerosPares() {
        ColaGenerica<Integer> cola = new ColaGenerica<>();
        this.recorrerInOrden(ab, cola); // Aca solo se usa uno de los dos
        this.recorrerPosOrden(ab, cola); // dejé los dos sin comentar para que no chillen warnings
        return cola;
    }

    private ColaGenerica<Integer> recorrerInOrden(ArbolBinario<Integer> ab, ColaGenerica<Integer> cola) {
        if (!ab.esVacio()) {
            if (ab.tieneHijoIzquierdo()) {
                recorrerInOrden(ab.getHijoIzquierdo(), cola);
            }
            if (ab.getDato() % 2 == 0) {
                cola.encolar(ab.getDato());
            }
            if (ab.tieneHijoDerecho()) {
                recorrerInOrden(ab.getHijoDerecho(), cola);
            }
        }
        return cola;
    }

    private void recorrerPosOrden(ArbolBinario<Integer> ab, ColaGenerica<Integer> cola) {
        if (!ab.esVacio()) {
            if (ab.tieneHijoIzquierdo()) {
                recorrerPosOrden(ab.getHijoIzquierdo(), cola);
            }
            if (ab.tieneHijoDerecho()) {
                recorrerPosOrden(ab.getHijoDerecho(), cola);
            }
            if (ab.getDato() % 2 == 0) {
                cola.encolar(ab.getDato());
            }
        }
    }

    public ListaGenerica<Integer> elementosNoRepetidos(ArbolGeneral<Integer> ag, Integer n) {
        ListaGenerica<Integer> l = new ListaEnlazadaGenerica<>();

        return l;
    }
}