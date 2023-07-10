/*
 1-A- Incluya la clase Triángulo a la jerarquía de figuras vista (carpeta tema5). Triángulo
debe heredar de Figura todo lo que es común y definir su constructor y sus atributos y
métodos propios. Además debe redefinir el método toString.

 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.tema5;

/**
 *
 * @author igarz
 */
public class Triangulo extends Figura {
    private double lado1;
    private double lado2;
    private double lado3;

    public Triangulo(double lado1, double lado2, double lado3, String CL, String CR) {
        super(CR, CL);
        this.lado1 = lado1;
        this.lado2 = lado2;
        this.lado3 = lado3;
    }

    public double getLado1() {
        return lado1;
    }

    public double getLado2() {
        return lado2;
    }

    public double getLado3() {
        return lado3;
    }

    public void setLado1(double lado1) {
        this.lado1 = lado1;
    }

    public void setLado2(double lado2) {
        this.lado2 = lado2;
    }

    public void setLado3(double lado3) {
        this.lado3 = lado3;
    }

    @Override
    public String toString() {
        String aux = super.toString() + " Lado 1: " + this.getLado1() + " Lado2: " + this.getLado2() + " Lado3: "
                + this.getLado3();
        return aux;
    }

    @Override
    public double calcularPerimetro() {
        double aux = this.getLado1() + this.getLado2() + this.getLado3();
        return aux;
    }

    @Override
    public double calcularArea() {
        double s = (this.getLado1() + this.getLado2() + this.getLado3()) / 2;
        double res = Math.sqrt(s * ((s - this.getLado1()) * (s - this.getLado2()) * (s - this.getLado3())));
        return res;
    }
}
