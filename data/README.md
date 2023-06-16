# Data

This folder is reserved for the data files to import into the service's database.

The data files can be obtained from the official [Japan Post website](https://www.post.japanpost.jp/zipcode/dl/kogaki-zip.html)
or via the provided [utility script](download-and-import.sh).

The data files need to be imported into the database. The service will try this automatically on startup _if_ no data is currently available in the database. This only happens once.

*Notice*: In order for this to work the database will need to be
up and running and also have the correct schema and tables in
place already!

Set the environment variable `JSZIP_DISABLE_AUTO_INIT_DB` to `1` in order to **disable** automatic database initialization with data. 

If data was already imported before the data files are ignored on service startup.

If you want to re-import the data please use the provided utility tools.


# CSV Columns

The CSV columns are explained [here](https://www.post.japanpost.jp/zipcode/dl/readme.html). Translation as follows:

1. Nationwide local government code (JIS X0401, X0402): Half-width numbers
2. (Old) Postal code (5 digits): Half-width numbers
3. Postal code (7 digits): Half-width numbers
4. Prefecture name: half-width katakana (listed in code order) (Note 1)
5. Municipalities: half-width katakana (listed in code order) (Note 1)
6. Town area name: half-width katakana (listed in alphabetical order) (Note 1)
7. Prefecture name: Kanji (listed in code order) (Note 1, 2)
8. Municipality: Kanji (listed in code order) (Note 1, 2)
9. Town area name: Kanji (listed in alphabetical order) (Note 1, 2)
10. Display when one town area is represented by two or more postal codes (Note 3) ("1" applies, "0" does not apply)
11. Display of the town area where the address is numbered for each small letter (Note 4) ("1" is applicable, "0" is not applicable)
12. Display for town areas with chome ("1" is applicable, "0" is not applicable)
13. Display when one zip code represents two or more town areas (Note 5) ("1" applies, "0" does not apply)
14. Indication of updates (Note 6) (“0” is unchanged, “1” is changed, “2” is obsolete (uses obsolete data only))
15. Reasons for change ("0" no change, "1" implementation of municipal administration, ward administration, town administration, sub-districts, ordinance-designated cities, "2" implementation of address indication, "3" land readjustment, "4" adjustment of postal wards, etc. "5" Correction, "6" Obsolete (only obsolete data used))

Note 1: The character code uses the MS Kanji code (SHIFT JIS).

Note 2: JIS X0208-1983 is used as the character set, and characters not specified are written in hiragana.

Note 3: "Display when one town area is represented by two or more postal codes" refers to town areas where the postal code cannot be specified by just the town area, and the number differs depending on chome, address, small letter, etc.

Note 4: "Display of town area where number is started for each small character" means that the town area (large character) for which the postal code is set has multiple small characters, and each small character is numbered. Therefore, it is a town area where the address cannot be specified only by the town area (postal code) and street address.

Note 5: "Indication when one zip code represents two or more town areas" means that one zip code represents multiple town areas and indicates that the address cannot be specified only by the zip code and street address. am.

Note 6: "Modified" indicates updated data with additions and corrections.

Note 7: If the full-width town area name exceeds 38 characters, or if the half-width katakana town area name exceeds 76 characters, it is split into multiple records.
