#!/bin/bash
echo Building Markdown pages...
rm md/*.html
rm md/*/*.html
mkdir md
python process.py
echo Building GitBook format...
gitbook build --output=./book
echo Structuring webpage...
cp book/index.html index.html
rm -rf gitbook
rm -rf md
cp -ar book/gitbook gitbook
cp -ar book/md md
cp book/index.html index.html
cp book/manifest.appcache manifest.appcache
cp book/search_index.json search_index.json
rm -rf book
