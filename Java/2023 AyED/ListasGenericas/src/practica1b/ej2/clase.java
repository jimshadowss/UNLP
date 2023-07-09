package practica1b.ej2;

public class clase {
    static int[] getVec(int n) {
        int[] vect = new int[n];
        int i;
        for (i = 0; i < n; i++) {
            vect[i] = n * (i + 1);
        }
        return vect;
    }
}
