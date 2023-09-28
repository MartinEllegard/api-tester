# Api Testing tool
## Description
This software is written for testing backends in a synthetic way. 
The tool gathers information about how long each request took and how big the response object was. 
Optionally it can also track a internal consumption metric or internal dependancy duration through these two optional headers:

  - duration
  - consumption

Currently it only supports writing to csv

## Use (Prebuilt)
1. Download latest release
2. Create a directory for the application
3. In the directory create an .env file
4. Follow the configuration section .env options

## Configuration
These are the fields the app supports in .env
```
URL="https://foo.bar"
QUERYSTRINGS="?foo=f&bar=b&baz=bz"
WORKERS=4
REQUESTS=1000
REQUESTHEADER1="header_name header_value"
REQUESTHEADER2="header_name header_value"
REQUESTHEADER3="header_name header_value"
```
- *URL* Mandatory
- *QUERYSTRINGS* Optional
- *WORKERS* Mandatory - Defines how many request will be sent concurrently
- *REQUESTS* Mandatory - Number of total requests sent
- *REQUESTHEADER{number}* - Optional - Define headers for request key and value seperated by space - Keys and values can not contain spaces
