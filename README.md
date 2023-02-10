# bwidman-hash

SQL-like database where you can select what columns you want in your table when you run the program for the first time. You can also filter entries for their column values when selecting or deleting.

## Syntax

Insert entry into the table:

`insert <column1 value> <column2 value> ...`

Delete entries matching specified header/column value:

`delete <header>=<value>`

Select and print entries matching specified header/column value:

`select <header>=<value>`
