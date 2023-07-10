package tp04.ejercicio1;

import tp02.ejercicio2.ListaEnlazadaGenerica;
import tp02.ejercicio2.ListaGenerica;
import tp02.ejercicio3.ColaGenerica;

public class ArbolGeneral<T extends Comparable<T>> {

	private T dato;

	private ListaGenerica<ArbolGeneral<T>> hijos = new ListaEnlazadaGenerica<ArbolGeneral<T>>();

	public T getDato() {
		return dato;
	}

	public void setDato(T dato) {
		this.dato = dato;
	}

	public void setHijos(ListaGenerica<ArbolGeneral<T>> hijos) {
		if (hijos == null)
			this.hijos = new ListaEnlazadaGenerica<ArbolGeneral<T>>();
		else
			this.hijos = hijos;
	}

	public ArbolGeneral(T dato) {
		this.dato = dato;
	}

	public ArbolGeneral(T dato, ListaGenerica<ArbolGeneral<T>> hijos) {
		this(dato);
		if (hijos == null)
			this.hijos = new ListaEnlazadaGenerica<ArbolGeneral<T>>();
		else
			this.hijos = hijos;
	}

	public ListaGenerica<ArbolGeneral<T>> getHijos() {
		return this.hijos;
	}

	public void agregarHijo(ArbolGeneral<T> unHijo) {

		this.getHijos().agregarFinal(unHijo);
	}

	public boolean esHoja() {

		return !this.tieneHijos();
	}

	public boolean tieneHijos() {
		return !this.hijos.esVacia();
	}

	public boolean esVacio() {

		return this.dato == null && !this.tieneHijos();
	}

	public void eliminarHijo(ArbolGeneral<T> hijo) {
		if (this.tieneHijos()) {
			ListaGenerica<ArbolGeneral<T>> hijos = this.getHijos();
			if (hijos.incluye(hijo))
				hijos.eliminar(hijo);
		}
	}

	public ListaEnlazadaGenerica<T> preOrden() {
		return null;
	}

	public int altura() {
		int alt = -1;
		if (this.esHoja()) {
			alt++;
		}
		if (this.tieneHijos()) {
			this.getHijos().comenzar();
			while (!this.getHijos().fin()) {
				alt = Math.max(this.getHijos().proximo().altura() + 1, alt);
			}
		}
		return alt;
	}

	public int nivel(T dato) {
		int niv = -1;
		ColaGenerica<ArbolGeneral<T>> cola = new ColaGenerica<>();
		ArbolGeneral<T> aux = new ArbolGeneral<T>(this.getDato());
		boolean encontre = false;
		if (this != null) {
			niv++;
			cola.encolar(this);
			cola.encolar(null);
		}
		while ((!cola.esVacia()) && (!encontre)) {
			aux = cola.desencolar();
			if (aux == null) {
				niv++;
				cola.encolar(null);
			} else if (aux.getDato() == dato) {
				encontre = true;
			} else if (aux.tieneHijos()) {
				aux.getHijos().comenzar();
				while (!aux.getHijos().fin()) {
					cola.encolar(aux.getHijos().proximo());
				}
			}
		}
		return niv;
	}

	public Integer ancho() {
		int large = 0;
		int anaux = 0;
		ColaGenerica<ArbolGeneral<T>> cola = new ColaGenerica<>();
		ArbolGeneral<T> aux = new ArbolGeneral<T>(this.getDato());
		if (this != null) {
			cola.encolar(this);
			cola.encolar(null);
		}
		while ((!cola.esVacia())) {
			aux = cola.desencolar();
			if (aux == null) {
				anaux = 0;
				if (!cola.esVacia()) {
					cola.encolar(null);
				}
			} else if (aux.tieneHijos()) {
				aux.getHijos().comenzar();
				while (!aux.getHijos().fin()) {
					cola.encolar(aux.getHijos().proximo());
					anaux++;
				}
				large = Math.max(anaux, large);
			}
		}
		return large;
	}

	public String toString() {
		String aux = " ";
		aux += this.getDato().toString();
		if (this.tieneHijos()) {
			aux += " " + this.getHijos().toString();
		}
		return aux;
	}

	public Boolean esAncestro(T a, T b) {
		boolean es = false;
		if (a != null) {
			if (this.getDato().compareTo(a) == 0) {
				if (this.tieneHijos()) {
					this.getHijos().comenzar();
					while (!this.getHijos().fin() && !es) {
						es = this.getHijos().proximo().esAncestro(null, b);
					}
				}
			} else {

				if (this.tieneHijos())
					while (!this.getHijos().fin() && !es) {
						es = this.getHijos().proximo().esAncestro(a, b);
					}
			}
		} else {
			if (this.getDato().compareTo(b) == 0) {
				es = true;
			} else {
				if (this.tieneHijos())
					while (!this.getHijos().fin() && !es) {
						es = this.getHijos().proximo().esAncestro(null, b);
					}
			}
		}

		return es;
	}
}