package tp04.ejercicio7;

import tp04.ejercicio1.ArbolGeneral;

public class RedDeAguaPotable {
    private ArbolGeneral<Integer> ag = new ArbolGeneral<Integer>(null, null);

    public void setAg(ArbolGeneral<Integer> ag) {
        this.ag = ag;
    }

    public ArbolGeneral<Integer> getAg() {
        return this.ag;
    }

    public double minimoCaudal(double caudal) {
        double res = -1;
        if (ag != null) {
            if (ag.esHoja())
                res = ag.getDato();
            else
                res = this.minimoCaudal(caudal, ag);
        }
        return res;
    }

    private double minimoCaudal(double caudal, ArbolGeneral<Integer> ag) {
        double min = Double.MAX_VALUE;
        ag.getHijos().comenzar();
        int tamanio = ag.getHijos().tamanio();
        while (!ag.getHijos().fin()) {
            if (ag.getHijos().proximo().esHoja()) {
                min = Math.min(min, ag.getDato() / tamanio);
            } else {
                min = Math.min(min, this.minimoCaudal(ag.getDato(), ag.getHijos().proximo()));
            }
        }
        return min;
    }

}
