// See https://aka.ms/new-console-template for more information
using System;

class Program
{
    static void Main()
    {
        int Numb;
        int k = 0;
        int len;
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
                len = Lenth(ref k);
                Task_3(k, len);
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

    static void Task_3(int a, int len)
    {  
        int step = 1, numb = 0;
        int X = 0;
        Console.Write("Numbers:");
        for (int i = 0; i < len; i++)
        {
            if (step == 1 && a / 10 != 0)
            {
                numb++;
                Console.Write($"\n{numb}: ");
            }
            if (a % 10 == (a / 10) % 10 || a % 10 == X % 10)
            {
                Console.Write(a % 10);
                step = 0;
            }
            else
            {
                step++;
            }
            X = a;
            a /= 10;
        }
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
