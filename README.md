# ROLang - interpretor pseudocod

Acesta este un interpretor de cod pseudocod dupa "specificația" de pe
[pbinfo](https://www.pbinfo.ro/articole/23972/limbajul-pseudocod).

Funcțiile funcționează puțin diferit:
 - Apelurile de funcții/subprograme în expresii se fac cu paranteze:
```
a <- adaugă(a, b)
```
 - În secțiuni cu stmt-uri, apelurile se pot face și cu paranteză, și fără:
```
scrie 'a: ', a, ' sum(10, 5)', sum(10, 5)
```

Analizatorul lexical (lexer) are suport pentru spații în cuvintele cheie "până
când" și "cât timp". Dacă un cuvânt cheie nu are diacritice, o eroare va fi
dată.

