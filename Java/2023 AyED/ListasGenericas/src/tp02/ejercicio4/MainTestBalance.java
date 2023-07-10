package tp02.ejercicio4;

import java.util.Scanner;
import tp02.ejercicio2.*;
import tp02.ejercicio3.*;

public class MainTestBalance {
    public static void main(String[] args) {
        Scanner s = new Scanner(System.in);
        TestBalanceString test = new TestBalanceString();
        System.out.println("Escribir");
        String dato = s.nextLine();
        if (test.test(dato, 0)) {
            System.out.println("Correcto");
        } else {
            System.out.println("Incorrecto");
        }
    }

}
