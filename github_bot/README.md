# The Stellar Github Bounty Bot

![https://media.giphy.com/media/jXOdWw9pKxuU/giphy.gif](https://media.giphy.com/media/jXOdWw9pKxuU/giphy.gif)

Hello and welcome to the Github Bounty Bot. This is a very simple bounty bot that awards people per merged pull request. 

## Setup
First, you will need to setup a postgres database. You can call this database whatever you want -- just make sure to specify the db name in the uri.

If you are running Ubuntu, [here is a great place to get started](https://gist.github.com/j1n3l0/c69280f039884e41e6cd3cb80163be07) with postgres.

[Here is also the command to get started setting](https://stackoverflow.com/questions/11919391/postgresql-error-fatal-role-username-does-not-exist) up your db (it helped me, maybe it will help you).

You will also need node.js. [Here is how to get node](https://askubuntu.com/questions/993975/how-can-i-install-npm-on-17-10) on Ubuntu.

Setup your environment variables:
```
STELLAR_SECRET_KEY => stellar secret key for the bot account
STELLAR_BASE_URL => the url of the horizon instance you wish to communicate with (horizon.stellar.org is the SDF public network url)
AWARD_AMOUNT => however many XLM you want to award per merged pull request
WRITE_AUTH_TOKEN_GITHUB => your github personal access token
BOT_INTERACTION_ISSUE_NUMBER => the issue number of the BOT INTERACTION CHANNEL where users will talk to the bot
POSTGRES_URI => "postgres://USERNAME:PASSWORD@localhost:PORT/DATABASE_NAME"
GITHUB_REPO => "github_organization/repo_name"
```

Fund your bot Stellar account... if this bot has 0 XLM, you won't be able to reward anyone.

## Quickstart
```
npm run sync
npm start
```

*Note:* will start on port 3003.

## Commands

**/claim**: claim your reward by withdrawing the XLM to the Stellar address you registered
**/register**: associate a stellar address with your github id

## How It Works
This bot is very simple. It uses `postgres` as the backend to keep track of user rewards. The schema is as follows:

**Github ID:** The id associated with a user's github account.
**Stellar Address:** The stellar public key where a user wishes to withdraw funds.
**Balance:** The balance of the user -- their reward pot.

Basic flow:
1. User communicates that he/she wants to tackle an open issue -- or proposes their own issue to complete.
2. User writes code.
3. User opens a pull request.
4. Maintainer merges pull request.
5. Bot updates balance of user w/ + AWARD_AMOUNT in postgres.
6. User navigates to the issue with the name `BOT INTERACTION CHANNEL`.
7. User registers a stellar address for their account with the `/register` command in a comment.
8. User withdraws/claims their reward with the `/claim` command in a comment.
9. User, high off of the adreline rush of being rewarded for their contribution, repeats 1-9.