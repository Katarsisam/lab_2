import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        int Numb = scanner.nextInt();
        int k = 0;
        int count = 0;
        switch (Numb) {
            case 1:
                System.out.println(Task_1(k));
                break;
            case 2:
                System.out.println(Task_2());
                break;
            case 3:
                int NumbX = scanner.nextInt();
                while (count < NumbX) {
                    k = scanner.nextInt();
                    System.out.println(Task_3(k, Lenth(k)));
                    count++;
                }
                break;
            default:
                System.out.println("Try again");
                break;
        }
    }

    public static int Task_1(int N) {
        int ak = 0, al = 0;
        Scanner scanner = new Scanner(System.in);
        N = scanner.nextInt();

        for (int i = 0; i <= N; i++) {
            if (SimplNumb(i)) {
                ak = i;
            }
        }

        for (al = N; !SimplNumb(al); al++) ;

        return N - ak < al - N ? N - ak : al - N;
    }

    public static int Task_2() {
        Scanner scanner = new Scanner(System.in);
        String a = scanner.nextLine();
        int c = 0;

        for (int i = 0; i <= 9; i++) {
            if (a.indexOf('R' + i) != -1 && a.indexOf('W' + i) != -1 && a.indexOf('B' + i) != -1) {
                c++;
            }
        }

        return c;
    }

    public static int Task_3(int a, int len) {
        int step = 1;
        int X = 0;
        int MX = a;
        for (int i = 0; i < len; i++) {
            if (a % 10 == (a / 10) % 10 || a % 10 == X % 10) {
                step = 0;
            } else {
                step++;
            }
            X = a;
            a /= 10;
        }
        return step == 0 ? MX : 0;
    }

    public static int Lenth(int a) {
        int len = 0;
        int X = a;
        while (X > 0) {
            X /= 10;
            len++;
        }
        return len;
    }

    public static boolean SimplNumb(int a) {
        if (a > 2) {
            for (int i = 2; i < a; i++) {
                if (a % i == 0) {
                    return false;
                }
            }
            return true;
        } else {
            return false;
        }
    }
}
