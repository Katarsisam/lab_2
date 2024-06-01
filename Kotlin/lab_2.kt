import java.util.*

fun Task_1(_N: Int): Int { 
    var N = _N
    val scanner = Scanner(System.`in`) 
    N = scanner.nextInt()
    var ak = 0 
    var al = N
    for (i in 0..N) { 
        if (SimplNumb(i)) { 
            ak = i } 
    } 
    while (!SimplNumb(al)) { 
        al++ 
    } 
    return if (N - ak < al - N) N - ak else al - N }



fun Task_2(): Int { 
    val scanner = Scanner(System.`in`) 
    val a = scanner.nextLine() 
    var c = 0 
for (i in 0..9) { 
    if (a.contains('R' + i) && a.contains('W' + i) && a.contains('B' + i)) { 
        c++ } 
    } 
    return c 
}



fun Task_3(a: Int, len: Int): Int {
    var step = 1
    var X = 0
    var MX = a
    var tempA = a
    for (i in 0 until len) {
        if (tempA % 10 == tempA / 10 % 10 || tempA % 10 == X % 10) {
            step = 0
        } else {
            step++
        }
        X = tempA
        tempA /= 10
    }
    return if (step == 0) MX else 0
}

fun Length(a: Int): Int {
    var X = a
    var len = 0
    while (X > 0) {
        X /= 10
        len++
    }
    return len
}


fun SimplNumb(a: Int): Boolean { 
    if (a > 2) { 
        for (i in 2 until a) { 
            if (a % i == 0) { 
                return false 
                } 
            } 
        return true 
    } else { 
        return false 
    } 
}



fun main() { 
    val scanner = Scanner(System.`in`) 
    var count = 0
    var NumbX: Int
    var k = 0 
    println("Print Number:") 
    var Numb = scanner.nextInt() 
    when (Numb) { 
        1 -> println(Task_1(k)) 
        2 -> println(Task_2()) 
        3 -> {
            NumbX = scanner.nextInt() 
            while (count < NumbX) {
                k = scanner.nextInt()
                println(Task_3(k, Length(k))) 
            	count++ }
        	 } 
        else -> println("Try again") } 
}
