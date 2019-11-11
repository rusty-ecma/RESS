const unit = 7.6;
const end_start = 24;
const sec = 3.3;

let selectors = '';
let frames = '';
let ends = [sec * 6, (sec * 6) * 2, (unit * 6) * 3];
function selector(i, len = 24) {
    return `
    #index-${i} {
        animation-name: key-${i};
        animation-duration: ${len}s;
    }
    `;
}
function key(i, start, end) {
    return `
    @keyframes key-${i} {
        0% {
            fill: none;
        }
        ${start.toFixed(2)}% {
            fill: black;
        }
        ${end.toFixed(2)}% {
            fill: none;
        }
        100% {
            fill: none;
        }
    }
    `;
}
selectors += selector(0);
frames += key(0 , 0, unit);
selectors += selector(1);
frames += key(1, sec * 2, unit * 2);
selectors += selector(2);
frames += key(2, sec * 3, unit * 3);

for (var i = 3; i < 11; i++) {
    selectors += selector(i);
    let start = ends.shift();
    let end = start + sec * 6;
    frames += key(i, start, end);
    ends.push(end);
}

console.log(selectors);
console.log(frames);