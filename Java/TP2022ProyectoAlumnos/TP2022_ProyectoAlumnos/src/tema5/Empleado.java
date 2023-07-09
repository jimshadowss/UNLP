/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.tema5;

/**
 *
 * @author igarz
 */
public abstract class Empleado {
    private String nombre;
    private double sueldo;
    private double antiguedad;

    public Empleado(String nombre, double sueldo, double antiguedad) {
        this.antiguedad = antiguedad;
        this.nombre = nombre;
        this.sueldo = sueldo;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public void setSueldo(double sueldo) {
        this.sueldo = sueldo;
    }

    public void setAnt(double antiguedad) {
        this.antiguedad = antiguedad;
    }

    public String getNombre() {
        return nombre;
    }

    public double getSueldo() {
        return sueldo;
    }

    public double getAnt() {
        return antiguedad;
    }

    @Override
    public String toString() {
        String aux = "Nombre: " + this.getNombre() + " Sueldo: " + this.calcularSueldoACobrar() + " Antiguedad: "
                + this.getAnt();
        return aux;
    }

    public abstract double calcularEfectividad();

    public abstract double calcularSueldoACobrar();
}
