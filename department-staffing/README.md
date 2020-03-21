# Department Staffing

https://doc.rust-lang.org/book/ch08-03-hash-maps.html

Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

## Notes

Much more input validation is required to make this robust. Additionally, I'd like to avoid the multi-branch conditional statement that switches on the first word entered in each interaction. An initial improvement could be to make it case-insensitive. As a first go at this, I chose to store the team members in alphabetical order, rather than deal with that on the display side of things. Ideally, the display could be abstracted, though, and accommodate any number of formats.
