# TRIFAN BOGDAN-CRISTIAN, 322 CD (UPB - ACS - CTI, anul universitar 2023-2024)
# REZOLVARE TEMA 1 - RUSTYBOX

> enuntul temei : https://upb-cs-rust.github.io/teme/rustybox.html

> Bibliografie : https://doc.rust-lang.org/std/fs/fn.create_dir.html 

> Bibliografie : https://doc.rust-lang.org/book/title-page.html

> GitHub repo : https://github.com/UPB-CS-Rust/rustybox-TrifanBogdan24


### Module folosite:
- `use `**`std::env`** => intearctioneza cu mediul programului scris in Rust : argumentele din linia de comanda, variabile de mediu, informatii legate de mediu (cum ar fi `numele directorului curent`)
- `use `**`std::fs`** => importa functii care realizeaza operatii asupra fisierelor din sistem
    `std` = standard
    `fs` = file system
- `use `**`std::os::unix::fs`** => functii specifice sistemului de operare UNIX/LINUX pentru operatii pe fisiere (functiile nu sunt disponibile pe alte sisteme de opeare : MacOS, Windows)

### Argumentele in linia de comanda
`std::env::args()`: Furnizeaza un vector de `String-uri` cu toate argumentele linei de comanda. Primul argument este întotdeauna numele executabilului.

Indexarea parametrilor in linia de comanda se face de la 0:
- args[0] -> primul argument      -> numele executabilului (./rustybox)
- args[1] -> al doiliea argument  -> numele comenzii de rulat in terminal
- args[2] -> al treilea argument  -> primul parametru al comenzii in terminal
- args[3] -> al patrulea argument -> al doilea parametru la comenzii in terminal
- args[args.len() - 1] -> utlimul argument
- args.len() -> numarul de argumente din linia de comanda

### Functii folosite pe parcursul programului:
- `Path::new(&path).exists()` = verifica existenta unui fisier / document la locatia specificata (intoarce `true` / `false`)
- `Path::new(&Path).is_dir()` = verifica daca la locatia data se afla un director (intoarce `true` / `false`)
- `Path::new(&Path).is_file()` = verifica daca la locatia data se afla un fisier (intoarce `true` / `false`)


### Functii
Pentru fiecare comanda in parte am implementat cate o functie Rust care ii poarta numele (functiile de `cp` si `rm` au functii de helper)
Acestea returneaza un `Result<u32, u32>` :
- primul camp al Result-ului este `Ok` si se returneaza in cazul in care oparatiile functiei au fost executate cu succes
- al doilea camp al Result-ului este `Err` si se returneaza in care in cadrul functiei, daca
    1. comanda este invalida (nu se respecta ordinea parametrilor din enunt, exemplu : **ln <optiune> sursa nume_link**), daca
        1. avem prea putine argumente in linia de comanda (doar executabilul / doar executabilul si numele utilitarului)
        1. avem prea multe argumente in linia de comanda
        1. avem falg-uri invalide
        1. flag-urile nu sunt consective, ex: **rm -r dir1 -d**
        In aceasta situatie, am hardocodat eroarea cu `Err(-1)`, caz in care trebuie sa afisam in terminal `Invalid command`
    1. comanda nu s-a executat cu succes => una dintre functiile din modulele precedent metionate intoarce o eroare => returnam eroarea `Err(1)`      


Daca intr-o functie avem atat erorea de executie (`Err(1)`), cat si o comanda invalida (`Err(-1)`), valoarea finala de retur a functiei va fi `Err(-1)`.

> eroarea generata in urma verificarii unei comenzii invalide are importanta mai mare decat cea a executiei

In `main`, vom face pattern-match-ing asupra apelului de functie, dupa cum urmeaza :
- s-a executat cu succes => nu se intampla nimic
- eroarea `-1`           => afisam `Invalid command` si orpim executia main-ului cu codul de iesire `255`
- eroarea `1`            => oprim executia main-ului cu codul de iesire specific numelui utilitarului

> Celalate erori nu ne intereseaza, nici nu sunt generate, intrucat eu mi-am definit doar doua cazuri de eroare : `-1` pentru comanda invalida si `1` pentru executie esuata

## PWD
Echivalentul in Rust a functiei Python `os.getcwd()` pentru afisare directorului curent,
si implicit a comenzii LINUX `pwd`, este **`env::current_dir()`**, al carui nume isi spune de la sine functionalitatea
Functia intoarce tipul de date `Result`, facem pattern matching pentru a extrage rezultatul concret,
Ca mai apoi sa fie convertit la tipul `String` si afisat in terminal.

```
use std::env;
env::current_dir()
```

> Nota: niciuna dintre aceste operatii nu ar trebui sa intoarca o eroare (vorbim de `pwd`, oarecum logic de ce : comanda LINUX nu afiseaza erori)


## ECHO
Functia implementata se bazeaza pe simpla folosire a afisarii argumentelor
Folosesc deci functia `print!` pentru a afisa in terminal fiecare argument (mai putin primii doi sau trei)
Iar daca argumentul curent nu este ultimul dat in linia de comanda, voi afisa un spatiu intre acestia
La final, decizia de tipari o linie noua este data de existenta flagului `-n`
- `echo -n ...` -> nu va afisa o linie noua
- `echo ...`    -> va afisa o linie noua

> By default, nici aceasta functie nu prezinta erori


## CAT
Functia parcurge fiecare argument (mai putin primele doua), citeste si afiseaza continutul 
fisierelor valide de la referinta acestora folsoind `fs::read_to_string(&file)`, asupra careia aplic pattern matching.
Se va opri executia programului la primul argument pentru care functia de mai sus intoarce o eroare (de executie).
In `main`, evaluam tot prin pattern matching daca functia s-a terminat cu acest tip de eroare, caz in care folosim codul de iesire `236`.

> De vreme ce utilitarul UNIX / LINUX nu prezinta flag-uri, singura comanda invalida este atunci cand se foloseste doar numele acestuia
> $ rustybox cat        # invalida (numar insuficient de parametri)

```
use std::fs;
fs::read_to_string(&file_name) 
```

## MKDIR
Similar, trebuie sa iteram toate argumentele (mai putin primele doua: numele executabilului si al comenzii)
wi vom incerca sa cream toate directoarele de la aceste locatii prin `fs::create_dir_all(&args[i])`
la prima operatie invalida, functia de mai sus va intoarce eroarea `1`, moment in care oprim executia functiei
si folosim in `main` codul de iesire `226`.

> De vreme ce utilitarul UNIX / LINUX nu prezinta flag-uri, singura comanda invalida este atunci cand se foloseste doar numele acestuia
> $ rustybox mkdir      # invalida (numar insuficient de parametri)

```
use std::fs;
fs::create_dir_all(&dir_name)
```


## MV
Aceasta cerinta a temei presupune implementarea unei functii care sa parcurga argumentele si sa redenumeasca (sa mute)
locatii (din ierarhia de fisiere UNIX/LINUX) cu numele ultimului argument folosind `fs::rename(&sursa, &dest)`,
care poate intoarce o eroare daca operatia nu s-a realizat cu succes, caz in care functia va intoarce si ea o eroare in `main`,
ca mai apoi sa folosim codul de iesire `216`.

> De vreme ce utilitarul UNIX / LINUX nu prezinta flag-uri, singura comanda invalida este atunci cand se foloseste doar numele acestuia
> $ rustybox mv     # invalida (numar insuficient de parametri)

```
use std::fs;
fs::rename(&file_name, &new_file_name_or_location)
```

## LN
De vreme ce utilitarul UNIX/LINUX `ln` creaza un shortcut (symbolic link) sau o copie (hard link) 
catre un fisier / director, asa si functia mea din Rust implementeaza acelasi lucru
Spre deosebire de alte comenzi, comanda LINUX `ln` se asteapta sa primeasca un numar fix de argumente :
- numele comenzii (`ln`)
- poate sau nu sa primeasca flagul pentru link simbolic (`-s` sau `--symobilic`)
- numele unui fisier / director
- numele shortcutului

Orice combinatie de argumente care nu respecta formatul de mai sus (`rustybox -s f1 l1` sau `rustybox --symbolic f1 l2` sau `rustybox f1 l3`),
poate fi considerata o comanda invalida, si deci trebuie tratata ca atare:
- se afisa pe ecran textul `Invalid command`
- `main-ul` isi va opri executia cu codul si iesire `255`

Daca in schimb functiile de linkare de mai jos genereaza o eroare, atunci 
implementarea va genera si ea o eroare specifica (`1`), asupra carei se va realiza pattern matching in main,
iar in acest caz, se va folosi codul de iesire `206`.

![imagine hard link vs soft link](https://media.licdn.com/dms/image/C4D12AQGuiroLZOHlAg/article-inline_image-shrink_1000_1488/0/1623902989322?e=1704326400&v=beta&t=ORdnCFcJAPZhZcUJZLGhhghtGEF6ZqW-CS0IhzJcG20)

### 1. LN (hard link)

Un hard link este o referinta directa la aceeasi zona de stocare pe disc ca si fisierul sau directorul original. Nu exista nicio distinctie clara între fisierul original si link-ul hard; ambele puncteaza catre acelasi set de date pe disc. Daca stergeti fisierul original, link-urile hard raman intacte atata timp cat exista cel putin un link hard catre acel fisier. Aceasta înseamna ca link-urile hard sunt în esenta copii ale aceluiasi fisier sau director original.

> este echivalentul unei copii

Pentru o comanda valida de hard link, programul `Rust` trebuie sa primeasca exact 4 argumente:
- numele executabilului Rust (`./rustybox`)
- numele utilitarului (`ln`)
- numele fisierului / directorului
- numele link-ului

Operatia de link-are a unui fisier / director se realizeaza in Rust folosind `unix_fs::symlink(&name_file_or_dir, &short_cut_name)`

```
use std::fs;
fs::hardlink(&name_file_or_dir, &short_cut_name)
```

### 2. LN -S / LN --SYMBOLIC (symbolic link)

Un soft link este asemanator cu un shortcut în sistemul de fisiere. Aceasta este o referinta 
la un alt fisier sau director. Soft link-ul contine doar o cale catre fisierul sau directorul 
la care face referire, în timp ce fisierul sau directorul original ramane neschimbat. Daca stergeti fisierul original la care face referire soft link-ul, link-ul simbolic nu va mai functiona corect.

> este echivalentul unui shortcut

Pentru o comanda valida de symobilc link, programul `Rust` trebuie sa primeasca exact 5 argumente:
- numele executabilului Rust (`./rustybox`)
- numele utilitarului (`ln`)
- flagul : `-s` sau `--symbolic`
- flagul trebuie sa apara o singura data, ca fiind al III-lea argument
- numele fisierului / directorului
- numele link-ului

Operatia de link-are a unui fisier / director se realizeaza in Rust folosind `unix_fs::symlink(&name_file_or_dir, &short_cut_name)`

```
use std::os::unix::fs as unix_fs;
unix_fs::symlink(&name_file_or_dir, &short_cut_name)
```

![imagine referinta hard / soft link](https://tutorialshut.com/wp-content/uploads/2021/05/HardLink-and-SoftLink-768x490.jpg)


## RMDIR
Functia intoarce un tip de date `Result` (eroare / ok), asupra carui aplica pattern matching in `main`
Implementarea `rmdir`-ului meu se bazeaza pe iterarea argumentelor din linia de comanda
si pe folosirea `fs::remove_dir(&cale)` care, fie sterge un director gol, fie intoarce o eroare,
iar daca se intampla asta, functia se va opri prin eroarea specific `Err(1)`, caz in care `main-ul` va folosi codul de iesire `196`. 

> De vreme ce utilitarul UNIX / LINUX nu prezinta flag-uri, singura comanda invalida este atunci cand se foloseste doar numele acestuia
> $ rustybox rmdir     # invalida (numar insuficient de parametri)

```
use std::fs;
fs::remove_dir(&cale_director);
```


## RM
Pentru a verifica toate combinatiile de flag-uri, folosesc doi vectori care retin string-urile
asociate fiecarui tip de flag in parte:
- primul vector : `-r`, `-R`, `--recursive`
- al doilea vector : `-d`, `-D`, `--dir`
  
Flagurile mai sus mentionate ocupa indecsi specifici in argumentele liniei de comada :
- ori al trilea argument
- ori argumentele 3 si 4

Eroare specifica unei comenzi invalide - `Err(-1)` - este generata :
- daca avem doar doi parametri in linia de comanda : `rustybox rm`
- avem doar flag-uri in linia de comanda : `rustybox rm -r`
- intalnim acelasi flag de doua ori
- al patrulea argument este un flag, dar al treilea nu este : `rustybox rm dir -1`
    > LINUX nu considera aceasta comanda ca fiind invalida

Daca functia `rm` primeste argumentele intr-un format (ordine) invalida, aceasta va returna
`Err(-1)` (eroare specifica primului caz), iar `main`-ul va afisa pe ecran `Invalid command`, 
oprindu-si executia folosind codul de iesire `255`. 

Mai apoi, vom parcurge inca o data fiecare argument, mai putin primii 2 si flag-urile si vom incerca :
- stergerea drept un fisier
    ```
    fs::remove_file(&file)
    ```
- stergerea drept un di`rector gol (doar daca exista unul din flagurile celui de al doilea vector)
    ```
    fs::remove_dir(&empty_dir)
    ```
- stergerea drept un director in mod recursiv (doar daca exista unul din flag-urile primului vector)
    ```
    fs::remove_dir_all(&recursive_rm_dir)
    ```

> Utilitarul UNIX / LINUX `rm` incearca sa stearga toate fisierele / directoarele, chiar daca acestea exista sau nu
> Daca avem un fisier / director care nu exista, se va incerca stergerea celorlalte, iar la final, functia va returna
> eroare specifica executiei (`Err(1)`)

Daca una dintre functiile de mai sus nu se efectueaza cu succes **nu se va opri executia**,
ci se va incerca eliminarea celorlalte locatii din memorie, iar la final, `main`-ul va afisa folosi codul de eroare `166`.D



## CP

> Functia mea nu trece testul de copiere recursiva a unui director (cp_r.sh)


Functia se asteapta sa primeasca un numar fx de parametrii, iar daca :
- numarul de argumente din linia de comanda este mai mic decat 4
- numarul de argumente din linia de comanda este mai mare decat 5
- daca functia primeste 5 argumente, dintre care al 3-lea nu face parte din multimea
  `-r`, `-R`, `--recursive` (nu este un flag valod)
atunci comanda poate fi considerata invalida, se va opri executia functiei cu eroarea de 
retur `-1`, iar functia `main` a programului va afisa `Comanda invalida`, folosind dupa codul
de iesire `255`.

O comanda valida are urmatorul valid **rustybox cp [flag] locatie1 locatie2**

Este important sa stim ca functia `cp` poate copia un singur fisier recursiv

1. Daca **locatie1** este un fisier
   1. daca **locatie2** nu exista, atunci copiem **locatie1** cu alt nume
    ```
    fn numefisier(cale: String) -> String { // extrage numele fisierului }
   ```
    > numele copiei este dat de ce este in **locatie1** dupa ultimul back-slash '/'
    > nume_fisier("dir1/dir2/dir3/file") -> "file"
    > nume_fisier("file") -> "file"
   
   2. daca **locatie2** exista si este un director, incercam sa il copiem acolo, iar daca nu putem, intoarcem `Err(1)`
    ```
    fn numefisier(cale: String) -> String { // extrage numele fisierului }
   ```

   > vom copia la numele lui **locatie2**, vom adauga `/`, si numele fisierului pe care dorim sa il copiem (= nume_fisier(locatie1))   (oarecum `join` din Python pentru o cale)

   3. daca **locatie2** exista si nu este un director, atunci avem eroarea de executie `Err(1)`
2. Daca **locatie1** : este un director, atunci incercam sa il copiem recursiv (printr-o `functie recursiva`), iar daca nu putem, intoarcem `Err(1)`
3. Daca **locatie1** nu exista, atunci avem eroarea de excutie `Err(1)`


In `main`, daca functia intoare eroarea
- pentru o comanda invalida `Err(-1)` => vom afisa `Invalid command`, folosind coudl `255`
- de executie `Err(1)` => vom folosi codul de iesire specific pentru `cp` : `166` 


## CHMOD
Aceasta comanda primeste un numar invariabil de parametri, iar rolul meu de programator este
sa modific permisiunile fiecarui fisier / director furnizat, lucru realizat cu ajutorul :

```
use std::fs;                                // fs::Permissions - converte un numar din baza 8 intr-o permisiune
use std::os::unix::fs::PermissionsExt;      // extrage permisiune sub forma unui numar in baza 8

let numar_in_baza_opt: u32 = metadata.permissions().mode();

// permisiuni = struct de tip fs::Permissions
let permisiuni = fs::Permissions::from_mode(numar_in_baza_opt);

fs::set_permissions(&nume_fisier_sau_director, permisuni)
```


Pentru implementarea acestui utilitar, distingem doua cazuri :
- cand setam permsiunile cu o valoare numerica concreta
- cand efectuam operatii de adaugare / eliminare asurpa permisiunilor unui fisier / director

> chmod calculator : https://chmod-calculator.com/

### 1. ./rustybox chmod 555 file1 file3 dir1
![numeric chmod](https://res.cloudinary.com/practicaldev/image/fetch/s--R_IIWvQL--/c_limit%2Cf_auto%2Cfl_progressive%2Cq_auto%2Cw_800/https://dev-to-uploads.s3.amazonaws.com/uploads/articles/991ih5ns6a9715guhodp.png)

> Niciodata sa nu rulati
> ```chmod 777```
> Pe mai mult de un singur fisier!!! Acest lucru va poate expune sistemul atacatorilor daca rulati comanda pe fisiere pe care nu ar trebui. Power comes with responsability!

In cadrul modificarii cu permisiuni numerice, trebuie sa iteram fiecare argument in parte (mai putin primmii 3)
si sa setam permisiunile cu numarul din baza 8 retinut. La final, toate fisierele / directoarele vor aceleasi permiuni.

Consideram o comanda `chmod` invalida (caz numeric), daca al treilea argument din linia de comanda
- are caractere diferite de cifre
- contine cifra `8` sa `9` (numarul nu este in `baza 8`)
- are mai mult de 3 cifre
    1. prima cifra   -> user
    1. a doua cifra  -> group
    1. a treia cifra -> others 

> argumentul de dupa chmod trebuie sa fie de forma : [0-7][0-7][0-7]

Daca al treilea argument nu respecta acest format, comanda poate fi considerata a fi **invalida**, caz care trebuie tratat ca atare in main,
afisand textul `Invalid command` si oprind executia programului folsind codul `255`. 

Daca operatia de modificare a informatiilor unui fisier / director nu se realizeaza cu succes, atunci functia va intoarce `Err(1)`,
la a carei interpretare in `main`, vom avea codul de iesire `231`.

Tot asa se va intampla si daca noua permisiune este invalida, adica daca al treilea argument
- nu este un numar in baza 8 (contine caracterele '8' sau '9')
- nu are exact 3 cifre
- contine si alte caractere in afara de cifre
  

In cazul in care al doilea argument este valid, trebuie sa facem urmatoarele convertiri inainte sa asignam permisiunile :

```
al treilea argument -> String -> vector cu exact 3 caractere (numar in baza 10) -> numar in baza 8
```

La final, cu numarul din baza 8 vom seta bitii de permisiune.
Toate fisierele / directoarele vor avea aceleasi permisiuni.


### 2. ./rustybox chmod a+rw file1 file3 dir1
![literal chmod](https://miro.medium.com/v2/resize:fit:1400/1*Qd9k5fOi4crDc33l0VveaQ.png)

> Niciodata sa nu rulati
> ```chmod a+rwx```
> Pe mai mult de un singur fisier!!! Acest lucru va poate expune sistemul atacatorilor daca rulati comanda pe fisiere pe care nu ar trebui. Power comes with responsability!

In cadrul modificarii cu permisiuni literare, trebuie sa iteram fiecare argument in parte, sa extregem permisiunile acestora sa sa le modificam in functie de al treilea argument din linia de comanda, iar daca acesta contine :
- orice caracter in afara de 'a', 'u', 'g', 'o', '+', '-', 'r', 'w', 'x'
- caractere duplicate
- caractere din setul 'a', 'u', 'g', 'o' dupa '+' / '-'
- caractere din setul 'r', 'w', 'x' inainte de '+' / '-'

Comanda poate fi considerata a fi invalida, drept pentru care functia va avea eroarea specifica `Err(-1)`,
la a carei interpretare in `main`, se va afisa `Invalid command`, iar codul de iesire va fi `255`.

Pe parcurs, vom face urmatoarele modificari asupra permisiunilor :

```
fisier -> verificare metadate -> extragerea permisiunilor -> vector de char-uri cu toate permisiunile -> modificare permisiuni -> numar in baza 8 -> setare permisiune
```

Vom seta caracterele hardcodand bitii de permisiune (facem **si logic** asupra numarului din baza opt) si asa obtinem vectorul cu permisiuni, asupra caruia vom adauga sau elimina din acestea.

La final, trebuie sa il transformam intr-un numar in baza 8 (**```Oo....```**), pornind de la 0 si facand **sau logic** intre fiecare bit de permisiune.

Interpretarea celui de al treilea argoment se bazeaza pe folosirea a doi vectori de caractere :
- un vector va contine doar caracterele `'u'`, `'g'`, `'o'`, sau pe toate daca :
  1. nu exista niciun caracter inainte de semn, ex: `+x`
  1. inainte de semn se afla `a` (a == all == u + g + o) : `a-x`
-  alt vector va contine doar caracterele `'r'`, `'w'`, `'x'`

Daca in primul vector se afla :
- `'u'` => modificam permisiunile userului (primele 3 caractere ale permisiunii literare) : `rwx`r-x-w-
- `g` => modificam permisiunile grupului (cele 3 caractere din mijloc ale permisiunii literare) : rwx`r-x`-w-
- `'o'` => modificam permisiunile celorlalti (ultimele 3 caractere ale permisiunii literare) : rwxr-x`-w-`
 
Vom evalua astfel noua permisiune, verificand fiecare combinatie dintre caracterele primului vector cu ale celui de al doilea vector, iar in functie de existenta acestora si de semnul expresiei, vom modifica vectorul permisiunilor literare. Mai apoi facem **sau logic** pornind de la 0 si obtinem permisiunea finala, in baza 8.


La final, cu numarul in baza 8 vom modifica informatiile fisierului / directorului.



## TOUCH
Daca functia nu primeste flaguri, atunci va crea un fisier nou
```
fs::File::create(&file)
```


Daca avem flag-ul `-a`, modificam `atime` = data ultimei accesari a fisierului :
```bash
$  stat -c "%x" file
```

Daca avem flag-ul `-m`, modificam `mtime` = data ultimei modificari aduse fisierului :
```bash
$  stat -c "%y" file
```

Daca avem flag-ul `-c`, modificam `ctime` = data ultimei modificari aduse fisierului :
```bash
$  stat -c "%y" file
```

todo!()


## GREP
Comanda grep afiseaza liniile care

Pentru a cauta in liniile unui fisier daca fac sau nu match pe o anumita expresie,
avem nevoie de container-ul `regex`

```Cargo.toml
[dependencies]
regex = "1.5"    # pentru grep    ✅
```

Pentru a implementa acest utilitar UNIX / LINUX, functia care ii poarta numele
se asteapta sa primeasca un numar fix de argumente in linia de comanda :
- 4 argumente : `rustybox grep [exp] file`
- 5 argumente : `rustybox grep -i [exp] file`

Altfel, comanda nu este valida (pentru acest caz nu facem nimic in main).

Prima etapa pentru a efectua `regex` este sa verificam prezenta flag-ului `-i` :
daca in linia de comanda sunt dati 5 parametri, iar al trilea nu este flag-ul `-i`,
comanda poate fi catalogata drept invalida.

Mai apoi, utilitarul UNIX / LINUX realizeaza operatii doar asupra fisierelor, deci
trebuie sa verific daca ultimul argument este un fisier, ca mai apoi sa fac si verificarea
daca se poate citi.

Trecand de toate aceste etape, argumentul de pe penultima pozitie
(expresia de facut pattern matching pe fisier) trebuie sa fie convertit
la tipul `Regex` din pachetul `regex`.

Dupa aceea, citind continutul fisierului si iterandu-l linie cu linie, algoritmul
afecteaza cautarea expresiei pe fiecare linie si va afisa liniile pentru care :
- expresia **ESTE** gasita pe linie si **NU AVEM** flagul `-i` ca parametru in linia de comanda
- expresia **NU ESTE** gasita pe linie si **AVEM** flagul `-i` ca paranetru in linia de comanda

Spre deosebire de alt functii, aceasta este de tip `void`, iar `main-ul` nu isi va termina executia cu
un exit code specific (exit-code-ul default `0`), atat in cazul comanda este invalida, cat si daca
executia a esuat pe parcurs.

## LS
todo!()


## LS -L (LL)
todo!()

### EDGE CASE
Daca in linia de comanda se furnizeaza un sngur parametru (doar executabilul),
sau daca se foloseste numele unui utilitar care nu este descris in acest text,
functia `main` va afisa in terminal textul `Invalid command` si va folosi codul de iesire `255`.
