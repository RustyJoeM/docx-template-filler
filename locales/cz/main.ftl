# $filename (String) - Conflicting filename that would cause a file overwrite.
docx-filler-fail-overwrite =
    Soubor \" { $filename } \" již existuje!
    Raději nebudu nic přepisovat!
docx-filler-fail-load = Nepovedlo se načíst obsah DOCX šablony!

valid-no-tokens = Ve výbraném souboru nejsou žádné proměnné!
# $token (String) - The token that user tried to use multiple times in replacements.
valid-token-duplicity = Proměnnou { $token } lze použít pouze jednou!
valid-missing-input = Chybí hodnoty pro vyplnění!
# $tokens (String) - Number of tokens on input.
# $values (String) - Number of values on input.
valid-count-mismatch =
    Počet žetonů a počet hodnot pro vyplnění nesedí (není stejný)!
    proměnné: { $tokens }
    hodnoty: { $values }
# $line (String) - Number of input line where the problem is.
# $details (String) - Problem details.
valid-line-mismatch =
    Chyba na řádce č. { $line }:
    { $details }
# $filename (String) - Resulting output file name that does not fulfill the .docx suffix requirement.
valid-no-docx-suffix =
    Jméno výsledného souboru musí končit na .docx!
    Vy máte: "{ $filename }".
# $filename (String) - Resulting output file name that multiple input values would have due to output pattern rules.
valid-same-output-filename = Vícero řádek by vytvořilo stejné jméno souboru "{ $filename }"!

#
ui-docx-app-title = vyplňovač docx šablon
ui-docx-load-failed = Nepovedlo se mi načíst šablonu!
ui-docx-generated = Soubor(y) ůspěšně vytvořen(y).
ui-docx-success =  Hotovo
ui-docx-failure = A sakra...
ui-docx-no-template = Není načtená žádná šablona!
ui-docx-fail-font = Nepovedlo se nastavit hlavní font!
ui-docx-fail-init = Selhal start "Native Windows GUI"!
ui-docx-fail-build = Nepodařilo se připravit interface!
#
ui-template-label = Načtená šablona:
ui-template-dialog = Otevřít soubor
ui-template-button = Načíst novou
ui-template-default-folder-fail = Nepovedlo se otevřít pracovní adresář!
#
ui-tokens-label = Proměnné nalezené v šabloně:
ui-tokens-failed-sep-create = Selhalo vytváření oddělovače!
ui-tokens-failed-sep-add = Selhalo zobrazení oddělovače!
ui-tokens-failed-tok-create = Selhalo vytváření proměnné!
ui-tokens-failed-tok-add = Selhalo zobrazení proměnné!
#
ui-values-label = Hodnoty pro nahrazení proměnných v šabloně (co řádek, to samostatný nový soubor):
#
ui-output-label = Pravidlo pro jméno nových souborů:
ui-output-button = Vytvořit DOCX
ui-options-sep-label = Oddělovač hodnot:

lang-not-found = Nelze změtnit nastavení na daný jazyk!
