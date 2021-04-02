/*
main.rs
Nuno Cruz
2021.03.14

utilização de vetores como tabelas com utilização dinâmica da memória

- tabelas de dimensão definida em tempo de compilação (arrays)
- tabelas de dimensão definida em tempo de execução (Vec)
 */

use std::time::{Instant};   //testar tempo
use rand::Rng;  //Biblioteca rand, módulo Rng
use rand::prelude::*;

/*
    Função algoritmo bubble sort
    @a Vec<i32> (valor passado por referência)
*/
fn bubble_sort(a: &mut Vec<i32>)
{
    let mut i:usize = 0;
    
    while i < a.len() {
        let mut j:usize = a.len() - 1;
        while j >= i+1 {
            if a[j] < a[j-1]{
                let tmp = a[j];
                a[j] = a[j-1];
                a[j-1] = tmp;
            }
            j-=1;
        } 
        i+=1;
    }
}


/*
    Função que calcula desvio padrão
    @x Vec<f64> (Valor passado por referência)
*/
fn media(x:&Vec<f64>) -> f64
{
    let mut m:f64 = 0.0; 
    let mut i:usize = 0;    //usize 64bits por default

    while i < x.len()
    {
        m += x[i];
        i+=1;
    }
    
    m = m / x.len() as f64; //convertido para f64
    m
}

/*
    Função que calcula o desvio padrão
    @x <f64>
    @return s <f64>
*/
fn desvio_p(x: &Vec<f64>) -> f64
{
    let mut s: f64 = 0.0;
    let mut i: usize = 0;
    let m = media(&x);

    while i < x.len() 
    {
        s += (x[i] - m) * (x[i] - m);
        i += 1;
    }

    s = s / x.len() as f64;
    s = s.sqrt();
    s
}


/*
    Função geradora da tabela de números aleatórios
    @n <i32>
    @return a_vec Vec<i32>
*/
fn criar_tabela_aleatoria(n:usize) -> Vec<i32>{

    // gerador de numero aleatorios utilizado na linha 22
    let mut rng = rand::thread_rng();
    let mut a_vec: Vec<i32> = vec![0; n];

    let mut i = 0;

    while i < n {
	    let a:i32 = rng.gen_range(0..100);
	    a_vec[ i ] = a;
	    i +=1;
    }
    a_vec
}


/*
    Função Main
*/
fn main() {
    const N:usize = 10;
    let now = Instant::now();   // Cria um timer 
    
    let mut a = criar_tabela_aleatoria(N);
    println!("Valores Tabela A: {:?}", a);

    let mut rng = thread_rng(); // Pertence ao metodo thread que vai fazer os numero aleatorios
    let mut b_vec:Vec<f64> = vec![0.0;N];   // VECTOR de virgual flutuante inicializado em 0.0 até N

    let t_inicial = now.elapsed().as_nanos();
    bubble_sort(&mut a);
    let t_final = now.elapsed().as_nanos();
    let t_dif = t_final - t_inicial;

    println!("Tempo de execução: {}", t_dif);   //tempo de execução em nano segundos
    //println!( "nano segundos");

    println!("Valores Tabela A: {:?}", a);
    
    
    let mut i = 0;
    while i < N {   // Gera uma nova tabela de números aleatórios (Vector)
        let y:f64 = rng.gen();

        b_vec[i] = y;   // preenche vector com números aleatorios
        
	    i +=1;
    }

    
    let t1 = now.elapsed().as_nanos() as f64;  // devolve o tempo de execução
                                               // Resultados mais concretos com floating point
    let m = media(&b_vec);
    println!("Media: {}", m);


    let s = desvio_p(&b_vec);
    println!("Devio padrão: {}", s);

    let e = s / m * 100.0;  // Calculo erro relativo
    println!("Erro relativo: {}", e);

    println!("T1: {}", t1); // Tempo de execução (nanossegundos)
}