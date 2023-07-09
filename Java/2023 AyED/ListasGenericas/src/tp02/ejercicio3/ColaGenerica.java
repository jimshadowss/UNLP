package tp02.ejercicio3;

import tp02.ejercicio2.*;

public class ColaGenerica<T> {
    private ListaEnlazadaGenerica<T> l = new ListaEnlazadaGenerica<T>();

    public void encolar(T elem) {
        l.agregarFinal(elem);
    }

    public T desencolar() {
        T elem = l.elemento(1);
        l.eliminarEn(1);
        return elem;
    }

    public T tope() {
        T elem = l.elemento(1);
        return elem;
    }

    public boolean esVacia() {
        boolean aux = l.esVacia();
        return aux;
    }
}
