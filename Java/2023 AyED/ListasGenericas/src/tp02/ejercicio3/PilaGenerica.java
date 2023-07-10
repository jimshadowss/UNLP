package tp02.ejercicio3;

import tp02.ejercicio2.*;

public class PilaGenerica<T> {
    private ListaEnlazadaGenerica<T> l = new ListaEnlazadaGenerica<>();

    public void apilar(T elem) {
        l.agregarFinal(elem);
    }

    public T desapilar() {
        T aux = l.elemento(l.tamanio());
        l.eliminarEn(l.tamanio());
        return aux;
    }

    public T tope() {
        T aux = l.elemento(l.tamanio());
        return aux;
    }

    public boolean esVacia() {
        boolean aux = l.esVacia();
        return aux;
    }
}
