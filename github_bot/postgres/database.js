const {
    Sequelize,
    Op,
  } = require('sequelize');
  
  const db = new Sequelize(process.env.POSTGRES_URI, {
    dialect: 'postgres',
    operatorsAliases: false,
    logging: false,
  });
  
  db.authenticate()
    .then(() => console.log(`\nConnected to "${process.env.POSTGRES_URI}".`))
    .catch((error) => {
      console.log(`\nUnable to connect to database: ${error}`);
      process.exit(1);
    });

const User = db.import('./models/User.js');

const getUser = async githubId => {
    githubId = String(githubId);
    try {
        const resp = await User.findOne({ where: { githubId } });
        if (!resp) return resp;
        return resp.dataValues;
    } catch (err) {
        console.log(err)
        return null;
    }
}

const getAllUsers = async () => {
    try {
        const resp = await User.findAll({ });
        if (!resp) return resp;
        return resp.map(x => x.dataValues);
    } catch (err) {
        console.log(err)
        return null;
    }
}

const addUser = async githubId => {
    githubId = String(githubId);
    try {
        let resp = await User.create({
            githubId: githubId,
            stellarAddress: 'missing',
            balance: 0,
        })
        return resp;
    } catch (err) {
        console.log(err)
        return null;
    }
}

const incrementBalance = async (githubId, amt) => {
    githubId = String(githubId);
    amt = String(amt);
    try {
        let checkExists = await getUser(githubId);
        if (!checkExists) {
            await addUser(githubId);
        }
        let resp = await User.update({ balance: Sequelize.literal(`balance + ${amt}`) }, { where: { githubId } });
        return resp;
    } catch (err) {
        console.log(err)
        return null;
    }
}

const withdrawBalance = async (githubId, amt) => {
    githubId = String(githubId);
    amt = String(amt);
    try {
        let resp = await User.update({ balance: 0 }, { where: { githubId } });
        return resp;
    } catch (err) {
        console.log(err)
        return null;
    }
}

const updateStellarAddress = async (githubId, stellarAddress) => {
    githubId = String(githubId);
    stellarAddress = String(stellarAddress);
    try {
        let checkExists = await getUser(githubId);
        if (!checkExists) {
            await addUser(githubId);
        } else 
        await User.update({ stellarAddress }, { where: { githubId } });
        return true;
    } catch (err) {
        console.log(err)
        return null;
    }
}


module.exports = {
    db,
    getUser,
    getAllUsers,
    addUser,
    incrementBalance,
    withdrawBalance,
    updateStellarAddress,
}