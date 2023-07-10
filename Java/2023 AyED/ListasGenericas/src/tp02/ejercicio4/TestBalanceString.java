package tp02.ejercicio4;

import tp02.ejercicio3.*;

public class TestBalanceString {
    private PilaGenerica<Character> pila = new PilaGenerica<>();

    public boolean test(String dato, int pos) {
        if (pos < dato.length()) {
            if (dato.charAt(pos) == '(' || dato.charAt(pos) == '[' || dato.charAt(pos) == '{') {
                pila.apilar(dato.charAt(pos));
                test(dato, ++pos);
            } else if (dato.charAt(pos) == ')' || dato.charAt(pos) == ']' || dato.charAt(pos) == '}') {
                if ((pila.tope() == '(' && (dato.charAt(pos) == ')')) || (pila.tope() == '['
                        && (dato.charAt(pos) == ']')) || (pila.tope() == '{' && (dato.charAt(pos) == '}'))) {
                    pila.desapilar();
                    test(dato, ++pos);
                } else {
                    return false;
                }
            } else {
                test(dato, ++pos);
            }
        } else {
            return false;
        }
        if (pila.esVacia()) {
            return true;
        } else {
            return false;
        }
    }

}
