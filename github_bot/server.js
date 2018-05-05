const bodyParser = require('body-parser');
const express = require('express');

const test = require('./github');

const app = express();
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

app.use(function(req, res, next) {
  res.header("Access-Control-Allow-Origin", "*");
  res.header("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept");
  next();
});

app.post('/github', (req, res) => {
  test.githubToAsana(req.body);
  res.sendStatus(200);
});

app.listen(process.env.PORT || 3003, () => {

});
