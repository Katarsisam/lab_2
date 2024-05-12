import Foundation

func task1(_ n: inout Int) -> Int {
    var ak = 0
    var al = 0
    let n = Int(readLine(strippingNewline: true)!)!  
    for i in 0...n {
        if simplNumb(i) {
            ak = i
        }
    }
    al = n
    while !simplNumb(al) {
       
        al += 1
        
    }
    return n - ak < al - n ? n - ak : al - n
}

func task2() -> Int {
    let a = readLine() ?? ""
    var c = 0
    for i in 0...9 {
        if a.contains("R\(i)") && a.contains("W\(i)") && a.contains("B\(i)") {
            c += 1
        }
    }
    return c
}

func task3(_ a : inout Int, _ len : Int) {
    var step = 1
    var numb = 0
    var X = 0
    print("Numbers:")
    var tempA = a
    for _ in 0..<len {
        if step == 1 && tempA / 10 != 0 {
            numb += 1
            print("\n\(numb): ", terminator: "")
        }
        if tempA % 10 == (tempA / 10) % 10 || tempA % 10 == X % 10 {
            print("\(tempA % 10)", terminator: "")
            step = 0
        } else {
            step += 1
        }
        X = tempA
        tempA /= 10
    }
}

func length(_ a: Int) -> Int {
    var len = 0
    var X = a
    while X > 0 {
        X /= 10
        len += 1
    }
    return len
}

func simplNumb(_ a: Int) -> Bool {
    if a > 2 {
        for i in 2..<a {
            if a % i == 0 {
                return false
            }
        }
        return true
    } else {
        return false
    }
}

let numb = Int(readLine(strippingNewline: true)!)!  
var k = 0

switch numb {
case 1:
    print(task1(&k))
case 2:
    print(task2())
case 3:
    var k = Int(readLine(strippingNewline: true)!)!  
    task3(&k, length(k))
    print("\n")
default:
    print("Try again")
}