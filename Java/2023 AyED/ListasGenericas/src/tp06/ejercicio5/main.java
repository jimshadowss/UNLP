package tp06.ejercicio5;

import tp02.ejercicio2.*;
import tp06.ejercicio3.Vertice;
import tp06.ejercicio3.VerticeImplMatrizAdy;
import tp06.ejercicio3.GrafoImplMatrizAdy;

public class main {
        public static void main(String[] args) {
                GrafoImplMatrizAdy<String> gr = new GrafoImplMatrizAdy<String>(10);
                VerticeImplMatrizAdy<String> v1 = new VerticeImplMatrizAdy<String>("1");
                VerticeImplMatrizAdy<String> v2 = new VerticeImplMatrizAdy<String>("2");
                VerticeImplMatrizAdy<String> v3 = new VerticeImplMatrizAdy<String>("3");
                VerticeImplMatrizAdy<String> v4 = new VerticeImplMatrizAdy<String>("4");
                VerticeImplMatrizAdy<String> v5 = new VerticeImplMatrizAdy<String>("5");
                VerticeImplMatrizAdy<String> v6 = new VerticeImplMatrizAdy<String>("6");
                VerticeImplMatrizAdy<String> v7 = new VerticeImplMatrizAdy<String>("7");
                VerticeImplMatrizAdy<String> v8 = new VerticeImplMatrizAdy<String>("8");

                gr.agregarVertice((Vertice<String>) v1);
                gr.agregarVertice((Vertice<String>) v2);
                gr.agregarVertice((Vertice<String>) v3);
                gr.agregarVertice((Vertice<String>) v4);
                gr.agregarVertice((Vertice<String>) v5);
                gr.agregarVertice((Vertice<String>) v6);
                gr.agregarVertice((Vertice<String>) v7);
                gr.agregarVertice((Vertice<String>) v8);
                gr.conectar((Vertice<String>) v4, (Vertice<String>) v1, 50);
                gr.conectar((Vertice<String>) v4, (Vertice<String>) v2, 5);
                gr.conectar((Vertice<String>) v2, (Vertice<String>) v1, 20);
                gr.conectar((Vertice<String>) v1, (Vertice<String>) v3, 10);
                gr.conectar((Vertice<String>) v2, (Vertice<String>) v5, 45);
                gr.conectar((Vertice<String>) v3, (Vertice<String>) v5, 40);
                gr.conectar((Vertice<String>) v3, (Vertice<String>) v8, 5);
                gr.conectar((Vertice<String>) v8, (Vertice<String>) v6, 5);
                gr.conectar((Vertice<String>) v5, (Vertice<String>) v7, 40);
                gr.conectar((Vertice<String>) v6, (Vertice<String>) v7, 5);
                MapaCiudades<String> mapa = new MapaCiudades<String>(gr);
                ListaGenerica<String> lista = mapa.caminoConMenorCargaDeCombustible(v4.dato(), v7.dato(), 50);
                lista.comenzar();

                System.out.println(lista);

                System.exit(0);

        }
}
