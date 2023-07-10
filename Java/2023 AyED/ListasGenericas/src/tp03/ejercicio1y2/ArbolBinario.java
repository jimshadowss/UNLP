package tp03.ejercicio1y2;

import tp02.ejercicio3.*;

public class ArbolBinario<T> {
	private T dato;
	private ArbolBinario<T> hijoIzquierdo;
	private ArbolBinario<T> hijoDerecho;

	public ArbolBinario() {
		super();
	}

	public ArbolBinario(T dato) {
		this.dato = dato;
	}

	/*
	 * getters y setters
	 * 
	 */
	public T getDato() {
		return dato;
	}

	public void setDato(T dato) {
		this.dato = dato;
	}

	/**
	 * Preguntar antes de invocar si tieneHijoIzquierdo()
	 * 
	 * @return
	 */
	public ArbolBinario<T> getHijoIzquierdo() {
		return this.hijoIzquierdo;
	}

	public ArbolBinario<T> getHijoDerecho() {
		return this.hijoDerecho;

	}

	public void agregarHijoIzquierdo(ArbolBinario<T> hijo) {
		this.hijoIzquierdo = hijo;
	}

	public void agregarHijoDerecho(ArbolBinario<T> hijo) {
		this.hijoDerecho = hijo;
	}

	public void eliminarHijoIzquierdo() {
		this.hijoIzquierdo = null;
	}

	public void eliminarHijoDerecho() {
		this.hijoDerecho = null;
	}

	public boolean esVacio() {
		return this.getDato() == null && !this.tieneHijoIzquierdo() && !this.tieneHijoDerecho();
	}

	public boolean esHoja() {
		return (!this.tieneHijoIzquierdo() && !this.tieneHijoDerecho());

	}

	public String recorrerToString() {
		String aux = this.getDato().toString();
		if (this.tieneHijoDerecho()) {
			aux = aux + " " + this.getHijoDerecho().recorrerToString();
		}
		if (this.tieneHijoIzquierdo()) {
			aux = aux + " " + this.getHijoIzquierdo().recorrerToString();
		}
		return aux;
	}

	@Override
	public String toString() {
		return this.getDato().toString();
	}

	public boolean tieneHijoIzquierdo() {
		return this.hijoIzquierdo != null;
	}

	public boolean tieneHijoDerecho() {
		return this.hijoDerecho != null;
	}

	public int contarHojas() {
		int cant = 0;
		if (!this.esVacio()) {
			if (this.tieneHijoDerecho()) {
				cant += this.getHijoDerecho().contarHojas();
			}
			if (this.tieneHijoIzquierdo()) {
				cant += this.getHijoIzquierdo().contarHojas();
			}
			if (!this.esHoja()) {
				cant++;
			}
		}
		return cant;
	}

	public ArbolBinario<T> espejo() {
		ArbolBinario<T> ab = new ArbolBinario<>(this.getDato());
		if (this.tieneHijoIzquierdo()) {
			ab.agregarHijoDerecho(this.getHijoIzquierdo().espejo());
		}
		if (this.tieneHijoDerecho()) {
			ab.agregarHijoIzquierdo(this.getHijoDerecho().espejo());
		}
		return ab;
	}

	public void entreNiveles(int n, int m) {
		int cont = 0;
		ArbolBinario<T> ab = new ArbolBinario<>();
		ColaGenerica<ArbolBinario<T>> cola = new ColaGenerica<>();
		cola.encolar(this);
		cola.encolar(null);
		while (!cola.esVacia()) {
			ab = cola.desencolar();
			if (ab != null) {
				if (cont >= n && cont <= m) {
					System.out.println(ab.getDato());
				}
				if (ab.tieneHijoIzquierdo()) {
					cola.encolar(ab.getHijoIzquierdo());
				}
				if (ab.tieneHijoDerecho()) {
					cola.encolar(ab.getHijoDerecho());
				}
				cola.encolar(null);
			} else {
				cont++;
			}

		}
	}

}
