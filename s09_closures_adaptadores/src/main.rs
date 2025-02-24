/*
	Seção 09 - Closures em Adaptadores

*/

/*
	// s09_a01 - Introdução às Closures em Adaptadores

	Adaptadores

	São funções que recebem um Iterator original e retornam um novo Iterator,
	sem destruir o iterador original.
	
	Alguns exemplos:

	take()
		Recebe um valor 'n' e cria um novo iterador que mantém apenas os primeiros 'n'
		elementos do iterador original

	filter()
		Recebe como parâmetro uma closure que retorna 'bool', aplica a closure a cada elemento do
		iterador original para definir se este elemento será mantido ('true') ou não ('false') no
		novo iterador

	map()
		Recebe uma closure como parâmetro e aplica esta closure a cada elemento do iterador original,
		criando um novo iterador com os elementos alterados		

	Muitos outros existem, por exemplo:
	All: Verifica se todos os elementos satisfazem certa condição
	Any: Verifica se algum dos elementos satisfaz certa condição
	Chain: Executa um encadeamento de vários iteradores
	Chunks: Divide a coleção em uma sequência de pedaços (chunks)
	Cycle: Ao chegar ao final da coleção retorna para o início
	Fold: Percorre os elementos gerando uma certa totalização
	Windows: Utiliza uma janela deslizante de certo tamanho sobre a coleção
	Zip: Aplica dois iteradores simultaneamente sobre a mesma coleção

	Método muito usado:

	collect()
		Reune os elementos da iteração em uma nova coleção
*/


#[derive(Debug)]
struct Conta {
	cpf: String,
	saldo: f64,
}


#[derive(Debug)]
struct ContaVip {
	cpf: String,
	saldo: f64,
	limite: f64,
}


// s09_a02 - Método collect() com Intervalos
fn s09_a02() {
	println!("\ns09_a02 - Método collect() com Intervalos");

	let letras:Vec<char> = ('a'..='z').collect();
	println!("\n!!!letras: {:?}", letras);
}



// s09_a03 - Método collect() com Iteradores
fn s09_a03() {
	println!("\ns09_a03 - Método collect() com Iteradores");

	let depositos = vec![ Conta{ cpf: "000.000.000-00".to_string(), saldo: 0.00},
						Conta{ cpf: "111.111.111-11".to_string(), saldo: 11.00},
						Conta{ cpf: "222.222.222-22".to_string(), saldo: 22.00},
						Conta{ cpf: "333.333.333-33".to_string(), saldo: 133.00},
						Conta{ cpf: "444.444.444-44".to_string(), saldo: 244.00} ];

	println!("\n!!!depositos[0]: {} {}", &depositos[0].cpf, depositos[0].saldo);
	println!("\n!!!depositos: {:?}",depositos);

	let mut saida1 = Vec::new();
	for val in depositos.into_iter() {
		saida1.push(val);
	}
	println!("\n!!!saida1: {:?}",saida1);

	let saida2:Vec<Conta> = saida1.into_iter().collect();
	println!("\n!!!saida2: {:?}",saida2);

	//println!("{:?}",depositos);	// borrow of moved value: `depositos`
	//println!("{:?}",saida1);		// borrow of moved value: `saida1`
	// into_iter() Cria um iterador que move para fora cada elemento do vector
}




// s09_a04 - Adaptador take()
fn s09_a04() {
	println!("\ns09_a04 - Adaptador take()");

	let depositos = vec![ Conta{ cpf: "000.000.000-00".to_string(), saldo: 11.00},
						Conta{ cpf: "111.111.111-11".to_string(), saldo: 22.00},
						Conta{ cpf: "222.222.222-22".to_string(), saldo: 33.00},
						Conta{ cpf: "333.333.333-33".to_string(), saldo: 144.00},
						Conta{ cpf: "444.444.444-44".to_string(), saldo: 255.00} ];

	println!("\n!!!depositos: {:?}",depositos);

	let saida:Vec<Conta> = depositos.into_iter().take(3).collect();
	println!("\n!!!saida: {:?}",saida);

	//println!("{:?}",depositos);		// borrow of moved value: `depositos`
}



// s09_a05 - Adaptador filter()
fn s09_a05() {
	println!("\ns09_a05 - Adaptador filter()");

	let depositos = vec![ Conta{ cpf: "000.000.000-00".to_string(), saldo: 11.00},
						Conta{ cpf: "111.111.111-11".to_string(), saldo: 22.00},
						Conta{ cpf: "222.222.222-22".to_string(), saldo: 33.00},
						Conta{ cpf: "333.333.333-33".to_string(), saldo: 144.00},
						Conta{ cpf: "444.444.444-44".to_string(), saldo: 255.00} ];

	println!("\n!!!depositos\n{:?}", depositos);

	let saldo_vip = 100.00;
	depositos.iter().
				filter(|conta| conta.saldo >= saldo_vip).
						for_each( |conta| println!("cliente vip {} {}", &conta.cpf, conta.saldo) );

	let clientes_vip: Vec<Conta> = depositos.into_iter().filter(|conta| conta.saldo >= saldo_vip).collect();
	println!("\n!!!clientes_vip\n{:?}", clientes_vip);

	//println!("\n!!!depositos\n{:?}", depositos);		// borrow of moved value: `depositos`
}



// s09_a06 - Adaptador map()
fn s09_a06() {
	println!("\ns09_a06 - Adaptador map()");

	let mut depositos = vec![ Conta{ cpf: "000.000.000-00".to_string(), saldo: 11.00},
						Conta{ cpf: "111.111.111-11".to_string(), saldo: 22.00},
						Conta{ cpf: "222.222.222-22".to_string(), saldo: 33.00},
						Conta{ cpf: "333.333.333-33".to_string(), saldo: 144.00},
						Conta{ cpf: "444.444.444-44".to_string(), saldo: 255.00} ];

	println!("\n!!!depositos\n{:?}", depositos);

	let lista_cpfs:Vec<String> = depositos.iter().map(|conta| conta.cpf.clone() ).collect();
	println!("\n!!!lista_cpfs\n{:?}", lista_cpfs);

	let limite_especial = 1000.00;
	let mut total = 0.00;
	let saldo_especial: Vec<f64> = depositos.
									iter_mut().
									map(|conta| {
										 conta.saldo += 100.00;
										 total += conta.saldo;
										 conta.saldo + limite_especial }).collect();

	println!("\n!!!total: {}", total);
	println!("\n!!!depositos\n{:?}", depositos);
	println!("\n!!!saldo_especial\n{:?}", saldo_especial);
}



// s09_a07 - Vários Adaptadores
use std::collections::HashMap;
fn s09_a07() {
	println!("\ns09_a07 - Vários Adaptadores");

	let depositos = vec![ Conta{ cpf: "000.000.000-00".to_string(), saldo: 11.00},
						Conta{ cpf: "111.111.111-11".to_string(), saldo: 22.00},
						Conta{ cpf: "222.222.222-22".to_string(), saldo: 33.00},
						Conta{ cpf: "333.333.333-33".to_string(), saldo: 144.00},
						Conta{ cpf: "444.444.444-44".to_string(), saldo: 255.00} ];

	let saldo_vip = 100.00;

	let contas_vip: HashMap<_,_> = depositos.
			iter().
			filter(|conta| conta.saldo >= saldo_vip).
			map(|conta| ( conta.cpf.clone(),
								 ContaVip{ cpf: conta.cpf.clone(), saldo: conta.saldo, limite: 100.00 } ) ).
			collect();

	println!("\n!!!contas_vip\n{:?}", contas_vip);

	match contas_vip.get( "333.333.333-33" ) {
		Some(conta) => println!("Pesquisa no hashmap: {} {}  {}", conta.cpf, conta.saldo, conta.limite),
		None => println!("CPF não cadastrado.")
    }

}



// s09_a08 - Exemplo do Livro no Listing 13-16
#[derive(PartialEq, Debug)]
struct Shoe {
	size: u32,
	style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
	shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn s09_a08() {
	println!("\ns09_a08 - Exemplo do Livro no Listing 13-16");

	let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

	println!("\n!!!shoes: {:?}", shoes);

    let in_my_size = shoes_in_size(shoes, 10);

	println!("\n!!!in_my_size: {:?}", in_my_size);

	//println!("\n!!!shoes: {:?}", shoes);			// borrow of moved value: `shoes`
}



fn main() {
    println!("\nSeção 09 - Closures em Adaptadores");
	s09_a02();
	s09_a03();
	s09_a04();
	s09_a05();
	s09_a06();
	s09_a07();
	s09_a08();
}



