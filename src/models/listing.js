// mongoose schema: item

const mongoose = require("mongoose");
const Schema = mongoose.Schema;

const listingSchema = new Schema({
    name: {
        type: String,
        require: true
    },
    description: {
        type: String,
        require: true
    },
    highlight: {
        type: String,
        require: true
    },
    price: {
        type: Number,
        require: true
    }
}, { timestamps: true });

const Item = mongoose.model("Item", itemSchema);
module.exports = Item;
