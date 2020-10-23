# $filename (String) - Conflicting filename that would cause a file overwrite.
docx-filler-fail-overwrite =
    Файл \"{ $filename }\" уже существует!
    Я не буду его переписывать!
docx-filler-fail-load = Не удалось прочитать содержимое DOCX шаблона!

valid-no-tokens = В выбранном файлн не найдены переменные!
# $token (String) - The token that user tried to use multiple times in replacements.
valid-token-duplicity = Переменную { $token }  можно использовать только один раз!
valid-missing-input = Отсутствуют данные для выполнения шаблона!
# $tokens (String) - Number of tokens on input.
# $values (String) - Number of values on input.
valid-count-mismatch =
    Количество переменных не равно количеству готовых значений!
    переменные: { $tokens }
    готовые значения: { $values }
# $line (String) - Number of input line where the problem is.
# $details (String) - Problem details.
valid-line-mismatch =
    Проблема в { $line } строке:
    { $details }
# $filename (String) - Resulting output file name that does not fulfill the .docx suffix requirement.
valid-no-docx-suffix =
    Имя выходного файла должно заканчиваться на .docx!
    Вы написали имя файла: "{ $filename }".
# $filename (String) - Resulting output file name that multiple input values would have due to output pattern rules.
valid-same-output-filename = У нескольких строк ввода будет одно и то же имя файла "{ $filename }"!

#
ui-docx-app-title = docx создатель шаблонов
ui-docx-load-failed = Не удалось загрузить шаблон docx!
ui-docx-generated = Файлы docx были успешно созданы.
ui-docx-success =  Готово
ui-docx-failure = Ошибка
ui-docx-no-template = Шаблон не выбран!! Пожалуйста, выберите файл.
ui-docx-fail-font = Не удалось установить шрифт по умолчанию!
ui-docx-fail-init = Не удалось запустить Native Windows GUI!
ui-docx-fail-build = Не удалось создать интерфейс!
#
ui-template-label = Текущий загруженный шаблон:
ui-template-dialog = Открыть файл
ui-template-button = Загрузить новый
ui-template-default-folder-fail = Не удалось открыть рабочую папку!
#
ui-tokens-label = В файле найдены следующие переменные (заполнители):
ui-tokens-failed-sep-create = Не удалось создать разделитель!
ui-tokens-failed-sep-add = Не удалось добавить разделитель!
ui-tokens-failed-tok-create = Не удалось создать переменные!
ui-tokens-failed-tok-add = Не удалось добавить переменные!
#
ui-values-label = Новые значения переменных (каждая строка будет является отдельным документом):
#
ui-output-label = Шаблон имени выходных файлов:
ui-output-button = Создать DOCX
ui-options-sep-label = Разделитель готовых значений:

lang-not-found = Невозможно переключиться на запрошенный язык!
