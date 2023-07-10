package tp04.ejercicio6;

import java.util.Scanner;

import tp02.ejercicio2.ListaEnlazadaGenerica;
import tp02.ejercicio2.ListaGenerica;
import tp04.ejercicio5.*;
import tp04.ejercicio1.ArbolGeneral;

public class MainEj6 {
    public static void main(String[] args) {
        // ListaGenerica<ArbolGeneral<Integer>> l = new ListaEnlazadaGenerica<>();
        // ArbolGeneral<Integer> a = new ArbolGeneral<Integer>((Integer) 10, l);
        // ArbolGeneral<Integer> c = new ArbolGeneral<Integer>((Integer) 20, null);
        // ArbolGeneral<Integer> b = new ArbolGeneral<Integer>(30, null);
        // ArbolGeneral<Integer> d = new ArbolGeneral<Integer>(40, null);

        // b.agregarHijo(d);
        // b.agregarHijo(c);
        // a.agregarHijo(b);
        // System.out.println(a.esAncestro((Integer) 10, (Integer) 20));
        String id = "";
        int latencia = -1;
        Scanner s = new Scanner(System.in);
        ArbolGeneral<AreaEmpresa> ag = new ArbolGeneral<>(null);
        System.out.println("Latencia (0 para terminar):");
        latencia = s.nextInt();
        s.nextLine();
        if (latencia != 0) {
            System.out.println("ID:");
            id = s.nextLine();
        }
        System.out.println("---------------------------");
        AreaEmpresa emp = new AreaEmpresa();
        emp.setId(id);
        emp.setLatencia(latencia);
        ag.setDato(emp);
        // vamos a probar armandolo

        // emp = new AreaEmpresa();
        // System.out.println("Latencia (0 para terminar):");
        // latencia = s.nextInt();
        // s.nextLine();
        // if (latencia != 0) {
        // System.out.println("ID:");
        // id = s.nextLine();
        // }
        // emp.setId(id);
        // emp.setLatencia(latencia);
        // ArbolGeneral<AreaEmpresa> agh = new ArbolGeneral<AreaEmpresa>(emp);
        // ag.agregarHijo(agh);
        ArbolGeneral<AreaEmpresa> agaux = ag;
        AreaEmpresa empaux; // Aca guardo la empresa que creo en el while
        ArbolGeneral<AreaEmpresa> aux = new ArbolGeneral<AreaEmpresa>(null);
        AreaEmpresa a = null; // para guardar los datos de esAncestro
        AreaEmpresa b = null;
        while (latencia != 0) {// Esta parte esta hacha a las patadas, muchas cosas mal, pero no es éste el
                               // ejercicio
            for (int i = 0; i < 3; i++) {
                if (latencia != 0) {
                    System.out.println("Latencia (0 para terminar):");
                    latencia = s.nextInt();
                    s.nextLine();
                    if (latencia != 0) {
                        System.out.println("ID:");
                        id = s.nextLine();
                        System.out.println("---------------------------");
                        empaux = new AreaEmpresa(id, latencia);
                        aux = new ArbolGeneral<AreaEmpresa>(empaux, null);
                        agaux.getHijos().agregarFinal(aux);
                        if (latencia == 10) {
                            System.out.println("latencia a");
                            a = empaux;
                        }
                        if (latencia == 20) {
                            System.out.println("latencia b");
                            b = empaux;
                        }
                        System.out.println("Latencia actual: " + empaux.getLatencia());
                    }
                }
            }
            if (agaux.tieneHijos()) {
                agaux.getHijos().comenzar();
                agaux = agaux.getHijos().proximo();
            }

        }
        // AreaEmpresa a = new AreaEmpresa("g", 10);
        // AreaEmpresa b = new AreaEmpresa("h", 20);
        System.out.println(ag.toString());
        System.out.println(ag.esAncestro(a, b));
    }
}
