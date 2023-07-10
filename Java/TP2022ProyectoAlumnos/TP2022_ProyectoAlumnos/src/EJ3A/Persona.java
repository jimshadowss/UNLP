/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.EJ3A;

/**
 *
 * @author igarz
 */
public class Persona {
    private String nombre;
    private int dni;
    private int edad;
    public Persona(String nombre, int dni, int edad){
        this.nombre=nombre;
        this.dni=dni;
        this.edad=edad;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public void setDni(int dni) {
        this.dni = dni;
    }

    public void setEdad(int edad) {
        this.edad = edad;
    }

    public String getNombre() {
        return nombre;
    }

    public int getDni() {
        return dni;
    }

    public int getEdad() {
        return edad;
    }
    @Override
    public String toString(){
        String aux="Mi nombre es "+this.getNombre()+", mi DNI es "+this.getDni()+" y tengo "+this.getEdad()+" anios.";
        return aux;
    }
    
}
