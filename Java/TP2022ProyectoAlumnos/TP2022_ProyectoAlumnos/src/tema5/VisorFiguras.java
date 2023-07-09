/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.tema5;

/**
 *
 * @author igarz
 */
public class VisorFiguras {
    private int mostradas;
    private Figura[] vector;
    private int dimL = -1;

    public VisorFiguras() {
        mostradas = 0;
        vector = new Figura[5];
    }

    public int getDimL() {
        return dimL;
    }

    public void guardar(Figura f) {
        dimL++;
        vector[dimL] = f;
        System.out.println("Guardado");
    }

    public boolean quedaEspacio() {
        if (dimL < 4) {
            return true;
        } else {
            return false;
        }
    }

    public void mostrar(Figura f) {
        System.out.println(f.toString());
        mostradas++;
    }

    public int getMostradas() {
        return mostradas;
    }

    public Figura[] getGuardadas() {
        return vector;
    }
}
