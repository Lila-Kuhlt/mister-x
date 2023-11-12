# Mister X
Play a game of Scotland Yard with your friends using public transport.

![image](https://github.com/Lila-Kuhlt/mister-x/assets/21245806/1170547b-d7e2-4bb1-937f-acad2c609984)


## Setup
**Requirements:**
- rust
- npm

To be able to use the TRIAS API, you need an API Token which you can put into a file called `robusta/.env` with the following structure:
```
TRIAS_ACCESS_TOKEN=YOUR_TOKEN
```


## Development
For development first navigate into the `robusta` directory and execute the `cargo run` command.
Then after that has completed and a server is hosted, navigate to the the `liberica` directory in a new terminal and execute `npm run dev`.
This should give you a url to the live server which you can use for local development.

# Deployment
For deployment simply execute `cargo run` and proxy the https connection using something like nginx. The frontend will be hosted by the rust server as well.
