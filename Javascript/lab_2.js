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
    let step = 1, numb = 0;
    let X = 0;
    console.log("Numbers:");
    for (let i = 0; i < len; i++) {
        if (step === 1 && Math.floor(a / 10) !== 0) {
            numb++;
            console.log("\n" + numb + ": ");
        }
        if (a % 10 === Math.floor(a / 10) % 10 || a % 10 === X % 10) {
            console.log(a % 10);
            step = 0;
        } else {
            step++;
        }
        X = a;
        a = Math.floor(a / 10);
    }
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
    let Numb, k;
    Numb = parseInt(prompt("Enter a number:"));
    switch (Numb) {
        case 1:
            console.log(Task_1(k));
            break;
        case 2:
            console.log(Task_2());
            break;
        case 3:
            k = parseInt(prompt("Enter a number:"));
            Task_3(k, Lenth(k));
            console.log("\n");
            break;
        default:
            console.log("Try again");
            break;
    }
}

main();