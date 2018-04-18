# tipmebch SF Cryptocurrency Devs

Tipping for Telegram with Bitcoin Forked for SF Cryptocurrency Devs to get regular developers building Cyrptocurrency Applications for fun and skills improvement. Tip me BCH!

![Logo](https://raw.githubusercontent.com/abrkn/tipmebch/master/logo.png)

## Warning

This software is highly experimental and may lead to loss of funds.
The author takes no responsibility for your money.

## Developer Github Repo Forking instructions
### Step 1: Fork a copy of this Repo into your own Github account.
Fork on Github into your own repo

### Step 2: Clone a copy from Github to a copy on your computer
```shell
git clone git@github.com:your-username/tipmebch.git
```

### Step 3: Change directorirs to the tipbot directory
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
git rebase master
```

### Step 7: Go to Github and make a Pull Request and The SF Cryptocurrency Devs Telegram should alert with a post!


## Installation

```bash
npm install
```

## Configuration

Define the environment variables:

```bash
export TELEGRAM_BOT_TOKEN=496343161:yoursecret
export BITCOIND_URL=http://rpcuser:rpcpassword@localhost:8333
export STAFF_USER_ID=403107081
export TELEGRAM_BOT_USERNAME=TipMeBchBot
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

## Author

Andreas Brekken <mailto:andreas@brekken.com>
