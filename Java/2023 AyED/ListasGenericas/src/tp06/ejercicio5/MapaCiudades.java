package tp06.ejercicio5;

import org.omg.CORBA.BooleanHolder;

import tp02.ejercicio2.*;
import tp02.ejercicio3.ColaGenerica;
import tp06.ejercicio3.*;

public class MapaCiudades<String> {
    Grafo<String> gr;

    public MapaCiudades(Grafo<String> gr) {
        this.gr = gr;
    }

    public Vertice<String> getVert(String c1) {
        ListaGenerica<Vertice<String>> lista = this.gr.listaDeVertices();
        lista.comenzar();
        Vertice<String> aux = lista.proximo();
        while (!lista.fin() && !aux.dato().equals(c1)) {
            aux = lista.proximo();
        }
        return aux;
    }

    public ListaGenerica<String> devolverCamino(String c1, String c2) {
        ListaEnlazadaGenerica<String> lista = new ListaEnlazadaGenerica<String>();
        boolean[] marcados = new boolean[gr.listaDeVertices().tamanio() + 1];
        Vertice<String> vert = this.getVert(c1);
        this.dfs(vert, marcados, lista, c2);
        return lista;
    }

    public boolean dfs(Vertice<String> vert, boolean[] marcados, ListaEnlazadaGenerica<String> lista, String c2) {
        boolean encontre = false;
        marcados[vert.getPosicion()] = true;
        lista.agregarFinal(vert.dato());
        if (vert.dato().equals(c2))
            encontre = true;
        ListaGenerica<Arista<String>> adys = gr.listaDeAdyacentes(vert);
        adys.comenzar();
        while (!adys.fin() && !encontre) {
            Vertice<String> destino = adys.proximo().verticeDestino();
            if (!encontre && !marcados[destino.getPosicion()]) {
                encontre = this.dfs(destino, marcados, lista, c2);
            }
        }
        if (!encontre) {
            lista.eliminarEn(lista.tamanio());
        }
        return encontre;
    }

    /*
     *  El método devolverCaminoExceptuando (String ciudad1, String ciudad2,
     * ListaGenerica<String> ciudades): ListaGenerica<String> // Retorna la lista de
     * ciudades que
     * forman un camino desde ciudad1 a ciudad2, sin pasar por las ciudades que
     * están contenidas en la lista
     * ciudades pasada por parámetro, si no existe camino retorna la lista vacía.
     * (Sin tener en cuenta el
     * combustible).
     */

    public ListaGenerica<String> devolverCaminoExceptuando(String ciudad1, String ciudad2,
            ListaGenerica<String> ciudades) {
        ListaEnlazadaGenerica<String> lista = new ListaEnlazadaGenerica<String>();
        boolean[] marcados = new boolean[gr.listaDeVertices().tamanio() + 1];
        ciudades.comenzar();
        while (!ciudades.fin()) {
            Vertice<String> vert = this.getVert(ciudades.proximo());
            marcados[vert.getPosicion()] = true;
        }
        Vertice<String> vert = this.getVert(ciudad1);
        this.dfs(vert, marcados, lista, ciudad2);
        return lista;
    }

    /*
     *  El método caminoMasCorto(String ciudad1, String ciudad2):
     * ListaGenerica<String> //
     * Retorna la lista de ciudades que forman el camino más corto para llegar de
     * ciudad1 a ciudad2, si no
     * existe camino retorna la lista vacía. (Las rutas poseen la distancia). (Sin
     * tener en cuenta el
     * combustible).
     */

    public ListaGenerica<String> caminoMasCorto(String ciudad1, String ciudad2) {
        ListaEnlazadaGenerica<String> lista = new ListaEnlazadaGenerica<String>();
        ListaEnlazadaGenerica<String> caminoCorto = new ListaEnlazadaGenerica<String>();

        boolean[] marcados = new boolean[gr.listaDeVertices().tamanio() + 1];
        Vertice<String> vert = this.getVert(ciudad1);
        marcados[vert.getPosicion()] = true;
        lista.agregarFinal(vert.dato());
        boolean encontre = false;
        ListaGenerica<Arista<String>> adys = gr.listaDeAdyacentes(vert);
        adys.comenzar();
        while (!adys.fin()) {
            Vertice<String> destino = adys.proximo().verticeDestino();
            if (!marcados[destino.getPosicion()]) {
                encontre = this.dfs(destino, marcados, lista, ciudad2);
                if (caminoCorto.esVacia() || lista.tamanio() < caminoCorto.tamanio()) {
                    caminoCorto = (ListaEnlazadaGenerica<String>) lista.clonar();
                }
            }

        }
        if (!encontre) {
            lista.eliminar(ciudad1);
        }
        return lista;
    }

    /*
     *  El método caminoSinCargarCombustible(String ciudad1, String ciudad2, int
     * tanqueAuto):
     * ListaGenerica<String> // Retorna la lista de ciudades que forman un camino
     * para llegar de ciudad1
     * a ciudad2. El auto no debe quedarse sin combustible y no puede cargar. Si no
     * existe camino retorna la
     * lista vacía.
     */
    public boolean dfsPesado(Vertice<String> vert, boolean[] marcados, ListaEnlazadaGenerica<String> lista, String c2,
            int tanque) {
        boolean encontre = false;
        marcados[vert.getPosicion()] = true;
        lista.agregarFinal(vert.dato());
        if (vert.dato().equals(c2))
            encontre = true;
        ListaGenerica<Arista<String>> adys = gr.listaDeAdyacentes(vert);
        adys.comenzar();
        while (!adys.fin() && !encontre) {
            Arista<String> destino = adys.proximo();
            int queda = tanque - destino.peso();
            System.out.println(queda);
            if (!encontre && !marcados[destino.verticeDestino().getPosicion()] && queda > 0) {
                encontre = this.dfsPesado(destino.verticeDestino(), marcados, lista, c2, queda);
            }
        }
        if (!encontre) {
            lista.eliminarEn(lista.tamanio());
        }
        return encontre;
    }

    public ListaGenerica<String> caminoSinCargarCombustible(String ciudad1, String ciudad2, int tanqueAuto) {
        ListaEnlazadaGenerica<String> lista = new ListaEnlazadaGenerica<String>();
        boolean[] marcados = new boolean[this.gr.listaDeVertices().tamanio() + 1];
        Vertice<String> vert = this.getVert(ciudad1);
        this.dfsPesado(vert, marcados, lista, ciudad2, tanqueAuto);
        return lista;
    }

    private int dfsMenorCarga(Vertice<String> vert, boolean[] marcados, ListaGenerica<String> lista,
            String c2, int tanque, final int capacidad, int carga, int cargaMin) {
        int cargado = -1;
        lista.agregarFinal(vert.dato());
        if (vert.dato().equals(c2)) {
            System.out.println("Encontre con cargas " + carga);
            lista.comenzar();
            while (!lista.fin()) {
                System.out.println("valor: " + lista.proximo());
            }
            cargado = carga;
        } else {
            ListaGenerica<Arista<String>> ady = gr.listaDeAdyacentes(vert);
            ady.comenzar();
            while (!ady.fin()) {
                Arista<String> destino = ady.proximo();
                if (!marcados[destino.verticeDestino().getPosicion()]) {
                    int aux = tanque - destino.peso();

                    if (aux <= 0) {
                        aux = capacidad - destino.peso();
                        carga += 1;
                    }
                    if (carga < cargaMin && cargado != -1) {
                        cargado = this.dfsMenorCarga(destino.verticeDestino(), marcados, lista, c2, aux,
                                capacidad,
                                carga,
                                cargaMin);
                    }
                }

            }

        }

        if (cargado == -1) {
            marcados[vert.getPosicion()] = false;
            lista.eliminarEn(lista.tamanio());
        }

        return cargado;
    }

    private boolean dfsMenorCarga2(Vertice<String> vert, boolean[] marcados, ListaGenerica<String> lista,
            String c2, int tanque, final int capacidad, int carga, int cargaMin, Carga cargado) {
        boolean encontre = false;
        if (vert.dato().equals(c2)) {
            cargado.c = carga;
        } else {
            ListaGenerica<Arista<String>> ady = gr.listaDeAdyacentes(vert);
            ady.comenzar();
            while (!ady.fin()) {
                Arista<String> destino = ady.proximo();
                if (!marcados[destino.verticeDestino().getPosicion()]) {
                    int aux = tanque - destino.peso();
                    if (aux <= 0) {
                        aux = capacidad - destino.peso();
                        carga += 1;
                    }
                    if (carga < cargaMin && !encontre) {
                        encontre = this.dfsMenorCarga2(destino.verticeDestino(), marcados, lista, c2, aux,
                                capacidad,
                                carga,
                                cargaMin, cargado);
                    }
                }

            }

        }
        if (!encontre) {
            marcados[vert.getPosicion()] = false;
            lista.eliminarEn(lista.tamanio());
        }

        return encontre;
    }
    /*
     * El método caminoConMenorCargaDeCombustible (String ciudad1, String ciudad2,
     * int
     * tanqueAuto): ListaGenerica<String> // Retorna la lista de ciudades que forman
     * un camino para
     * llegar de ciudad1 a ciudad2 teniendo en cuenta que el auto debe cargar la
     * menor cantidad de veces. El
     * auto no se debe quedar sin combustible en medio de una ruta, además puede
     * completar su tanque al
     * llegar a cualquier ciudad. Si no existe camino retorna la lista vacía.
     */

    public ListaGenerica<String> caminoConMenorCargaDeCombustible(String ciudad1, String ciudad2, int tanqueAuto) {
        final int capacidad = tanqueAuto;
        int menorCargas = Integer.MAX_VALUE;
        int cargas;
        ListaGenerica<String> lista = new ListaEnlazadaGenerica<String>();
        ListaGenerica<String> caminoCorto = new ListaEnlazadaGenerica<String>();
        boolean[] marcados = new boolean[this.gr.listaDeVertices().tamanio() + 1];
        Vertice<String> vert = this.getVert(ciudad1);
        marcados[vert.getPosicion()] = true;
        ListaGenerica<Arista<String>> ady = gr.listaDeAdyacentes(vert);
        ady.comenzar();
        while (!ady.fin()) {
            Arista<String> destino = ady.proximo();
            cargas = this.dfsMenorCarga(destino.verticeDestino(), marcados, lista, ciudad2, tanqueAuto - destino.peso(),
                    capacidad, 0,
                    menorCargas);
            lista.comenzar();
            System.out.println("cargas son " + cargas);
            while (!lista.fin()) {
                System.out.println(lista.proximo());
            }
            if (cargas < menorCargas && cargas >= 0) {
                this.clonarListas(lista, caminoCorto);
                menorCargas = cargas;
            }
            lista = new ListaEnlazadaGenerica<String>();

            for (int i = 0; i < gr.listaDeVertices().tamanio() + 1; i++) {
                marcados[i] = false;
            }

        }
        caminoCorto.agregarEn(vert.dato(), 1);
        return caminoCorto;
    }

    private void clonarListas(ListaGenerica<String> origen, ListaGenerica<String> destino) {
        destino.comenzar();
        while (!destino.fin()) {
            destino.eliminar(destino.proximo());
        }
        origen.comenzar();
        ;
        while (!origen.fin()) {
            destino.agregarFinal(origen.proximo());
        }
    }

}
