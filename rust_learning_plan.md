> **Note (EN):** this is my personal, day-by-day Rust study plan, written in Polish.
> The code in this repository follows it. See the [README](README.md) for an English overview.

# Plan nauki Rust — od algorytmów do serwera HTTP

**Profil:** matematyk, junior developer, Python/FastAPI/GenAI background  
**Styl nauki:** przeplatanie teorii z projektem  
**Szacowany czas całkowity:** 8–12 tygodni

---

## MODUŁ 1 — Struktury danych i algorytmy

**Cel:** Opanować ownership, lifetimes i generyki przez ból implementacji, nie przez czytanie.  
**Czas:** 2–3 tygodnie

---

### Tydzień 1 — Struktury liniowe

**Dzień 1–2: Stack i Queue**

- Zaimplementuj Stack oparty na `Vec<T>` z metodami `push`, `pop`, `peek`, `is_empty`, `size`
- Zaimplementuj Queue oparty na dwóch stosach (klasyczna sztuczka) — tu zobaczysz pierwszy raz dlaczego `Option<T>` jest kluczowy
- Napisz testy jednostkowe (`#[cfg(test)]`) dla każdej metody — Rust ma wbudowany test runner, naucz się go od razu
- Pytania do przemyślenia: dlaczego `pop` zwraca `Option<T>` a nie `T`? Co się stanie gdy wywołasz pop na pustym stosie w języku bez Option?

**Dzień 3–4: Linked List**

- To jest klasycznie najtrudniejsza struktura w Ruscie dla początkujących — borrow checker będzie walczył z tobą
- Zacznij od prostej singly-linked list z `Box<T>` (właśnie to poznałeś!)
- Zaimplementuj: `push_front`, `pop_front`, `peek`, iterację przez listę
- Spodziewaj się, że kompilator odrzuci kilka naiwnych prób — to jest cel ćwiczenia
- Opcjonalnie: przeczytaj "Learn Rust With Entirely Too Many Linked Lists" (darmowe online) — to najlepszy materiał o tym jak Rust myśli o ownership

**Dzień 5–7: HashMap od zera**

- Zaimplementuj własny `HashMap<K, V>` z open addressing lub separate chaining
- Potrzebujesz: tablicy kubełków, funkcji hash (możesz użyć `std::collections::hash_map::DefaultHasher`), obsługi kolizji, resize gdy load factor > 0.75
- To ćwiczenie łączy generyki (`<K: Hash + Eq, V>`), traity i zarządzanie pamięcią razem
- Porównaj wydajność swojej implementacji z `std::collections::HashMap` przez `std::time::Instant`

---

### Tydzień 2 — Struktury drzewiaste

**Dzień 1–3: Binary Search Tree**

- Zaimplementuj BST z `Option<Box<Node<T>>>`
- Metody: `insert`, `contains`, `min`, `max`, `delete` (najtrudniejsze!)
- Trzy typy przejścia drzewa: inorder, preorder, postorder — każde jako iterator lub przez rekurencję
- Delete to klasycznie najtrudniejsza operacja w BST — trzy przypadki (liść, jeden potomek, dwóch potomków)

**Dzień 4–5: Heap i Priority Queue**

- Zaimplementuj Min-Heap lub Max-Heap jako tablicę (nie drzewo wskaźnikowe — to jest standardowa implementacja)
- Metody: `insert`, `extract_min/max`, `heapify`
- Zaimplementuj na tym Heap Sort
- Dla matematyka: przeanalizuj dlaczego heapify ma złożoność O(n) a nie O(n log n) — nieintuicyjny wynik

**Dzień 6–7: Trie**

- Zaimplementuj Trie (prefix tree) — idealne do wyszukiwania słów
- Użyj `HashMap<char, TrieNode>` dla dzieci każdego węzła
- Metody: `insert`, `search`, `starts_with`, `delete`
- Mini-projekt na Trie: autocomplete — wpisz prefix, dostaniesz wszystkie słowa

---

### Tydzień 3 — Algorytmy grafowe

**Dzień 1–2: Reprezentacja grafu**

- Zaimplementuj graf jako listę sąsiedztwa: `HashMap<NodeId, Vec<NodeId>>`
- Osobno: graf jako macierz sąsiedztwa `Vec<Vec<bool>>`
- Przemyśl: kiedy macierz sąsiedztwa jest lepsza, kiedy lista? (gęste vs rzadkie grafy)
- Dodaj metody: `add_node`, `add_edge`, `neighbors`, `has_edge`

**Dzień 3–4: BFS i DFS**

- Zaimplementuj BFS iteracyjnie (przez Queue którą już masz)
- Zaimplementuj DFS iteracyjnie (przez Stack) i rekurencyjnie
- Zastosowania: znajdowanie najkrótszej ścieżki (BFS), wykrywanie cykli (DFS), topological sort (DFS)
- Problem do rozwiązania: czy graf jest spójny? Ile ma składowych spójnych?

**Dzień 5–7: Dijkstra i projekt podsumowujący**

- Zaimplementuj algorytm Dijkstry (najkrótsze ścieżki) — tu użyjesz swojego Priority Queue
- Projekt końcowy modułu: **Mały solver labiryntu** — labirynt jako graf, BFS znajduje wyjście, wyświetl ścieżkę w terminalu ASCII

---

### Co powinieneś umieć po module 1

- Rozumieć ownership i borrowing na poziomie wyczucia, nie tylko teorii
- Pisać generyczne struktury danych z trait bounds
- Testować kod w Ruscie (`cargo test`)
- Profilować podstawową wydajność (`std::time::Instant`)
- Czytać błędy kompilatora bez paniki

---

## MODUŁ 2 — Deep Learning: gra z botem

**Cel:** Zrozumieć jak działa uczenie maszynowe od środka, nauczyć się traits/enums/Rc w realnym projekcie, dotknąć multithreadingu.  
**Czas:** 3–4 tygodnie

---

### Tydzień 1 — Silnik gry kółko i krzyżyk

**Dzień 1–2: Reprezentacja stanu gry**

- Zaprojektuj typy: `Board`, `Player` (enum: X lub O), `Cell` (enum: Empty, Taken(Player)), `GameResult` (enum: Win(Player), Draw, InProgress)
- Plansza jako `[Cell; 9]` — tablica na stosie, zero alokacji
- Przemyśl: dlaczego enum z danymi (`Taken(Player)`) jest lepszy niż dwa oddzielne boole?

**Dzień 3–4: Logika gry**

- Funkcja `apply_move(board, position, player) -> Result<Board, MoveError>`
- Funkcja `check_winner(board) -> GameResult`
- Funkcja `available_moves(board) -> Vec<usize>`
- Trait `Display` dla `Board` — żeby ładnie wyświetlić planszę w terminalu
- Napisz testy dla wszystkich przypadków wygranej i remisu

**Dzień 5–7: CLI do grania człowiek vs człowiek**

- Pętla gry: wyświetl planszę, wczytaj ruch z stdin (`std::io::stdin()`), validuj, wykonaj, sprawdź wynik
- Obsługa błędów: zły format, zajęte pole, poza planszą — wszystko przez `Result<T, E>`
- Cel: gra musi działać kompletnie zanim zaczniesz pisać bota

---

### Tydzień 2 — Bot Minimax

**Dzień 1–3: Algorytm Minimax**

- Zaimplementuj czysty Minimax bez żadnej biblioteki
- Minimax to drzewo przeszukiwań — tu wraca twój `Box<T>` i rekurencja z modułu 1
- Funkcja `minimax(board, is_maximizing, depth) -> i32` — zwraca ocenę pozycji
- Funkcja heurystyczna: +10 za wygraną X, -10 za wygraną O, 0 za remis
- Funkcja `best_move(board, player) -> usize` — wybiera najlepszy ruch

**Dzień 4–5: Alpha-Beta Pruning**

- Minimax dla kółko-krzyżyk przeszukuje maksymalnie 9! = 362 880 stanów — mało
- Alpha-beta odcina gałęzie które nie zmienią wyniku — zrozum dlaczego to działa
- Implementacja: dodaj dwa parametry `alpha: i32, beta: i32` do funkcji minimax
- Zmierz ile węzłów przeszukujesz z i bez alpha-beta (`static AtomicUsize` jako licznik)

**Dzień 6–7: Rozszerzenie na Gomoku (5 w rzędzie)**

- Zmień planszę na 15x15 lub 10x10
- Teraz pełne przeszukiwanie jest niemożliwe — potrzebujesz limitu głębokości (`max_depth`)
- Napisz lepszą heurystykę: oceniaj pozycję na podstawie wzorców (3 w rzędzie = +100, 4 w rzędzie = +1000 itd.)
- To jest moment gdzie zobaczysz "horizon effect" — bot który nie widzi dalej niż N ruchów robi głupie rzeczy

---

### Tydzień 3 — Warcaby z MCTS

**Dzień 1–2: Silnik warcabów**

- Reprezentacja planszy 8x8, figury: pionek i damka dla dwóch graczy
- Generowanie legalnych ruchów: zwykłe przesunięcia, bicia obowiązkowe, bicia wielokrotne
- Walidacja ruchów, promocja na damkę, wykrycie końca gry
- To jest znacznie bardziej złożone niż kółko-krzyżyk — planuj struktury danych ostrożnie

**Dzień 3–5: Monte Carlo Tree Search (MCTS)**

- MCTS to alternatywa dla Minimax — nie wymaga heurystyki, uczy się przez losowe symulacje
- Cztery fazy: Selection (UCB1 formula), Expansion, Simulation (random rollout), Backpropagation
- Zaimplementuj węzeł MCTS: `Rc<RefCell<MctsNode>>` — tu pojawia się `Rc` z poprzedniej teorii naturalnie
- Liczba symulacji jako parametr: 100 symulacji = słaby bot, 10 000 = przyzwoity

**Dzień 6–7: Multithreading — równoległe symulacje**

- Symulacje MCTS są niezależne — idealny kandydat na równoległość
- Przenieś `Rc<RefCell<>>` na `Arc<Mutex<>>` — jeden krok, kompilator pokaże co trzeba zmienić
- Użyj `std::thread::spawn` lub `rayon::par_iter` do równoległego rollout
- Zmierz speedup: ile razy szybciej z N wątkami?

---

### Tydzień 4 — Sieć neuronowa do oceny pozycji

**Dzień 1–2: Matematyczne fundamenty**

- Zanim użyjesz biblioteki — zaimplementuj ręcznie: forward pass prostej sieci (matrix multiply + activation), backpropagation dla jednej warstwy, gradient descent krok po kroku
- Użyj tylko `Vec<f32>` i własnych funkcji mnożenia macierzy
- Cel: zrozumieć co biblioteka robi za ciebie, nie tylko jak jej używać

**Dzień 3–5: Biblioteka Burn**

- `burn` to natywna biblioteka deep learning w Ruscie (nie wrapper PyTorch jak `tch-rs`)
- Zdefiniuj prostą sieć: input (reprezentacja planszy jako wektor), 2-3 warstwy ukryte, output (ocena pozycji -1 do +1)
- Generuj dane treningowe przez self-play z MCTS: zapamiętuj (stan, kto wygrał) dla każdej gry
- Trenuj sieć na zebranych danych

**Dzień 6–7: Integracja z botem**

- Zastąp losowy rollout w MCTS oceną sieci neuronowej
- Zamiast losowej symulacji do końca gry — sieć szacuje prawdopodobieństwo wygranej z danej pozycji
- To jest dokładnie idea AlphaZero (uproszczona)
- Porównaj bota z siecią vs bota bez sieci w 100 grach

---

### Co powinieneś umieć po module 2

- Rozumieć search algorithms i MCTS (ważne w kontekście GenAI — to samo co RLHF używa)
- Używać `Rc<RefCell<>>` i `Arc<Mutex<>>` w realnym projekcie
- Rozumieć multithreading na poziomie podstawowym
- Pisać testy integracyjne, nie tylko unit testy
- Używać zewnętrznych crateów (`cargo add`)

---

## MODUŁ 3 — HTTP Serwer od zera

**Cel:** Zrozumieć jak działa sieć, HTTP, bazy danych — rzeczy których Python ukrywa za FastAPI/SQLAlchemy.  
**Czas:** 3–4 tygodnie

---

### Tydzień 1 — TCP i HTTP ręcznie

**Dzień 1–2: Jak działa TCP**

- Zanim napiszesz kod — zrozum: co to jest socket, port, połączenie, handshake
- `std::net::TcpListener::bind("127.0.0.1:7878")` — jeden import, zero bibliotek
- Napisz echo server: klient wysyła tekst, serwer odsyła ten sam tekst
- Narzędzie do testowania: `nc localhost 7878` (netcat) lub `curl`

**Dzień 3–4: Parser HTTP/1.1**

- HTTP request to zwykły tekst: pierwsza linia = `METHOD PATH HTTP/1.1\r\n`, potem nagłówki, potem body
- Napisz parser który z `Vec<u8>` wyciąga: metodę HTTP, ścieżkę, nagłówki jako `HashMap<String, String>`, body
- HTTP response: `HTTP/1.1 200 OK\r\nContent-Length: N\r\n\r\nbody`
- Przetestuj: serwer zwracający "Hello World" na każdy request — otwórz w przeglądarce

**Dzień 5–7: Obsługa wielu połączeń przez wątki**

- Naiwna pętla `loop { accept() }` obsługuje jedno połączenie naraz — sprawdź przez dwa terminale
- `std::thread::spawn` dla każdego połączenia — teraz działa równolegle
- Problem: co gdy masz 10 000 połączeń naraz? Każdy wątek to ~2MB stosu
- Zaimplementuj Thread Pool: stała liczba wątków (np. 8), kanał `mpsc::channel` do przekazywania połączeń
- Dokładnie ten problem rozwiązuje async/Tokio — ale najpierw zrozum dlaczego wątki nie skalują

---

### Tydzień 2 — Routing i middleware

**Dzień 1–3: Router**

- Zaprojektuj system routingu: `HashMap<(Method, String), Handler>` gdzie `Handler` to `Box<dyn Fn(Request) -> Response>`
- Zarejestruj handlery: `router.get("/users", handle_users)`
- Obsługa parametrów ścieżki: `/users/:id` — wyciągnij `id` z URL
- Obsługa query params: `/users?page=2&limit=10`

**Dzień 4–5: Middleware**

- Middleware to funkcja która opakowuje handler: `fn(Request, NextFn) -> Response`
- Zaimplementuj: logger (każdy request → stdout z czasem i statusem), CORS headers, basic rate limiting (X requestów na minutę per IP)
- Przemyśl: jak FastAPI implementuje middleware? Teraz rozumiesz co `@app.middleware("http")` robi

**Dzień 6–7: JSON**

- Dodaj crate `serde` i `serde_json`
- `serde` to system serializacji/deserializacji w Ruscie — `#[derive(Serialize, Deserialize)]` na struct
- Parsuj JSON body z requestu do struktury Rust
- Serializuj strukturę Rust do JSON response
- Porównaj z Pydantic w FastAPI — analogia jest bezpośrednia

---

### Tydzień 3 — PostgreSQL i CRUD

**Dzień 1–2: Baza danych od podstaw**

- Zanim użyjesz biblioteki: co to jest connection pool i dlaczego jeden connection nie wystarczy?
- Co to jest prepared statement i dlaczego SQL injection jest możliwe bez nich?
- Zainstaluj PostgreSQL lokalnie lub przez Docker: `docker run -e POSTGRES_PASSWORD=pass -p 5432:5432 postgres`

**Dzień 3–4: sqlx**

- `sqlx` to Rust crate do PostgreSQL (i innych baz) — compile-time checked queries
- Połączenie z bazą: `PgPool::connect(DATABASE_URL)`
- Napisz migrację: stwórz tabelę `users (id SERIAL, name TEXT, email TEXT, created_at TIMESTAMPTZ)`
- Zaimplementuj podstawowe queries: SELECT, INSERT, UPDATE, DELETE

**Dzień 5–7: Pełne REST API**

- Połącz router z modułu 2 z sqlx z modułu 3
- Zaimplementuj pełny CRUD dla `users`:
  - `GET /users` — lista użytkowników z paginacją
  - `GET /users/:id` — jeden użytkownik lub 404
  - `POST /users` — stwórz użytkownika, walidacja emaila
  - `PUT /users/:id` — update
  - `DELETE /users/:id` — usuń
- Obsługa błędów: własny typ `AppError` implementujący konwersję na HTTP response

---

### Tydzień 4 — Async i przepisanie na Tokio

**Dzień 1–2: Dlaczego async?**

- Thread pool z tygodnia 1 ma limit: N wątków = N równoległych połączeń
- Async/await: jeden wątek obsługuje tysiące połączeń przez cooperative multitasking
- Przeczytaj teraz rozdział 16-17 z Rust Book — masz konkretny powód, wszystko będzie jasne
- Tokio: `#[tokio::main]`, `async fn`, `.await` — zmień echo server z tygodnia 1 na async

**Dzień 3–5: Przepisanie serwera na Tokio**

- Zamień `std::net::TcpListener` na `tokio::net::TcpListener`
- Zamień `std::thread::spawn` na `tokio::spawn`
- `sqlx` ma natywne wsparcie async — `pool.fetch_all(query).await`
- Porównaj wydajność: thread pool vs async przez `wrk` lub `hey` (narzędzia do load testingu)

**Dzień 6–7: Projekt końcowy — porównanie z FastAPI**

- Napisz ten sam endpoint w FastAPI i w swoim serwerze Rust
- Zmierz: latency (średnia, p99), throughput (req/s), zużycie RAM
- Wyniki będą spektakularne — Rust będzie ~10-100x wydajniejszy w RAM, porównywalny lub szybszy w latency
- To jest moment "aha" dlaczego Rust istnieje w backendie

---

### Co powinieneś umieć po module 3

- Rozumieć TCP/IP i HTTP na poziomie bajtów
- Wiedzieć co FastAPI/uvicorn robi za ciebie
- Rozumieć async/await i dlaczego istnieje
- Pisać production-style error handling w Ruscie
- Używać PostgreSQL bez ORM

---

## Co dalej po wszystkich modułach

Po ukończeniu tych trzech modułów masz solidne podstawy. Naturalne kierunki:

**Jeśli chcesz pogłębić Rust:** `unsafe` Rust, FFI (wywoływanie kodu C z Rusta), makra proceduralne, praca z WASM

**Jeśli chcesz wrócić do GenAI:** biblioteka `candle` (Hugging Face, Rust-native inference), pisanie Python bindings przez `PyO3` (szybkie moduły Rust wywoływane z Pythona — idealne dla applied GenAI)

**Jeśli chcesz backend production:** przestudiuj Axum (zbudujesz identyczny serwer w 10% kodu), Tower (middleware ecosystem), tracing (structured logging)

---

## Zasoby na każdy moduł

**Moduł 1:**
- "Learn Rust With Entirely Too Many Linked Lists" — https://rust-unofficial.github.io/too-many-lists/
- `cargo test` dokumentacja — https://doc.rust-lang.org/book/ch11-00-testing.html

**Moduł 2:**
- Dokumentacja cratea `burn` — https://burn.dev
- Wikipedia: Monte Carlo Tree Search — matematyczne podstawy UCB1

**Moduł 3:**
- Rozdział 20 Rust Book (HTTP serwer) — przeczytaj po własnej implementacji, nie przed
- Dokumentacja `sqlx` — https://docs.rs/sqlx
- Dokumentacja `serde` — https://serde.rs
- `tokio` tutorial — https://tokio.rs/tokio/tutorial

---

*Szacowany czas: moduł 1 = 2-3 tygodnie, moduł 2 = 3-4 tygodnie, moduł 3 = 3-4 tygodnie*  
*Razem: 8-11 tygodni przy ~2-3 godzinach dziennie*
