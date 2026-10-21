Every Java project looks the same inside, from this tiny quest to a huge backend. Know the shape once and you can open any Java project and find your way.

**One shape.** A quest folder looks like this:

```text
quest/
  build.gradle.kts      what the project needs
  settings.gradle.kts   the project's name
  gradlew, gradlew.bat  the Gradle wrapper
  src/main/java/        YOUR code
  src/test/java/        the checker
  build/                generated output
```

**Your code.** `src/main/java` is where the files you edit live. Each file starts with a line like `package oxide.hello_oxide;`, which says which folder inside it the file belongs to.

**The checker.** `src/test/java` holds the tests: small methods that call your code and compare the result. Each test is one objective, and its name is what Oxide ticks off. You read these files, you do not change them.

**The settings.** `build.gradle.kts` tells Gradle what the project needs: Java, plus the test libraries JUnit and AssertJ. You will not edit it in this world.

**The mess folder.** `build/` is output: compiled classes and test reports. Delete it any time and the next run rebuilds it.

## Your turn

Answer the three questions below.
