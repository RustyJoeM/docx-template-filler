

## tokens (placeholders/variables?)

A concept important for what this application does, is the so called "*token*" replacement.

**Token** is a piece of text in your DOCX document, surrounded by double curly braces - e.g. `{{NAME}}`, or `{{favorite-pet}}`. 

There are no extra rules for tokens, the curly brace embrace (pun intended) is the only one. Valid token can have any text inside, even spaces, numbers, etc. e.g `{{full name}}`, or `{{shoe-size-11}}` are also valid tokens.

## example

Let's describe what the application does by a simple example. Let's say that I have a DOCX document, with the following text inside, nicely formatted, etc.:

```
Dear {{PERSON}},

let me congratulate you, dear {{PERSON}} to your {{OCCASION}} on {{EVENT_DATE}}! Yay, what a joyful day!

sincerely,
{{WRITER}}
```

I want to send this to all my employees at specific occasions, and creating dozens or even hundreds of such files might be rather monotone and tiring work. Let's use the *docx-template-filler* instead!

I load the DOCX document above as a "template" into *docx-template-filler*, and it shows me which unique **tokens** it found in the document:

```
{{PERSON}} ; {{OCCASION}} ; {{EVENT_DATE}} ; {{ WRITER}}
```

I can change **order of tokens** by switching values in dropdowns that show when i load the DOCX template in the main application window.

Now i need to enter several lines, one per each standalone document i want to automatically create. We put this text (these **values**) into the big text area:

```
Joseph; birthday; April 1st; Janice
Karl; marriage anniversary; February 31st; Janice
Albert; tremendous discovery; that one day; Nikola
```

Notice the use of "separator" - character ";" in the lines above.

**Separator** is a special character that tells the program how to cut one whole line of text into several tokens.

You can also change this separator to something else, like comma (","), pipe character ("|"), etc. Just remember that You can NOT use the same text that you decide to be used as separator in the values.
e.g. if you decide to use comma "," as a separator, you cannot write addresses that include comma character as a value for {{ADDRESS}} token, etc.

We could go on, putting more and more lines as needed... Such lines (that represent sort of table of data) can usually be obtained from XLS spreadsheet, or exported from other office / finance / human resources related applications / web-pages... Worst case, they can be written / copy&pasted manually if it fits your use-case.

Order of the values you have set in the token dropdowns is important!
Your data has to be in the order matching the set order of tokens, so that program places correct text into correct parts of new document!

One last part we have to address is the **output filename pattern**.
What is it? We are going to create lots of new files (3 in this example).
We cannot give name to each and every file manually...

docx-template-filler does this for us, by using the rule/pattern. Notice how this pattern was automatically set when opening the DOCS template to {`{your-first-token}}.docx`. We can of course change this to any other text, and can use `{{...}}` tokens to make parts of the file names automatically...

Best way to explain is just to show you what we will get as a result:

when we press the final button to generate the output, we get three new shiny documents in the directory where the docx-filler-app is located:

file named `Joseph.docx` will have this content:

```
Dear Joseph,

let me congratulate you, dear Joseph to your birthday on April 1st! Yay, what a joyful day!

sincerely,
Janice
```

file named `Karl.docx` will have this content:

```
Dear Karl,

let me congratulate you, dear Karl to your marriage anniversary on February 31st! Yay, what a joyful day!

sincerely,
Janice
```

file named `Albert.docx` will have this content:

```
Dear Albert,

let me congratulate you, dear Albert to your tremendous discovery on that one day! Yay, what a joyful day!

sincerely,
Nikola
```

Try to experiment with the settings / input data to gain experience. In the end, it should save you some time when creating lots of almost identical documents...

## docx-template-filler limitations

currently, there are some usability restrictions on the application:

- whole token text has to be a single DOCX document style

- you have to provide values for ALL of the tokens identified in file
- 
- TODO
