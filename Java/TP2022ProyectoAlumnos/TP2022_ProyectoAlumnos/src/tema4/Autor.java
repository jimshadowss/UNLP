/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.tema4;

/**
 *
 * @author igarz
 */
public class Autor {
    private String nombre;
    private String biografia;
    private String origen;

    public Autor(String nombre, String biografia, String origen) {
        this.nombre = nombre;
        this.biografia = biografia;
        this.origen = origen;
    }

    public Autor() {
    }

    public String getNombre() {
        return nombre;
    }

    public String getBiografia() {
        return biografia;
    }

    public String getOrigen() {
        return origen;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public void setBiografia(String biografia) {
        this.biografia = biografia;
    }

    public void setOrigen(String origen) {
        this.origen = origen;
    }

    @Override
    public String toString() {
        String aux = "Autor: " + this.getNombre() + ", biografia: " + this.getBiografia() + ", origen: "
                + this.getOrigen();
        return aux;
    }

}
