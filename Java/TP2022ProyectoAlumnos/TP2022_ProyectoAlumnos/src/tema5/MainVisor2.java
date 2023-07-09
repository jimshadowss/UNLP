/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Main.java to edit this template
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.tema5;

import PaqueteLectura.Lector;

/**
 *
 * @author igarz
 */
public class MainVisor2 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        double lado, base, altura;
        String CR, CL;
        VisorFiguras vs = new VisorFiguras();
        System.out.println("Cuadrado 1:");
        System.out.println("Ingrese lado:");
        lado = Lector.leerDouble();
        System.out.println("Color de relleno:");
        CR = Lector.leerString();
        System.out.println("Color de linea:");
        CL = Lector.leerString();
        Cuadrado c1 = new Cuadrado(lado, CR, CL);
        if (vs.quedaEspacio()) {
            vs.guardar(c1);
        }
        System.out.println("Cuadrado 2:");
        System.out.println("Ingrese lado:");
        lado = Lector.leerDouble();
        System.out.println("Color de relleno:");
        CR = Lector.leerString();
        System.out.println("Color de linea:");
        CL = Lector.leerString();
        Cuadrado c2 = new Cuadrado(lado, CR, CL);
        if (vs.quedaEspacio()) {
            vs.guardar(c2);
        }
        System.out.println("Rectangulo:");
        System.out.println("Ingrese base:");
        base = Lector.leerDouble();
        System.out.println("Ingrese altura:");
        altura = Lector.leerDouble();
        System.out.println("Color de relleno:");
        CR = Lector.leerString();
        System.out.println("Color de linea:");
        CL = Lector.leerString();
        Rectangulo r1 = new Rectangulo(base, altura, CR, CL);
        if (vs.quedaEspacio()) {
            vs.guardar(r1);
        }
        for (int i = 0; i <= vs.getDimL(); i++) {
            System.out.println(vs.getGuardadas()[i].toString());
        }
    }

}
