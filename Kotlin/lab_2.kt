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



fun Task_3(_a: Int, len: Int) { 
    var a = _a
    var step = 1 
    var numb = 0 
    var X = 0 
    print("Numbers:") 
    for (i in 0 until len) { 
        if (step == 1 && a / 10 != 0) { 
            numb++ 
            print("\n$numb: ") } 
        if (a % 10 == (a / 10) % 10 || a % 10 == X % 10) { 
            print(a % 10) 
            step = 0 
        } else { 
            step++ 
        } 
        X = a
        a /= 10 
    } 
}



fun Lenth(a: Int): Int { 
    var num = a 
    var len = 0 
    while (num != 0) { 
        num /= 10 
        len++ } 
    return len }



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
    
    var k = 0 
    println("Введите число:") 
    var Numb = scanner.nextInt() 
    when (Numb) { 
        1 -> println(Task_1(k)) 
        2 -> println(Task_2()) 
        3 -> {  k = scanner.nextInt()
                Task_3(k, Lenth(k)) 
            	println() 
        	 } 
        else -> println("Попробуйте снова") } 
}
