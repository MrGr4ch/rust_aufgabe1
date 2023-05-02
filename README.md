
# Übungsblatt 1
## Generelles
Die ersten beiden Aufgaben dienen zur Vertiefung Ihrer praktischen Erfahrungen in Rust. Sie implementieren einen Stack und eine Baumstruktur.

## Allgemeine Hinweise
Für diese und alle folgenden Praktikumsaufgaben gilt, dass Einsendungen, die in der jeweils mitgegebenen Testumgebung nicht laufen, entsprechend Punktabzug erhalten! Das beinhaltet insbesondere alle Programme, die sich nicht fehlerfrei kompilieren lassen. Da Cargo für die Ausführung verantwortlich ist, sollte das Projekt bei Ihnen am Ende mit `cargo test` ohne Fehler und Warnungen durchlaufen. 


## Abgabemodus
Die Lösung ist in einem eigenen Git-Repository abzugeben. 
Sie können in ihrer Lösung jedoch beliebige Hilfstypen und Module selbst definieren. 
Die grundlegende Struktur des hier mitgegebenen Templates sollte jedoch nicht verändert werden. 
Insbesondere ist es wichtig, dass die öffentliche Schnittstelle der Library, welche über die Signaturen der Methoden und Funktionen und deren absoluten Pfad definiert wird, nicht verändert wird. 

Zur Lösung der Aufgaben steht für Sie dieses Repository mit
- vorgegebenen Modulen [stack](src/stack.rs) und [syntax_tree](src/syntax_tree.rs)
- der vorgegebenen Schnittstelle der Library in [lib](src/lib.rs)
- einer Reihe von Testfällen (Unit-Tests) innerhalb der Module

zur Verfügung.
> Sie können die Implementierung mit `cargo test` prüfen. Mit `cargo test -- --nocapture` werden Konsolenausgaben auch bei korrekten Tests angezeigt.

## Aufgabe 1 (50 Punkte)
### Kurzbeschreibung
Implementieren Sie einen Stack, der beliebig viele Integerzahlen speichern kann ("beliebig viel" bedeutet für uns: lediglich begrenzt durch den Arbeitsspeicher). Erweitern Sie dabei eine vorgegebene Schnittstelle (Stack Trait), sodass Ihr Stack in andere (bereits existierende) Programme eingebunden werden kann.

### Aufgabenstellung
Wie sie in [lib](src/lib.rs) sehen können, betrachten wir einen Stack als eine Datenstruktur, auf der folgende Operationen ausgeführt werden können:

```rust
pub trait Stack {
    /// Initialisiere einen leeren Stack
    fn init() -> Self;

    /// Füge einen neuen Wert zum Stack hinzu
    fn push_val(&mut self, i: i32);

    /// Lese den obersten Wert, ohne diesen zu entfernen
    fn top_val(&self) -> Option<&i32>;

    /// Entferne den obersten Wert und gib diesen zurück
    fn pop_val(&mut self) -> Option<i32>;

    /// Prüfe ob der Stack leer ist
    fn is_empty(&self) -> bool;
}
```
- Implementieren Sie den Stack-Trait in [stack.rs](src/stack.rs) für `ListStack` und für `Vec<T>`

#### a) Implementierung für Vec<T> (10 Punkte)
- Implementieren Sie den Trait für Vec<T>
- Da Vec<T> bereits selbst über die Methoden eines Stack verfügt, muss hier lediglich an Vec delegiert werden.

#### b) Implementierung für ListStack (40 Punkte)
- Vervollständigen Sie die Implementierung von Stack für ListStack
- Die Implementierung erfordert den Smart-Pointer `Box<T>`, um den rekursiven Typ sicher zu definieren. Eine Erklärung hierzu finden Sie im [Buch](https://doc.rust-lang.org/book/ch15-01-box.html)
- Zusätzlich wird ein Option<T> benötigt, um Werte mit [take](https://doc.rust-lang.org/std/option/enum.Option.html#method.take) auszutauschen


## Aufgabe 2 (50 Punkte)
### Kurzbeschreibung
Implementieren Sie eine Datenstruktur, die beliebig verzweigte Bäume speichern kann, mit den im Folgenden beschriebenen Methoden.

### Aufgabenstellung
Die hier von Ihnen zu implementierende Datenstruktur dient der Repräsentation eines abstrakten Syntaxbaumes.

```rust
struct SyntaxTree;

impl<T> SyntaxTree<T> {
    /// Create a SyntaxTree with a root node that carries the given value
    pub fn new(value: T) -> SyntaxTree<T> {
        todo!()
    }

    /// Add another SyntaxTree as last child of this tree
    pub fn push_node(&mut self, new_node: SyntaxTree<T>) {
        todo!()
    }

    /// Add another SyntaxTree as first child of this tree
    pub fn prepend_node(&mut self, new_node: SyntaxTree<T>) {
        todo!()
    }
    
    /// Insert the given SyntaxTree into the children of this tree at the given index
    pub fn insert_node(&mut self, index: usize, new_node: SyntaxTree<T>) {
        todo!()
    }
    
    /// Perform a depth-first search with the given predicate.
    /// The method returns a reference to the first SyntaxTree instance for which the predicate
    /// return true. If no instance is found, None is returned.
    pub fn find_node(&self, predicate: fn(&SyntaxTree<T>) -> bool) -> Option<&SyntaxTree<T>> {
        todo!()
    }

    /// Perform a depth-first search with the given predicate.
    /// The method returns a mutable reference to the first SyntaxTree instance for which the predicate
    /// return true. If no instance is found, None is returned.
    pub fn find_node_mut(
        &mut self,
        predicate: fn(&SyntaxTree<T>) -> bool,
    ) -> Option<&SyntaxTree<T>> {
        todo!()
    }
}
```
- Die zu implementierende Datenstruktur SyntaxTree soll Baumknoten in beliebig komplexen Konfigurationen speichern können und davon beliebig viele.
- Vervollständigen Sie die Implementierung in der Datei [syntax_tree](src/syntax_tree.rs).
