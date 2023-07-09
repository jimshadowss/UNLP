package practica1b.ej5;

public class calcular {
    static int min = (int) Integer.MAX_VALUE;
    static int max = (int) Integer.MIN_VALUE;
    static int prom;
    private int i;
    private int[] res = new int[] {
            (int) Integer.MAX_VALUE, 0, (int) Integer.MIN_VALUE
    };
    private int[] datos = new int[3];

    public int[] calc1(int a, int b, int c) {
        datos[0] = a;
        datos[1] = b;
        datos[2] = c;
        for (i = 0; i < res.length; i++) {
            if (res[0] > datos[i]) {
                res[0] = datos[i];
            }
            if (res[2] < datos[i]) {
                res[2] = datos[i];
            }
            res[1] = res[1] + datos[i];
        }
        res[1] = res[1] / datos.length;
        return res;
    }

    public ObjRes calc2(int a, int b, int c) {
        ObjRes res = new ObjRes();
        res.calc(a, b, c);
        return res;
    }

    static void calc3(int a, int b, int c) {

        if (min > a) {
            min = a;
        }
        if (min > b) {
            min = b;
        }
        if (min > c) {
            min = c;
        }
        if (max < a) {
            max = a;
        }
        if (max < b) {
            max = b;
        }
        if (max < c) {
            max = c;
        }
        prom = (a + b + c) / 3;
    }
}
