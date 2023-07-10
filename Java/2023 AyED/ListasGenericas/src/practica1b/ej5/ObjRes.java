package practica1b.ej5;

public class ObjRes {
    private int max = (int) Integer.MIN_VALUE;
    private int min = (int) Integer.MAX_VALUE;
    private int prom = 0;

    public int getMax() {
        return this.max;
    }

    public void setMax(int max) {
        this.max = max;
    }

    public int getMin() {
        return this.min;
    }

    public void setMin(int min) {
        this.min = min;
    }

    public int getProm() {
        return this.prom;
    }

    public void setProm(int prom) {
        this.prom = prom;
    }

    public void calc(int a, int b, int c) {
        if (this.getMax() < a) {
            this.setMax(a);
        }
        if (this.getMax() < b) {
            this.setMax(a);
        }
        if (this.getMax() < c) {
            this.setMax(a);
        }
        if (this.getMin() > a) {
            this.setMin(a);
        }
        if (this.getMin() > b) {
            this.setMin(b);
        }
        if (this.getMin() > c) {
            this.setMin(c);
        }

        this.setProm((a + b + c) / 3);
    }

}
