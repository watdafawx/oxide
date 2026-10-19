**Why this matters:** every Java project, from this tiny quest to a huge backend, has the same shape. Once you know where things live, you can open any Java codebase and find your way.

```
quest/
  build.gradle.kts      dependencies and test settings
  settings.gradle.kts   the project's name
  gradlew, gradlew.bat  the Gradle wrapper
  src/main/java/        YOUR code
  src/test/java/        the tests (the spec)
  build/                generated: compiled classes, test reports
```

- **`src/main/java`** holds what you write. Folders match **packages**: `package oxide.hello_oxide;` means the file lives in `oxide/hello_oxide/`.
- **`src/test/java`** holds the JUnit tests. Each test method is one objective; its **name** is what Oxide ticks off. You read these, you do not change them.
- **`build.gradle.kts`** says what the project needs: Java, JUnit and AssertJ from Maven Central. You will not edit it in this world.
- **`build/`** is output. Delete it any time; the next run rebuilds it.

## Your turn

Answer the three questions below.
