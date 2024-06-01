
using System;

class Program
{
    static void Main()
    {
        int Numb, k = 0, count = 0, NumbX;
        Numb = int.Parse(Console.ReadLine());
        switch (Numb)
        {
            case 1:
                Console.WriteLine(Task_1(out k));
                break;
            case 2:
                Console.WriteLine(Task_2());
                break;
            case 3:
                NumbX = int.Parse(Console.ReadLine());
                Console.WriteLine("Numbers:");
                while (count < NumbX)
                {
                    Console.WriteLine(Task_3(ref k, Lenth(ref k)));
                    count++;
                }
                Console.WriteLine();
                break;
            default:
                Console.WriteLine("Try again");
                break;
        }
    }

    static int Task_1(out int N)
    {
        int ak = 0, al = 0;
        N = int.Parse(Console.ReadLine());
        for (int i = 0; i <= N; i++)
        {
            if (SimplNumb(i))
            {
                ak = i;
            }
        }
        for (al = N; !SimplNumb(al); al++) ;
        return N - ak < al - N ? N - ak : al - N;
    }

    static int Task_2()
    {
        string a = Console.ReadLine();
        int c = 0;
        for (int i = 0; i <= 9; i++)
        {
            if (a.Contains("R" + i) && a.Contains("W" + i) && a.Contains("B" + i))
            {
                c++;
            }
        }
        return c;
    }

    static int Task_3(ref int a, int len)
    {
        int step = 1;
        int X = 0;
        int MX = a;
        for (int i = 0; i < len; i++)
        {
            if (a % 10 == (a / 10) % 10 || a % 10 == X % 10)
            {
                step = 0;
            }
            else
            {
                step++;
            }
            X = a;
            a /= 10;
        }
        return step == 0 ? MX : 0;
    }

    static int Lenth(ref int a)
    {
        a = int.Parse(Console.ReadLine());
        int len = 0, X = a;
        while (X > 0)
        {
            X /= 10;
            len++;
        }
        return len;
    }

    static bool SimplNumb(int a)
    {
        if (a > 2)
        {
            for (int i = 2; i < a; i++)
            {
                if (a % i == 0)
                {
                    return false;
                }
            }
            return true;
        }
        else
        {
            return false;
        }
    }
}
