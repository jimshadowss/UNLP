package tp04.ejercicio5;

public class AreaEmpresa implements Comparable<AreaEmpresa> {
    private String id;
    private int latencia;

    public AreaEmpresa() {
    }

    public AreaEmpresa(String id, int latencia) {
        this.id = id;
        this.latencia = latencia;
    }

    public void setId(String id) {
        this.id = id;
    }

    public void setLatencia(int latencia) {
        this.latencia = latencia;
    }

    public String getId() {
        return this.id;
    }

    public int getLatencia() {
        return this.latencia;
    }

    public int compareTo(AreaEmpresa a) {
        return Integer.compare(this.getLatencia(), a.getLatencia());
    }

    public String toString() {
        String aux = "Latencia: " + this.getLatencia() + " - Id: " + this.getId();
        return aux;
    }
}
