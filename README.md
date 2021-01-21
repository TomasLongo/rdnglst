# Rdnglst

Small tool to manage your readinglist, e.g. a list containing all your articles, books, e-books you read or want to read along with some metadata in tabular form.

[image]

## Installation

Download a binary release (only mac at the moment) or clone the source and build it. A simple `Cargo build --release` should do the trick.

## Usage

`readinglist -h` presents some information on how to use the tool.

### Print the whole list

Simply invoking `readinglist` will print all the items in your list as a nicely formated table.

### Add a new item

Use `readinglist add` to add a new item your list. The tool asks you to enter information about your reading item, every piece of data beeing optional.

### Update an item

In order to update an item you first have to obtain the id of the item. `readinglist -i` prints an additional column showing the id of every item.

With the id at your disposal invoke `readinglist update [id]`. The tool will, again, ask you to fill in the information for the item, this time pre filling the prompts with the data already stored. You can then modify the fields or simply press enter at fields you don't want edit.

### Delete an item

`readinglist delete [id]` after getting the id of the item you want to delete (see section about updating an item).

## Data fields on your items

The kind of data your reading list holds about your reading items is static at the moment. I have plans to enable the user to define her own set of data fields.

The current fields are:

* Author
* Title
* Form (whether it is a book, an online article, e-book or whatever you want.)
* Tags (arbitrary metadata to further describe what the reading item is about)
* Status (whether your are currently reading the item or want to read it or already read it)
* Genre

## Filter your item list

`rdnglst` provides a small querylanguage if you want to filter your reading list by some kind of condition. Use the `-q` Option to specify a query. Here are some examples:

* To filter for a certain author invoke `readinglist -q 'author is "Neil Gaiman"'`
* To only show items you already read: `readinglist -q 'status is read'`

> You maybe noticed that the first example encloses the author in double quotes while the second example doesn't. Everytime, the value you want to compare your field against is made up of multiple word, doublequotes are required. You can omit the double quotes if you want to compare your fields against a single word.

### Anatomy of a query

The querylanguage is centered around comparing the fields that describe your reading items against some value you provide in the query. The basic structure of a query is

```
fieldname comparator value
```

Where fieldname ist a field of your item and `comparator` is a keyword describing how the value in the field should be compared against the value you provided. 


### Current limitations and further plans for the querylanguage

At the moment only them comparator `is` is supported which performs an exact comparison between the field of your readin item and the provided value.

Given an item with by the author `Neil Gaiman` the query `author is "Neil Gaiman"` would return the item but `author is Gaiman` would not, since the tool compares "Neil Gaiman" against `Gaiman`. 

Obviously, this is not sufficient to execute fuzzy queries, where we would like to get all items where the author's name contains, for example, `John`.

Also, asking `or` questions is not supported at the moment (e.g. `author is "Neil Gaiman" or "Nassim Taleb"`).
