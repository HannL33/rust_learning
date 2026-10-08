# Mały plan Gomoku

**Podział pracy:** silnik, reguły gry i bot piszesz sam. GUI implementuje AI po ustaleniu API silnika.

## Założenia pierwszej wersji

- Plansza **15 × 15**, dwóch graczy: X i O; zaczyna X.
- Wygrywa **co najmniej 5 znaków w rzędzie**: poziomo, pionowo lub po przekątnej.
- Bez ograniczeń Renju i dodatkowych reguł otwarcia.
- Pełna plansza bez zwycięzcy oznacza remis; wygraną sprawdzamy przed remisem.
- Indeksy pól są 0-based. Ustal jedną konwencję: `index = row * size + column`.

## Struktura projektu

```text
src/
├── lib.rs                    # udostępnia moduły obu gier
├── main.rs                   # obecne kółko i krzyżyk
├── gomoku_engine.rs          # Twoja implementacja
├── gomoku_bot.rs             # Twoja implementacja
└── bin/
    └── gomoku/
        ├── main.rs           # start okna — AI
```

Uruchamianie z folderu `ml_project`: `cargo run --bin gomoku`.
Testy obu gier: `cargo test`.
Po dodaniu drugiego programu ustaw `default-run = "ml_project"` w `[package]`,
jeśli samo `cargo run` ma nadal otwierać kółko i krzyżyk.

## Etap 1 — silnik i testy (Ty)

- Typy: `Board`, `Cell`, `Player`, `GameResult`, `MoveError` i `Game`.
- `Game` pilnuje tury, legalności ruchu oraz zakazu gry po zakończeniu partii.
- Udostępnij odczyt planszy, jej rozmiaru, aktualnej tury i wyniku.
- Bot potrzebuje sposobu tworzenia niezależnych stanów po ruchu oraz generowania legalnych ruchów.
- Wykrywanie zwycięstwa: cztery kierunki; sprawdzaj granice wierszy i kolumn,
  żeby linia nie przeskakiwała między krawędziami planszy.
- Testy: 5 i 6 znaków w każdym kierunku, brak wygranej dla 4 znaków,
  linie przy krawędziach, zajęte pole, ruch poza planszą, brak zmiany stanu
  po błędzie, blokada po końcu gry oraz remis.

**Gotowe gdy:** da się rozegrać partię samymi wywołaniami silnika i wszystkie testy przechodzą.

## Etap 2 — GUI człowiek–człowiek (AI)

- Najpierw przekazujesz gotowy silnik; AI dopasowuje GUI do jego API.
- Osobny program `gomoku`, plansza 15 × 15, klikanie pustych pól.
- Status tury i wyniku, nowa gra, podświetlenie zwycięskiej linii, jeśli API ją udostępnia.
- Rysowanie i obsługa wejścia korzystają z silnika; reguły nie są kopiowane do GUI.

**Gotowe gdy:** można rozegrać pełną partię dwóch osób i rozpocząć następną.

## Etap 3 — bot z alpha–beta (Ty)

- Minimax z alpha–beta i **rzeczywistym limitem głębokości**.
- Najpierw sprawdź wynik gry; dla niezakończonej pozycji przy głębokości 0 użyj heurystyki.
- Ocena zawsze z perspektywy bota. Wygrana/przegrana musi dominować nad oceną heurystyczną.
- Zacznij od oceny ciągów 2, 3 i 4 znaków oraz liczby otwartych końców tych ciągów.
- Ogranicz kandydatów do pustych pól w odległości najwyżej 2 wierszy i kolumn
  od istniejących znaków; na pustej planszy zacznij od środka.
  To ograniczenie przeszukiwania, nie zmiana legalnych ruchów silnika.
- Zacznij od głębokości 2–3. Mierz liczbę odwiedzonych węzłów i czas obliczeń.
- Wybór ruchu zwraca `None` po końcu gry. Przy zawężaniu okna nie zastępuj
  sprawdzonego ruchu kolejnym tylko dlatego, że zwrócił równą granicę oceny.
- Testy: natychmiastowa wygrana, konieczna blokada, legalny ruch przy wymuszonej
  porażce, obsługa głębokości 0 i zgodność alpha–beta z minimaxem na małych pozycjach
  przy tej samej głębokości, heurystyce i generatorze kandydatów.

**Gotowe gdy:** bot wybiera legalne ruchy, wykorzystuje natychmiastową wygraną,
blokuje pojedynczą bezpośrednią groźbę, gdy może się obronić, i kończy obliczenia w akceptowalnym czasie.

## Etap 4 — GUI człowiek–bot (AI)

- Wybór strony człowieka i podłączenie gotowego bota.
- Obliczenia poza wątkiem GUI, blokada ruchów podczas myślenia bota.
- Nowa gra nie może przyjąć spóźnionego wyniku obliczeń poprzedniej partii.

**Gotowe gdy:** można zagrać jako X i O, okno pozostaje responsywne, restart działa poprawnie.
