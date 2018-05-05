module.exports = (db, Sequelize) => (
    db.define('user', {
      id: {
        type: Sequelize.INTEGER,
        primaryKey: true,
        autoIncrement: true,
      },
      githubId: {
        type: Sequelize.STRING,
        unique: true,
      },
      stellarAddress: {
        type: Sequelize.STRING,
      },
      balance: {
        type: Sequelize.INTEGER,
      },
    }, {
      underscored: true,
    })
  );