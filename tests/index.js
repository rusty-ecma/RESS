'use strict';


function Thing(name) {
    this.name = name;
}
Thing.prototype.getName = function() {
    return this.name;
};

Thing.prototype.setDoubleName = function(first, last) {
    this.name = first + ' ' + last;
};
