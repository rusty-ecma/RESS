const esprima = require('esprima');

function check_section() {
    let js = `function () {
    return diff === first || ( diff % first === 0 && diff / first >= 0 );
}`;

    esprima.tokenize(js);
}

check_section();