const express = require("express");
// const Account = require("./models/account");
// const Item = require("./models/item");


// express app
const app = express();


// middleware - log each request
app.use((req, res, next) => {
    console.log("new request made:");
    console.log("hostname: ", req.hostname);
    console.log("path: ", req.path);
    console.log("method: ", req.method);
    next();
});


// direct
app.get("/", (req, res) => {
    res.sendFile("./views/index.ejs", { root: __dirname });
});

// redirect
app.get("/home", (req, res) => {
    res.redirect("/");
});


// 404 or error
app.use((req, res) => {
    res.sendFile("./views/404.ejs", { root: __dirname });
});
