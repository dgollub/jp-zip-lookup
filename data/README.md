# Data

This folder is reserved for the data files to import into the service's database.

*TODO* The data files can be obtained from the official [Japan Post website](https://www.post.japanpost.jp/zipcode/dl/kogaki-zip.html)
or via the provided utility tools. (TODO)

The data files then need to be imported into the database. The service will try this automatically on startup _if_ no data is currently available in the database.

If data was already imported before the data files are ignored on service startup.

If you want to re-import the data please use the provided utility tools.

*TODO* usage/howto examples

maybe the service can be started with a "force reimport" flag?
