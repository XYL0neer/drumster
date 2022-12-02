# dRUmSTer

CLI Drum Machine

## Calc Timing

```
4/4 Takt 60bpm 16-tel auflösung

16/4
4 schläge pro schlag möglich
also jeder schlag hat eine länge von

bpm / 60 = frequenz
60bpm/60s = 1 metrums schlag pro sekunde

frequenz / (auflösung / metrum)

länge eines schlages der programmierten auflösung beträgt .25 sekunden

Um exakt zu schlagen muss pro iteration abstand zur startzeit gemessen werden und dann warten bis die zeit vorbei ist um nächsten schlag auszulösen

davor mapping der verschiedene instrumente auf metrum und damit töne gleichzeitig spielen lassen

for i in schläge {
  play_hit
  loop {
    if start+(länge*i) >= now -> break
  }
}
```
