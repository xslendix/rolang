# ROLang - interpretor pseudocod

Acesta este un interpretor de cod pseudocod dupa "specificația" de pe
[pbinfo](https://www.pbinfo.ro/articole/23972/limbajul-pseudocod).

Sunt câteva diferențe:
 - Apelurile de funcții/subprograme se fac cu paranteze:
```
scrie("a + b = ", a + b)
```
 - Citirea se face prin numele variabilelor în paranteze:
```
citește("a (număr natural)")
```

Aceste decizii au fost luate pentru simplicitatea analizatorului sintatic
(parser).

Analizatorul lexical (lexer) are suport pentru spații în cuvintele cheie "până
când" și "cât timp". Dacă un cuvânt cheie nu are diacritice, o eroare va fi
dată.

