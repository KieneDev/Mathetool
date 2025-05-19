# Mathetool - Rechenschritte sichtbar machen

## Was ist dieses Projekt?

Dieses Tool ist ein Open-Source-Projekt, das Mathematik nicht nur ausrechnet, sondern die **Rechenschritte** zeigt.  
Es soll helfen, Mathematik besser zu verstehen, statt nur Ergebnisse zu liefern.

## Warum dieses Tool?

Mathematik wird oft als eine reine Rechenaufgabe gesehen – aber dahinter steckt **Logik, Struktur und klare Schritte**.  
Dieses Tool zeigt jeden einzelnen Rechenschritt und erklärt, was passiert.  
Zielgruppe sind alle, die Mathematik besser verstehen wollen – von Schülern bis zu Erwachsenen, die ihre Kenntnisse auffrischen.

## ⚙️ Technische Infos

- **Sprache:** [Rust](https://www.rust-lang.org/)
- **Plattform:** Linux (getestet unter Manjaro)
- **Kommandozeile (Terminal)** – keine GUI nötig
- **Parser** berücksichtigt:
  - Punkt-vor-Strich-Rechnung
  - Klammern
  - Negative Zahlen (auch trickreich umgesetzt durch Voranstellen von 0)
  - Brüche (optional, aktuell in Entwicklung)
  - Rechenoperatoren `+`, `-`, `*`, `:`, `/`, `^`

---

## 💡 Warum Rust?

> "Weil Rust mich zwingt, **sauber zu denken**."

Entscheidung gegen C++ und für Rust:
- **Null-Cost-Abstraction**, ohne Garbage Collector
- **Safety durch Ownership & Borrowing**
- **Klarer, wartbarer Code**
- **Systemnah** wie C, aber sicherer

Rust passt perfekt zu meiner Philosophie: **nicht schnell irgendwas hacken, sondern bewusst entwickeln**.

---

## 🔭 Aktueller Stand

| Feature | Status |
|--------|--------|
| Parsing | ✅ fertig |
| Operator-Reihenfolge (BODMAS) | ✅ fertig |
| Klammer-Auflösung | ✅ fertig |
| Negative Zahlen erkennen | ✅ fertig |
| Brüche | 🧪 in Arbeit |
| Rechenschritte anzeigen | ✅ fertig |
| Terminal-Anwendung | ✅ läuft |
| GUI (optional) | 🔜 geplant |

---

## 🎯 Zielgruppe

- Schüler:innen (zum Lernen)
- Erwachsene (zum Auffrischen)
- Entwickler:innen (zum Mitmachen)
- Menschen, die **nicht nur das Ergebnis**, sondern den **Weg dorthin** verstehen wollen

---

## 🤝 Mitmachen?

**Pull Requests willkommen** – aber bitte mit Bedacht:  
> Qualität vor Schnelligkeit.

Keine Code-Schnipsel ohne Erklärung. Wer mitdenken will, ist herzlich eingeladen!

---

## 📜 Lizenz

Dieses Projekt steht unter der [MIT-Lizenz](./LICENSE).  
→ **Benutzen, lernen, verbessern – aber fair bleiben.**

---

## 🧠 Mehr über die Philosophie?

In [PHILOSOPHY.md](./PHILOSOPHY.md) erkläre ich, warum ich dieses Tool baue, worauf es mir ankommt und warum ich lieber **Verständnis** statt **Zertifikate** fördere.

---

## 🔗 Projekt-Link

👉 GitHub: [github.com/KieneDev/Mathetool](https://github.com/KieneDev/Mathetool)

