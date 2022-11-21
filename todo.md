Fehler
--------
Pro Spalte ist nur ein Typ möglich, wenn Dropdown dann nur Dropdown.
Daher empfehle ich die Freitextspalte zu lassen und eine zusätzliche Spalte mit Dropdown hinzuzufügen.
Bitte Dropdowntexte angeben.

Hier die typischen Fehler:
Falsche Stückzahl geliefert.
Falsches Motiv geliefert.
Holz Einlage vergessen.
Name oder Datum falsch graviert.
Sockel vergessen.
Weinflasche vergessen.
Zerbrochen angekommen.
Schlechte Qualität


Datenmüll
----------
Mehrfachscann unterbinden.
Ich werde im Clienten Duplikate (Barcode / User) verhindern.
Wenn es bereits einen Eintrag gibt, wird kein neues gespeichert.
Es wird dazu keine Messagebox geben, vielleicht nur eine Windows Notifikation, unten Rechts so wie bei jedem Barcode, quasi als Protokoll.

Einverstanden. Das Update ist für uns sehr wichtig.



Fehlermeldungen
---------------
Es gibt momentan zwei Messageboxen.
Eine Abfrage ob man wirklich die App schliessen will und eine Warnung das leere Einträge unzulässig sind.
Die zweite Warnung kann ich wechseln zu Windows Meldung und bei der ersten Frage ob wirklich beenden schaue ich noch ob es einen Timer dazu gibt.

Das klingt gut. Danke.


Mangelware & Ausschuss
-----------
Zwei Barcodes für Mangelware sollen mehrfach Scannbar sein -> kann ich umsetzen.
Meine Idee ist es das ich diese Ausnahmen im Strapi hinterlege, so könnt ihr dann selbständig diese Ausnahmen managen.

Wie habt ihr euch das vorgestellt?
Zuerst Barcode scannen, danach Mangelware-Barcode scannen?
Dann könnt ihrs schauen welche Einträge eine Zeile vor dem Mangelware Barcode.
Auswertung könnte ggf. Mühsam sein.

Mein Vorschlag ist es das der Mitarbeiter z.B Mangelware Scannen kann, danach den Barcode vom Fehler, dann wird dieser Barcode als Mangelware direkt markiert.
Dazu könnte ich die eure noch zu definierende Fehlerdropdowns mit Mangelware und Ausschuss ergänzen?
Es wird dazu keine Abfrage geben um nicht den Client zu blockieren.
Falscheinträge müsste der Admin direkt korrigieren.

Dazu sende ich Dir gleich eine kurze Sprachnachricht via WhatsApp.

Zeitzonen
--------------
Zeitzonen zwischen Strapi und Metabase sind unterschiedlich, kann mann das anpassen?

Wir sehen im Metabase 3 verschiedene Timestamps pro Barcode. Wofür sind die zwei weiteren notwendig? Müsste es nicht nur einen Timestamp geben wann der Datensatz ins Trapi gekommen ist, oder habe ich da einen Denkfehler?