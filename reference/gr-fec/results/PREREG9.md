# Pre-registration 9 — 'time-crystal' cycle averaging — written 2026-10-24 06:38:16 UTC, before confirmation data
Exploratory (3.0 dB, seeds 1300/1301, 550 failed frames of LAY-RND): deciding failed frames from the mean posterior LLR over the
last 20 iterations gave a valid codeword for 52/550, all correct (0 wrong codewords); window 4 gave 0.
AVG = LAY-RND (50 it, new random order each iteration); frames still failing at 50 are decided by sign of the mean posterior LLR
over iterations 31-50 and accepted only if all parity checks are satisfied.
PRIMARY (P10): at 3.0 dB (seed 1400, 48 000 frames) and 3.5 dB (seed 1401, 96 000 frames), AVG rescues >= 5% of LAY-RND
failures, AND the number of AVG-accepted frames that are wrong codewords is <= 1% of AVG rescues (i.e. averaging does not
create undetected errors).
