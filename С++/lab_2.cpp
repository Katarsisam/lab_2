#include <iostream>

#include <stdio.h>
#include <string>
#include <cmath>
#include <locale>
int Task_1(int &N);
int Task_2();
int Task_3(int &a, int len);
int Lenth(int &a);
int SimplNumb(int &a);
using namespace std;
int main(){
    int Numb, k, count = 0, NumbX;
    setlocale(LC_ALL,"rus");
    scanf("%d", &Numb);
    switch (Numb)
    {
    case 1:
        printf("%d\n",Task_1(k));
        break;
    case 2:
        printf("%d\n",Task_2());
        break;
    case 3:{
        scanf("%d", &NumbX);
        printf("Numbers:");
        while(count < NumbX){
        printf("\n%d",Task_3(k, Lenth(k)));
        count++;}
        printf("\n");}
        break;
    default:
        printf("Try again");
        break;
    }
    return 0;

    
}


int Task_1(int &N){
    int ak, al;
    scanf("%d", &N);
    for(int i = 0; i<=N ; i++)
        if(SimplNumb(i))
            ak = i;
    for(al = N; !SimplNumb(al) ; al++);
    return N-ak < al -N ? N-ak : al - N;
        
}


int Task_2()
{   string a;
    getline(cin,a);   
    int c=0;
    for(int i = 0; i<=9; i++)
        if(a.find('R'+i)!=a.npos && a.find('W'+i)!=a.npos && a.find('B'+i)!=a.npos) 
            c++;
    return c;
}

int Task_3(int &a, int len){
   
    int step = 1;
    int X = 0;
    int MX = a;
   
    
   
    
    for(int i = 0; i<len; i++){
        

        if(a%10 == (a/10)%10 || a%10 == X%10){
           // printf("%d",a%10);
            step=0;}
        else 
            step++;
        
        X =a;
        a/=10;
    }
    return step == 0 ? MX:NULL;
}





int Lenth(int &a){
    scanf("%d",&a);
    int  len, X = a;
    while(X){
        X/=10;
       // printf("%d\n", X);
        len++;
    }
    return len;
}



int SimplNumb(int &a) {
    if (a > 2) {
        for (int i = 2; i < a; i++)
            if (a % i == 0) return false;
        return true;
    }
    else
        return false;
}