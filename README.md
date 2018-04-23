# tipmebch SF Cryptocurrency Devs

Tipping for Telegram with Bitcoin Forked for SF Cryptocurrency Devs to get regular developers building Cyrptocurrency Applications for fun and skills improvement. Tip me BCH!

![Logo](https://raw.githubusercontent.com/abrkn/tipmebch/master/logo.png)

## Warning

This software is highly experimental and may lead to loss of funds.
The author takes no responsibility for your money.

## Where to get started?
Ah, great question! In fact, we anticipated this very question. To make it as easy as possible to jump right in, we have
created project proposals within the issues of this repo. The project proposals are denoted with the Project Proposal label.
To make it even easier to distinguish between the different project proposals, we have labeled them:

* JavaScript or Rust
* Cryptocurrency integration or Chat Bot Integration
* Work in Prgress (WIP) or Not Yet Started (NYS)

## Developer Github Repo Forking instructions
### Step 1: Fork a copy of this Repo into your own Github account.
Fork on Github into your own repo

### Step 2: Clone a copy from Github to a copy on your computer
```shell
git clone git@github.com:your-username/tipmebch.git
```

### Step 3: Change directories to the tipbot directory
```shell
cd tipmebch
```

### Step 4: Add the SF Cryptocurrency Devs Repo to your upstream
```shell
git remote add upstream git@github.com:SFCryptocurrencyDevs/tipmebch.git
```

### Step 5: Begin working on your branch idea and save your work!
```shell
git checkout -b new-branch-feature
git commit -am 'Your updated commits'
```

### Step 6: Update and rebase your local copy with the SF Cryptocurrency Devs Repo as there may be changes
```shell
git fetch upstream
git checkout master
git merge upstream/master
git checkout new-branch-feature
git push
```

### Step 7: Go to Github and make a Pull Request and The SF Cryptocurrency Devs Telegram should alert with a post!


## Installation

### For Mac OS Add Home Brew GCC to your Homebrew
```shell
/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
brew install gcc
```

### For Linux Install the Developer Tools
```shell
sudo yum groupinstall 'Development Tools'
```

Install rust https://www.rust-lang.org/en-US/install.html
```
curl https://sh.rustup.rs -sSf | sh
```

# Add rust to your environment path
```
source $HOME/.cargo/env
```

```bash
brew install nodejs
npm install
```

## Configuration

Define the environment variables:

```bash
export TELEGRAM_BOT_TOKEN=yourbottoken:yoursecret
export BITCOIND_URL=http://rpcuser:rpcpassword@localhost:8333
export STAFF_USER_ID=403107081
export TELEGRAM_BOT_USERNAME=YourBotUserName
export DEFAULT_STICKER_SET=pepe
export REDIS_URL=redis://localhost
```

## Running

```bash
npm start
```

## Stickers sets

See [sticker set documentation](docs/stickers.md)

## License

See [LICENSE](LICENSE)

## Authors

1. Andreas Brekken <mailto:andreas@brekken.com>
2. SF Cryptocurrency Devs San Francisco <http://SFCryptocurrencyDevs.com>
