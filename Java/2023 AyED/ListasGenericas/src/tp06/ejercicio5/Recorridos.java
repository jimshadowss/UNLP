package tp06.ejercicio5;

import tp02.ejercicio2.*;
import tp02.ejercicio3.ColaGenerica;
import tp06.ejercicio3.*;

public class Recorridos<T> {
    public ListaGenerica<Vertice<T>> dfs(Grafo<T> gr) {
        ListaGenerica<Vertice<T>> lista = new ListaEnlazadaGenerica<Vertice<T>>();
        boolean[] marcados = new boolean[gr.listaDeVertices().tamanio() + 1];
        gr.listaDeVertices().comenzar();
        for (int i = 0; i < gr.listaDeVertices().tamanio(); i++) {
            marcados[i] = false;
        }
        this.dfsShoot(gr, gr.listaDeVertices().proximo(), marcados, lista);
        return lista;
    }

    private boolean dfsShoot(Grafo<T> gr, Vertice<T> vr, boolean[] marcados, ListaGenerica<Vertice<T>> lista) {
        boolean es = false;
        marcados[vr.getPosicion()] = true;
        lista.agregarFinal(vr);
        ListaGenerica<Arista<T>> aux = gr.listaDeAdyacentes(vr);
        aux.comenzar();
        while (!aux.fin()) {
            Vertice<T> prox = aux.proximo().verticeDestino();
            if (!marcados[prox.getPosicion()]) {
                this.dfsShoot(gr, prox, marcados, lista);
            }
        }
        return es;
    }

    public ListaGenerica<Vertice<T>> bfs(Grafo<T> grafo) {
        ListaGenerica<Vertice<T>> lista = new ListaEnlazadaGenerica<Vertice<T>>();
        boolean[] marcados = new boolean[grafo.listaDeVertices().tamanio() + 1];
        grafo.listaDeVertices().comenzar();
        for (int i = 0; i < marcados.length; i++) {
            marcados[i] = false;
        }
        grafo.listaDeVertices().comenzar();
        while (!grafo.listaDeVertices().fin()) {
            Vertice<T> aux = grafo.listaDeVertices().proximo();
            if (!marcados[aux.getPosicion()]) {
                this.bfsShoot(grafo, aux, marcados, lista);
            }
        }
        return lista;
    }

    private void bfsShoot(Grafo<T> gr, Vertice<T> vr, boolean[] marcados, ListaGenerica<Vertice<T>> lista) {
        ListaGenerica<Arista<T>> aux;
        Vertice<T> vert;
        Vertice<T> destino;
        ColaGenerica<Vertice<T>> cola = new ColaGenerica<Vertice<T>>();
        cola.encolar(vr);
        while (!cola.esVacia()) {
            vert = cola.desencolar();
            marcados[vert.getPosicion()] = true;
            lista.agregarFinal(vert);
            aux = gr.listaDeAdyacentes(vert);
            aux.comenzar();
            while (!aux.fin()) {
                destino = aux.proximo().verticeDestino();
                if (!marcados[destino.getPosicion()]) {
                    cola.encolar(destino);
                }
            }
        }
    }

}
