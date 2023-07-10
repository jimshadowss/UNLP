/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.tema5;

/**
 *
 * @author igarz
 */
public class Entrenador extends Empleado {
    private int camps;

    public Entrenador(String nombre, double sueldo, double antiguedad, int camps) {
        super(nombre, sueldo, antiguedad);
        this.camps = camps;
    }

    public void setCamps(int camps) {
        this.camps = camps;
    }

    public int getCamps() {
        return camps;
    }

    @Override
    public double calcularEfectividad() {
        double aux = (double) this.getCamps() / this.getAnt();
        return aux;
    }

    @Override
    public double calcularSueldoACobrar() {
        double aux = this.getSueldo();
        if ((this.getCamps() >= 1) && (this.getCamps() < 5)) {
            aux += 5000;
        } else if ((this.getCamps() >= 5) && (this.getCamps() < 11)) {
            aux += 30000;
        } else if (this.getCamps() >= 11) {
            aux += 50000;
        }
        return aux;
    }
}
