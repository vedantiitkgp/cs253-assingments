#!/bin/bash

javac *.java
jar cfm framework.jar manifest.mf *.class
cd ../app1
javac -cp ../framework/framework.jar *.java
jar cf app1.jar *.class
cd ../deploy
cp ../framework/*.jar ../app1/*.jar .
java -jar framework.jar