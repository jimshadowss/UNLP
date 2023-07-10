/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.tema5;

/**
 *
 * @author igarz
 */
public class Circulo extends Figura {
    private double radio;

    public Circulo(double radio, String CR, String CL) {
        super(CR, CL);
        this.radio = radio;
    }

    public void setRadio(double radio) {
        this.radio = radio;
    }

    public double getRadio() {
        return this.radio;
    }

    @Override
    public String toString() {
        String aux = super.toString() + " Radio: " + this.getRadio();
        return aux;
    }

    @Override
    public double calcularPerimetro() {
        double aux = 2 * Math.PI * this.getRadio();
        return aux;
    }

    @Override
    public double calcularArea() {
        double aux = Math.PI * this.getRadio() * this.getRadio();
        return aux;
    }
}
