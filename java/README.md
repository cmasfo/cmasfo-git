
# Java

How to Run Java in VS Code

## Prerequisites - Windows

Install JDK(Java Development Kit).

Set system variables:

* add to PATH: {jdk path}\bin
* set JAVA_HOME: {jdk path}
* set CLASSPATH: {jdk path}\lib

Warning: Use actual jdk path. Do not type that literally.

Installation check: ```java --version```

## Basic Convention

In Java, class name must be same with the file.

For example, if I want to use 'Hello' class, I need to create 'Hello.java'.

## Running Command

Single File Example

```
javac Hello.java
java Hello
```

Multiple Files Example

```
javac Hello.java Hello2.java # seting manually
javac *.java # all java files
java Hello # start from main class
```
