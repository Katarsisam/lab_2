function Task_1(N) {
    let ak, al;
    N = parseInt(prompt("Enter a number:"));
    for (let i = 0; i <= N; i++) {
        if (SimplNumb(i)) {
            ak = i;
        }
    }
    for (al = N; !SimplNumb(al); al++);
    return N - ak < al - N ? N - ak : al - N;
}

function Task_2() {
    let a = prompt("Enter a string:");
    let c = 0;
    for (let i = 0; i <= 9; i++) {
        if (a.includes('R' + i) && a.includes('W' + i) && a.includes('B' + i)) {
            c++;
        }
    }
    return c;
}

function Task_3(a, len) {
    let step = 1;
    let X = 0;
    let MX = a;
    for (let i = 0; i < len; i++) {
        if (a % 10 === Math.floor(a / 10) % 10 || a % 10 === X % 10) {
            step = 0;
        } else {
            step++;
        }
        X = a;
        a = Math.floor(a / 10);
    }
    return step === 0 ? MX : 0;
}

function Lenth(a) {
   
    let len = 0;
    let X = a;
    while (X) {
        X = Math.floor(X / 10);
        len++;
    }
    return len;
}

function SimplNumb(a) {
    if (a > 2) {
        for (let i = 2; i < a; i++) {
            if (a % i === 0) return false;
        }
        return true;
    } else {
        return false;
    }
}

function main() {
    let Numb, k, count = 0, NumbX;
    Numb = parseInt(prompt("Enter a number:"));
    switch (Numb) {
        case 1:
            console.log(Task_1(k));
            break;
        case 2:
            console.log(Task_2());
            break;
        case 3:
            NumbX = parseInt(prompt());
            console.log("Numbers:");
            while (count < NumbX) {
                k = parseInt(prompt());
                console.log(Task_3(k, Lenth(k)));
                count++;
            }
            console.log("\n");
            break;
        default:
            console.log("Try again");
            break;
    }
}

main();