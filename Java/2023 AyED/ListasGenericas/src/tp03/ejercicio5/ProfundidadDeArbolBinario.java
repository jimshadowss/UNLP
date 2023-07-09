package tp03.ejercicio5;

import tp03.ejercicio1y2.ArbolBinario;
import tp02.ejercicio3.*;

public class ProfundidadDeArbolBinario {
    private ArbolBinario<Integer> ab = new ArbolBinario<>();

    public void setAb(ArbolBinario<Integer> ab) {
        this.ab = ab;
    }

    public int sumaElementosProfundidad(int p) {
        int suma = 0;
        int cont = 0;
        if (p == 0) {
            return ab.getDato();
        } else if (p < 0) {
            return Integer.MIN_VALUE;
        } else {
            ArbolBinario<Integer> aux = new ArbolBinario<>();
            ColaGenerica<ArbolBinario<Integer>> cola = new ColaGenerica<>();
            cola.encolar(ab);
            cola.encolar(null);
            while (!cola.esVacia()) {
                aux = cola.desencolar();
                if (aux != null) {
                    if (cont == p) {
                        suma += aux.getDato();
                    }
                    if (aux.tieneHijoIzquierdo()) {
                        cola.encolar(aux.getHijoIzquierdo());
                    }
                    if (aux.tieneHijoDerecho()) {
                        cola.encolar(aux.getHijoDerecho());
                    }
                    cola.encolar(null);
                } else {
                    cont++;
                }
            }
            return suma;
        }
    }
}
