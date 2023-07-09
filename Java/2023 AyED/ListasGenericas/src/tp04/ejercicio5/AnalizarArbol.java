package tp04.ejercicio5;

import tp04.ejercicio1.*;
import tp02.ejercicio3.*;

public class AnalizarArbol {

    public int devolverMaximoPromedio(ArbolGeneral<AreaEmpresa> ag) {
        int res = 0;
        int promact = 0;
        int cont = 0;
        ColaGenerica<ArbolGeneral<AreaEmpresa>> cola = new ColaGenerica<>();
        ArbolGeneral<AreaEmpresa> aux = new ArbolGeneral<AreaEmpresa>(ag.getDato());
        if (this != null) {
            cola.encolar(ag);
            cola.encolar(null);
        }
        while (!cola.esVacia()) {
            aux = cola.desencolar();
            if (aux == null) {
                if (!cola.esVacia()) {
                    cola.encolar(null);
                }
                if (cont != 0) {
                    res = Math.max(res, promact / cont);
                }
                promact = 0;
                cont = 0;
            } else {
                promact += aux.getDato().getLatencia();
                cont += 1;
                if (aux.tieneHijos()) {
                    aux.getHijos().comenzar();
                    while (!aux.getHijos().fin()) {
                        cola.encolar(aux.getHijos().proximo());
                    }
                }
            }
        }

        return res;
    }

}
