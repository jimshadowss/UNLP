package tp04.ejercicio5;

import java.util.Scanner;

import tp04.ejercicio1.ArbolGeneral;

public class MainEj5 {
    public static void main(String[] args) {
        String id;
        int latencia = -1;
        Scanner s = new Scanner(System.in);
        ArbolGeneral<AreaEmpresa> ag = new ArbolGeneral<>(null);
        System.out.println("Latencia (0 para terminar):");
        latencia = s.nextInt();
        s.nextLine();
        System.out.println("ID:");
        id = s.nextLine();
        System.out.println("---------------------------");
        AreaEmpresa emp = new AreaEmpresa();
        ag.setDato(emp);
        ArbolGeneral<AreaEmpresa> agaux = ag;
        emp.setId(id);
        emp.setLatencia(latencia);
        AreaEmpresa empaux;
        ArbolGeneral<AreaEmpresa> aux = new ArbolGeneral<AreaEmpresa>(null);
        while (latencia != 0) {
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
                    }
                }
            }
            if (agaux.tieneHijos()) {
                agaux = agaux.getHijos().proximo();
            }

        }
        System.out.println(ag);
        AnalizarArbol analizar = new AnalizarArbol();
        System.out.println(analizar.devolverMaximoPromedio(ag));
    }
}
