package tp03.ejercicio1y2;

public class MainAB {
    public static void main(String[] args) {
        String raiz = "RAIZ";
        String HI = "HI";
        String HD = "HD";
        ArbolBinario<String> ab = new ArbolBinario<>(raiz);

        ab.agregarHijoIzquierdo(new ArbolBinario<>(HI));
        ab.agregarHijoDerecho(new ArbolBinario<>(HD));
        HI = "HII";
        ab.getHijoIzquierdo().agregarHijoIzquierdo(new ArbolBinario<>(HI));
        HI = "HID";
        ab.getHijoIzquierdo().agregarHijoDerecho(new ArbolBinario<>(HI));
        HI = "HIII";
        ab.getHijoIzquierdo().getHijoIzquierdo().agregarHijoIzquierdo(new ArbolBinario<>(HI));
        HI = "HIID";
        ab.getHijoIzquierdo().getHijoIzquierdo().agregarHijoDerecho(new ArbolBinario<>(HI));
        // ArbolBinario<String> res = ab.espejo();
        // System.out.println(ab.recorrerToString());
        // System.out.println(res.recorrerToString());
        ab.entreNiveles(0, 3);
    }
}
