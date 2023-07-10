package tp04.ejercicio7;

import tp04.ejercicio1.ArbolGeneral;

public class MainEj7 {
    public static void main(String[] args) {
        RedDeAguaPotable red = new RedDeAguaPotable();
        ArbolGeneral<Integer> ag = new ArbolGeneral<Integer>(55, null);
        ArbolGeneral<Integer> aux;
        for (int i = 0; i < 3; i++) {
            aux = new ArbolGeneral<Integer>(55, null);
            ag.agregarHijo(aux);
        }
        red.setAg(ag);
        System.out.println(ag.toString());
        System.out.println("Minimo caudal: " + red.minimoCaudal(1200));
    }
}
