cmake_minimum_required(VERSION 3.0)
project(PoCRust C)
add_library(pocrust STATIC pocrust.c)
install(TARGETS pocrust DESTINATION .)