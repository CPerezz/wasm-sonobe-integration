var express = require('express');
var path = require("path");
var app = express();
app.use(express.static(path.join(__dirname, "public")));

var port = process.env.PORT || 3000;


app.get("/", function (req, res) {

    res.render("index");
});

app.listen(port, function () {
    console.log("server is running on port" + port);
});
