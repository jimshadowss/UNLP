package tp04.ejercicio1;

import tp02.ejercicio2.ListaEnlazadaGenerica;
import tp02.ejercicio2.ListaGenerica;

public class Practica {
    private ArbolGeneral<Integer> ag;

    public void setAg(ArbolGeneral<Integer> ag) {
        this.ag = ag;
    }

    public ArbolGeneral<Integer> getAg() {
        return this.ag;
    }

    public ListaEnlazadaGenerica<ArbolGeneral<Integer>> resp(int num) {
        ListaEnlazadaGenerica<ArbolGeneral<Integer>> lista = new ListaEnlazadaGenerica<ArbolGeneral<Integer>>();
        if (!this.getAg().esVacio()) {
            this.buscar(this.getAg(), lista, num);
        }
        return lista;
    }

    private boolean buscar(ArbolGeneral<Integer> ag, ListaEnlazadaGenerica<ArbolGeneral<Integer>> lista, int num) {
        boolean es = false;
        if (ag.getDato() % num == 0) {
            if (ag.tieneHijos()) {
                ag.getHijos().comenzar();
                while (!ag.getHijos().fin() && !es) {
                    if (this.buscar(ag.getHijos().proximo(), lista, num)) {
                        es = true;
                        lista.agregarInicio(ag);
                    }
                }
            } else {
                es = true;
                lista.agregarFinal(ag);
            }
        }
        return es;
    }
}
