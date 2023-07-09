/*

4-A- Definir una clase para representar micros. Un micro conoce su patente, destino, hora
de salida, el estado de sus 20 asientos (boolean: true ocupado, false libre) y la cantidad de
asientos ocupados al momento. Lea detenidamente a) y b) y luego implemente.
a) Implemente un constructor que permita iniciar el micro con una patente, un destino y
una hora de salida (que se reciben) y con sus asientos libres.
b) Implemente métodos para:
i. devolver/modificar patente, destino y hora de salida
ii. devolver la cantidad de asientos ocupados
iii. devolver si el micro está lleno
iv. validar un nro. de asiento que se recibe (es decir, devolver si está en rango o no)
v. devolver el estado de un nro. de asiento válido que se recibe
vi. ocupar un nro. de asiento válido que se recibe
vii. liberar un nro. de asiento válido que se recibe
viii. devolver el nro. del primer asiento libre
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.tema4;

/**
 *
 * @author igarz
 */
public class Micro {
    private String patente;
    private String destino;
    private String hSalida;
    private final int columnas = 2;
    private final int filas = 10;
    private boolean[][] asientos = new boolean[columnas + 1][filas + 1];

    public Micro(String patente, String destino, String hSalida) {
        int i;
        int j;
        this.patente = patente;
        this.destino = destino;
        this.hSalida = hSalida;
        for (i = 0; i < columnas; i++) {
            for (j = 0; j < filas; j++) {
                asientos[i][j] = false;
            }
        }
    }

    public void setPatente(String patente) {
        this.patente = patente;
    }

    public String getPatente() {
        return patente;
    }

    public void setDestino(String destino) {
        this.destino = destino;
    }

    public String getDestino() {
        return destino;
    }

    public void setHSalida(String hSalida) {
        this.hSalida = hSalida;
    }

    public String getHSalida() {
        return hSalida;
    }

    public int getOccupied() {
        int i, j, cant = 0;
        for (i = 0; i < columnas; i++) {
            for (j = 0; j < filas; j++) {
                if (asientos[i][j]) {
                    cant++;
                }
            }
        }
        return cant;
    }

    public boolean getSiLleno() {
        int i, j;
        boolean si = true;
        for (i = 0; i < columnas; i++) {
            for (j = 0; j < filas; j++) {
                if (!asientos[i][j]) {
                    si = false;
                }
            }
        }
        return si;
    }

    public boolean getSiEnRango(int[] n) {
        boolean si = false;
        if ((n[0] <= columnas) && (n[1] <= filas)) {
            si = true;
        }
        return si;
    }

    public boolean getSiOcupado(int[] n) {
        return asientos[n[0]][n[1]];
    }

    public void setOcupado(int[] n) {
        asientos[n[0]][n[1]] = true;
    }

    public void setLibre(int[] n) {
        asientos[n[0]][n[1]] = false;
    }

    public int[] getLibre() {
        int i, j = 0;
        int[] n = new int[2];
        aqui: for (i = 0; i < columnas; i++) {
            for (j = 0; j < filas; j++) {
                if (!asientos[i][j]) {
                    break aqui;
                }
            }
        }
        n[0] = i;
        n[1] = j;
        return n;
    }

    public String toString() {
        String aux = "Micro patente " + patente + " a " + destino + " saliendo a las " + hSalida;
        return aux;
    }
}
