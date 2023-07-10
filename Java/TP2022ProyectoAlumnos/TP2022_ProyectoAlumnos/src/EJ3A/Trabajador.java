/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.EJ3A;

/**
 *
 * @author igarz
 */
public class Trabajador extends Persona {
    private String tarea;

    public Trabajador(String nombre, int dni, int edad, String tarea) {
        super(nombre, dni, edad);
        this.tarea = tarea;
    }

    public void setTarea(String tarea) {
        this.tarea = tarea;
    }

    public String getTarea() {
        return tarea;
    }

    @Override
    public String toString() {
        String aux = super.toString() + " Soy " + this.getTarea() + ".";
        return aux;
    }
}
