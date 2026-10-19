**Why it matters:** "do this to each item" and "keep the items where this is true" are everywhere. Lambdas let you hand that bit of behaviour to a method instead of writing another loop.

A **lambda** is a small unnamed function: `parameters -> result`. It fits wherever a **functional interface** (one abstract method) is expected:

```java
Predicate<String> isEmpty = s -> s.isEmpty();       // String -> boolean
Function<String, Integer> len = s -> s.length();    // String -> Integer
Runnable hello = () -> System.out.println("hi");    // no input, no output

isEmpty.test("");     // true
len.apply("abc");     // 3
hello.run();
```

- `Predicate<T>`: `test` returns boolean. `Function<T, R>`: `apply` turns a T into an R. `Runnable`: `run`, nothing in or out.
- A lambda can read local variables, but only ones that never change after being set.
- When the lambda only calls one existing method, use a **method reference**: `s -> s.length()` is `String::length`. Same thing, less noise.
- A method that *returns* a lambda is normal: `longerThan(3)` hands you a ready-made `Predicate`.

## Your turn

- Open `src/main/java/oxide/lambdas_and_method_refs/Lambdas.java`.
- `longerThan`: return a `Predicate<String>` that is true when the word is longer than `n`.
- `lengthByLambda` and `lengthByMethodRef`: return a `Function<String, Integer>` written both ways.
