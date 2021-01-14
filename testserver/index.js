const express = require('express')
const bodyParser = require('body-parser')

const port = 3007
const app = express()
app.use(bodyParser.json()) // for parsing application/json
app.use(bodyParser.urlencoded({ extended: true })) 
// for parsing application/x-www-form-urlencoded
app.get('/', (req, res) => {
  res.send('Hello World!')
})
app.post('/', (req, res) => {
  console.log(Object.keys(req))
  console.log(req.body)
  res.send('you made a post')
})
app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`)
})
