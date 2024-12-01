@echo off

for /l %%d in (1, 1, 25) do (
	echo use aoc2024::utils; > day%%d.rs
	echo. >> day%%d.rs
	echo pub fn solve^(^) ^{ >> day%%d.rs
	echo. >> day%%d.rs
	echo ^} >> day%%d.rs
)

