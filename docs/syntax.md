# PAF-Lang syntax doc

If we want named parameters, calling methods should likely be with parenthesis
- Doesn't feel as good when partially applying functions.

```hs
fn myFun(x: int, y: float) {
    if (x < y) {
        y
    };
    x
}

myFun|x: 3|
```

Is the same parameter applied twice, the latest takes precedence

## Abstract syntax

$f:fn\ i(fa_{formal})$\
$f_{bind}:i|fa_{actual}|$\
$f_{call}:i(fa_{actual})$

$fa_{formal}:fa_{formal}'|$\
$fa_{formal}':fa_{formal}',fa_{formal}'|i:T$

$fa_{actual}:fa_{actual}'|$\
$fa_{actual}':fa_{actual}',fa_{actual}'|i:T$


$e:e\star e|let\ i=e$\
$e_{block}:\{e_{block}'\}$\
$e_{block}':e|e;e'_{block}$

$i: [a-zA-Z\_][a-zA-Z\_0-9]^{*}[']^{*}$
