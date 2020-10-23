# $filename (String) - Conflicting filename that would cause a file overwrite.
docx-filler-fail-overwrite =
    File \" { $filename } \" already exists!
    I will not overwrite anything!
docx-filler-fail-load = Failed to read DOCX template contents!

valid-no-tokens = No tokens found!
# $token (String) - The token that user tried to use multiple times in replacements.
valid-token-duplicity = Token { $token } can be used only once!
valid-missing-input = Missing input values!
# $tokens (String) - Number of tokens on input.
# $values (String) - Number of values on input.
valid-count-mismatch =
    Number of tokens is not equal to number of values on input!
    tokens: { $tokens }
    values: { $values }
# $line (String) - Number of input line where the problem is.
# $details (String) - Problem details.
valid-line-mismatch =
    Problem on line { $line } of the input:
    { $details }
# $filename (String) - Resulting output file name that does not fulfill the .docx suffix requirement.
valid-no-docx-suffix =
    Output file name should end with .docx!
    You have: "{ $filename }".
# $filename (String) - Resulting output file name that multiple input values would have due to output pattern rules.
valid-same-output-filename = Multiple input lines would have same output filename "{ $filename }"!

#
ui-docx-app-title = docx template filler
ui-docx-load-failed = Failed to load docx template!
ui-docx-generated = Created docx files succesfully.
ui-docx-success =  Success
ui-docx-failure = Oops...
ui-docx-no-template = No template file opened yet!
ui-docx-fail-font = Failed to set default font!
ui-docx-fail-init = Failed to init Native Windows GUI!
ui-docx-fail-build = Failed to build the interface!
#
ui-template-label = Currently loaded template:
ui-template-dialog = Open File
ui-template-button = Load new docx
ui-template-default-folder-fail = Failed to set working folder!
#
ui-tokens-label = Tokens (placeholders) found in file:
ui-tokens-failed-sep-create = Failed to create separator!
ui-tokens-failed-sep-add = Failed to add separator to layout!
ui-tokens-failed-tok-create = Failed to create token!
ui-tokens-failed-tok-add = Failed to add token to layout!
#
ui-values-label = Values to be filled over tokens (one line per docx to create):
#
ui-output-label = Output files name pattern:
ui-output-button = Generate DOCX files
ui-options-sep-label = Value separator:

lang-not-found = Cannot switch to requested language!
