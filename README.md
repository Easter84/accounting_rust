This is a console application for handling accounts written in Rust. It will connect to a SQLite db and contain the ability to add debtors, and income sources, and hold the ledgers. As the application grows I will add changes to this Readme.

ChangeLog
10/13/2024
* Added a while loop so the application does not close upon selection of options.
* Added 3 new menu options (Bills, Income, Ledger).
* Added a constant to be used for seperating headers from the sub menus.
* Fixed a infinte loop issue.
* Fixed a logic problem with the sub menus not loading properly.
* Refactored the handling of user input to keep with single responsiblity concepts.