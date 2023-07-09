package practica1b.ej3;

import java.util.Scanner;

public class ej3 {
    public static void main(String[] args) {
        Scanner s = new Scanner(System.in);
        int i;
        Estudiante[] estudiantes = new Estudiante[2];
        Profesor[] profesores = new Profesor[3];
        for (i = 0; i < 2; i++) {
            estudiantes[i] = new Estudiante();
            System.out.println("****Estudiante " + (i + 1) + "****");
            System.out.println("Nombre: ");
            estudiantes[i].setNombre(s.nextLine());
            System.out.println("Apellido:");
            estudiantes[i].setApellido(s.nextLine());
            System.out.println("Email:");
            estudiantes[i].setEmail(s.nextLine());
            System.out.println("Direccion:");
            estudiantes[i].setDireccion(s.nextLine());
            System.out.println("Comision:");
            estudiantes[i].setComision(s.nextInt());
            s.nextLine();
            System.out.println(estudiantes[i].tusDatos());
        }
        for (i = 0; i < 3; i++) {
            profesores[i] = new Profesor();
            System.out.println("****Profesor " + (i + 1) + "****");
            System.out.println("Nombre: ");
            profesores[i].setNombre(s.nextLine());
            System.out.println("Apellido:");
            profesores[i].setApellido(s.nextLine());
            System.out.println("Email:");
            profesores[i].setEmail(s.nextLine());
            System.out.println("Catedra:");
            profesores[i].setCatedra(s.nextLine());
            System.out.println("Facultad:");
            profesores[i].setFacultad(s.nextLine());
            System.out.println(profesores[i].tusDatos());

        }
        s.close();

    }
}
