package tp03.ejercicio4;

//Lo resuelvo usando recorrido en profundidad

import tp03.ejercicio1y2.*;

public class RedBinariaLlena {
    private ArbolBinario<Integer> ab = new ArbolBinario<>();
    private int ret = 0;

    public void setAb(ArbolBinario<Integer> ab) {
        this.ab = ab;
    }

    public int retardoReenvio() {
        int act = 0;
        this.recorrer(ab, act);
        return ret;
    }

    private void recorrer(ArbolBinario<Integer> ab, int act) {
        if (!ab.esVacio()) {
            act += ab.getDato();
            if (ab.tieneHijoIzquierdo()) {
                recorrer(ab.getHijoIzquierdo(), act);
            }
            if (ab.tieneHijoDerecho()) {
                recorrer(ab.getHijoDerecho(), act);
            }
        }
        if (act > this.ret) {
            this.ret = act;
        }
    }

}
