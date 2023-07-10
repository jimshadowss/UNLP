package practica1b.ej2;

import java.util.Scanner;

public class ej2 {
    public static void main(String[] args) {
        int n;
        Scanner s = new Scanner(System.in);
        n = s.nextInt();
        int[] vect = clase.getVec(n);
        for (int i = 0; i <= vect.length; i++) {
            System.out.println(vect[i]);
        }
        s.close();

    }
}
