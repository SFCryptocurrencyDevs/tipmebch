const { db } = require('./postgres/database');
  
(async () => {
  await db.sync();
  process.exit(0);
})();