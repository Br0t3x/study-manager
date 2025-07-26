## Running
To run this app, you only need to have Rust installed. Just type ``cargo build`` and you should be able to build the app

## Commands
The main commands are listed below:
- new - these two commands can create a new subject, chapter or exercise
  - subject <name> will create a new subject with the specified name
  - chapter <name> will create a new chapter with the specified name
- edit - let you change almost everything about a subject, chapter and exercise;
- list - one of the greatest commands here, it can list all subjects and chapters you've created
  - subject - it will list every subject and its numeric id
  - chapter - it will list every chapter and its associated id in the current selected subject
- select - it selects the subject or chapter specified
  - subject <numeric-id> - it will select the specified subject and save it in the config file
  - chapter <numeric-id> it will select the specified chapter and save it in the config file
- remove - remove a range of subjects, chapters and exercises
- report - makes you a report of all data. It is useful especially for visualizing a timeline of your studies