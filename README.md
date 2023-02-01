# traewelling dashboard rss

Create a RSS feed from your traewelling dashboard.

Public instance: [https://traewelling-rss.adridoesthings.com](https://traewelling-rss.adridoesthings.com)

# How it works
## How to create your rss feed?
Go to the url of your instance (e.g. the public instance): Follow the steps in traewelling and you will get a rss url. *Don't share this url with other people, because YOUR dashboard is private*

## Delete the rss feed
Just run a `DELETE` HTTP request to the rss url.

# Host your own instance
Just use the docker image published on github or run the binary with `cargo` yourself. You need to create a docker volume to `/database`.

The redirect_uri of the traewelling oauth grant is `$PUBLIC_URL/callback`.

## Environment variables
- DATABASE_PATH = The path to your sqlite file (Docker default = /database/db.sql)
- TRAEWELLING_CLIENT_ID = Get your client id [here](https://traewelling.de/settings/applications)
- TRAEWELLING_CLIENT_SECRET = Get your client secret [here](https://traewelling.de/settings/applications)
- PUBLIC_URL = The public url of your instance