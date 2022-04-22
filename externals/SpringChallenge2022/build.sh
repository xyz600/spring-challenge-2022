#!/bin/bash

mvn assembly:assembly -DdescriptorId=jar-with-dependencies

java -cp target/spider-attack-spring-2022-1.0-SNAPSHOT-jar-with-dependencies.jar Spring2022
