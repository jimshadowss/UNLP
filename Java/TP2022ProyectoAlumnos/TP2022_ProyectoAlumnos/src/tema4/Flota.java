/*
 * Click nbfs://nbhost/SystemFileSystem/Templates/Licenses/license-default.txt to change this license
 * Click nbfs://nbhost/SystemFileSystem/Templates/Classes/Class.java to edit this template
 */
package Java.TP2022ProyectoAlumnos.TP2022_ProyectoAlumnos.src.tema4;

/**
 *
 * @author igarz
 */
public class Flota {
    private Micro[] micros = new Micro[15];
    private int dimL = -1;

    public Flota() {

    }

    public boolean getSiHayEspacio() {
        if (dimL < 15) {
            return true;
        } else {
            return false;
        }
    }

    public void addMicro(String patente, String destino, String salida) {
        Micro micro = new Micro(patente, destino, salida);
        dimL++;
        micros[dimL] = micro;
    }

    public Micro getXDestino(String destino) {
        for (int i = 0; i <= dimL; i++) {
            if (micros[i].getDestino().equals(destino)) {
                return micros[i];
            }
        }
        return null;
    }

}
