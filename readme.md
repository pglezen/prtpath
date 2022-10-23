# Print PATH Environment Variable

This is a simple utility to print the entries from
your PATH environment variable in a tabular format
with indicators for invalid and duplicate entries.

--------

## Usage

By default it prints the PATH variable entries.
But you can specify any other variable you want.

```
$ prtpath
Index Status    Path
   1   dir      /Users/me/Applications/miniconda3/condabin
   2   dir      /Users/me/.nvm/versions/node/v14.17.3/bin
   3   dir      /usr/local/bin
   4   dir      /usr/bin
   5   dir      /bin
   6   dir      /usr/sbin
   7   dir      /sbin
   8   dir      /Library/TeX/texbin
```

-------

## Find Problems

It can sniff out things like duplicate directories
or malformed entries.  For example, if I add the following
two entries to the front of the PATH:

* `/usr/bin/` - a duplicate of entry 4 above (even though one
  ends with a `/` and the other doesn't),
* `/usr/blah` - a path which doesn't exist,

the output shows:

```
$ prtpath
Index Status    Path
   1   dir      /usr/bin/
   2  -invalid- /usr/blah
   3   dir      /Users/me/.nvm/versions/node/v14.17.3/bin
   4   dir      /Users/me/Applications/miniconda3/condabin
   5   dir      /usr/local/bin
   6  -dup----- /usr/bin
   7   dir      /bin
   8   dir      /usr/sbin
   9   dir      /sbin
  10   dir      /Library/TeX/texbin
```

Entry 2 is invalid and entry 6 becomes a duplicate of entry 1.

-------

## Not Just for PATH

You can specify any other variable you'd like.

```
$ prtpath MANPATH
Index Status    Path
   1   dir      /Users/me/.nvm/versions/node/v14.17.3/share/man
   2   dir      /usr/local/share/man
   3   dir      /usr/share/man
   4   dir      /Library/TeX/texbin/man
   5   dir      /usr/local/MacGPG2/share/man
   6   dir      /opt/X11/share/man
   7   dir      /Library/Apple/usr/share/man
```