const octonode = require('octonode');
const stellar = require('./stellar/stellar');

const db = require('./postgres/database');

const client = octonode.client(process.env.WRITE_AUTH_TOKEN_GITHUB);

const handleGithubWebhook = async (data) => {
    if (data.ref_type !== 'branch' && !data.ref) {
      const { action } = data;
      let githubID;
  
      if (data.pull_request && !data.issue) {
        githubID = data.pull_request.id;
      } else if (data.issue.pull_request && data.issue) {
        const prObj = client.pr(data.repository.full_name, data.issue.number);
        const prInfo = await prObj.infoAsync();
        githubID = prInfo[0].id;
      } else {
        githubID = data.issue.id;
      }
        try {
          // Parse information
          switch (action) {
              case 'created': {
                  if (data.issue.number === process.env.BOT_INTERACTION_ISSUE_NUMBER && data.comment.body.split(" ")[0] == "/register") {
                      if (StellarSdk.StrKey.isValidEd25519PublicKey(data.comment.body.split(" ")[1].trim())) {
                        let resp = await db.updateStellarAddress(data.comment.user.id, data.comment.body.split(" ")[1].trim());
                        if (resp) await client.issue('process.env.GITHUB_REPO', process.env.BOT_INTERACTION_ISSUE_NUMBER).createCommentAsync({body:"New key registered 👍" })
                        else await client.issue('process.env.GITHUB_REPO', process.env.BOT_INTERACTION_ISSUE_NUMBER).createCommentAsync({body:"Invalid key 👎" })
                      } else {
                        await client.issue('process.env.GITHUB_REPO', process.env.BOT_INTERACTION_ISSUE_NUMBER).createCommentAsync({body:"Invalid key 👎" })
                      }
                  } else if (data.issue.number === process.env.BOT_INTERACTION_ISSUE_NUMBER && data.comment.body.trim() == "/claim") {
                    let user = await db.getUser(data.comment.user.id);
                    if (!user || user.stellarAddress == 'missing') {
                      await client.issue('process.env.GITHUB_REPO', process.env.BOT_INTERACTION_ISSUE_NUMBER).createCommentAsync({body: `Hello ${data.comment.user.login}, first you need to register a Stellar Address. You can do this by using the /register STELLAR_ADDRESS command.` })
                    } else if (user.balance == 0) {
                      await client.issue('process.env.GITHUB_REPO', process.env.BOT_INTERACTION_ISSUE_NUMBER).createCommentAsync({body: `Hello ${data.comment.user.login}. Unfortunately you cannot withdraw because your balance is 0.` })
                    } else {
                      const botHasMoney = await stellar.botHasMoney()
                      if (botHasMoney) {
                        const exists = await stellar.accountExists(user.stellarAddress);
                        if (exists) {
                          await stellar.sendXLM(user.stellarAddress);
                        } else {
                          await stellar.createAccount(user.stellarAddress);
                        }
                        await db.withdrawBalance(data.comment.user.id);
                        await client.issue('process.env.GITHUB_REPO', process.env.BOT_INTERACTION_ISSUE_NUMBER).createCommentAsync({body: `Congrats ${data.comment.user.login}! We will see you on the moon.` })
                      } else {
                        await client.issue('process.env.GITHUB_REPO', process.env.BOT_INTERACTION_ISSUE_NUMBER).createCommentAsync({body: `Sorry ${data.comment.user.login}, the bot needs to refuel. Try again later!` })
                      }
                      
                    }
                  }
              }
            case 'closed': {
              if (data.pull_request && data.pull_request.merged_at) {
                await db.incrementBalance(data.pull_request.user.id, process.env.AWARD_AMOUNT);
                await client.issue('process.env.GITHUB_REPO', data.number).createCommentAsync({body:`Thanks for contributing. Your account has been awarded ${process.env.AWARD_AMOUNT}XLM. To claim it, go to the BOT INTERACTION issue (#${process.env.BOT_INTERACTION_ISSUE_NUMBER}). There you will need to /register your Stellar address and then you can /claim your bounty!` })
              }
              break;
            }
            default:
              // TODO: deal with defult case
          }
        } catch (err) {
          console.log('error', err);
        }
    }
  };

  module.exports = {
    handleGithubWebhook
  }