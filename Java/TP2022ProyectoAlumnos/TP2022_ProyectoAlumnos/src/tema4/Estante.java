/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.tema4;

/**
 *
 * @author igarz
 */
public class Estante {
    private Libro[] estante = new Libro[20];
    private int dimL = -1;

    public Estante(Libro[] estante, int dimL) {
        this.estante = estante;
        this.dimL = dimL;
    }

    public Estante() {
        int i;
        for (i = 0; i < 20; i++) {
            Libro libro = new Libro();
            estante[i] = libro;
        }

    }

    public Libro[] getEstante() {
        return estante;
    }

    public void iniciar() {
        int i;
        for (i = 0; i < 20; i++) {
            Autor autor = new Autor();
            estante[i].setPrimerAutor(autor);
        }
    }

    public void setLibro(Libro libro) {
        dimL++;
        estante[dimL] = libro;
    }

    public Libro getLibro(int i) {
        return estante[i];
    }

    public int getAlmacenados() {
        return dimL;
    }

    public boolean getSiLleno() {
        if (dimL == 19) {
            return true;
        } else {
            return false;
        }
    }

    public Libro getXTitulo(String titulo) {
        int i = 0;
        while ((i < dimL) && (estante[i].getTitulo().equals(titulo) != true)) {
            i++;
        }
        if (estante[i].getTitulo().equals(titulo)) {
            return estante[i];
        } else {
            return null;
        }

    }

    @Override
    public String toString() {
        int i = 0;
        String aux = "";
        while (i < dimL) {
            aux = aux + estante[i].toString();
            i++;
        }
        return aux;
    }

}
