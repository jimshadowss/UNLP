/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.tema5;

/**
 *
 * @author igarz
 */
public class Jugador extends Empleado {
    private int partidos;
    private int goles;

    public Jugador(String nombre, double sueldo, double antiguedad, int partidos, int goles) {
        super(nombre, sueldo, antiguedad);
        this.partidos = partidos;
        this.goles = goles;
    }

    public void setPartidos(int partidos) {
        this.partidos = partidos;
    }

    public void setGoles(int goles) {
        this.goles = goles;
    }

    public int getPartidos() {
        return partidos;
    }

    public int getGoles() {
        return goles;
    }

    @Override
    public double calcularEfectividad() {
        double aux = (double) this.getGoles() / this.getPartidos();
        return aux;
    }

    @Override
    public double calcularSueldoACobrar() {
        double aux = this.getSueldo();
        if (this.calcularEfectividad() > 0.5) {
            aux = aux * 2;
        }
        return aux;
    }
}
