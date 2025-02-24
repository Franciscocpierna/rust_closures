/*
	Seção 11 - Exemplos Adicionais

*/



#[derive(Debug)]
struct Conta {
	cpf: String,
	saldo: f64,
}


// s11_a01 - Adaptadores São Preguiçosos
//		Adaptadores são métodos que produzem novos iteradores 
//		alterando algum aspecto de um iterador original.
fn s11_a01() {
	println!("\ns11_a01 - Adaptadores São Preguiçosos");

	let mut depositos = vec![ Conta{ cpf: "000.000.000-00".to_string(), saldo: 11.00},
						Conta{ cpf: "111.111.111-11".to_string(), saldo: 22.00},
						Conta{ cpf: "222.222.222-22".to_string(), saldo: 33.00},
						Conta{ cpf: "333.333.333-33".to_string(), saldo: 144.00},
						Conta{ cpf: "444.444.444-44".to_string(), saldo: 255.00} ];
	println!("\n!!!depositos original\n{:?}", depositos);


	let mut total = 0.00;
	depositos.iter_mut().map(|conta| {
								conta.saldo += 100.00;
								total += conta.saldo;
								});
	println!("\n!!!total sem collect(): {}", total);


	let saldos: Vec<f64> = depositos.
							iter_mut().
							map(|conta| {
							conta.saldo += 100.00;
							total += conta.saldo;
							conta.saldo }).collect();
	println!("\n!!!saldos com collect\n{:?}", saldos);
	println!("!!!total com collect(): {}", total);


	let mut total = 0.00;
	depositos.iter_mut().for_each(|conta| {
								conta.saldo += 100.00;
								total += conta.saldo;
							});
	println!("\n!!!depositos com for_each\n{:?}", depositos);
	println!("!!!total com for_each: {}", total);


	let mut total = 0.00;
	for conta in depositos.iter_mut() {
		conta.saldo += 100.00;
		total += conta.saldo;
	};
	println!("\n!!!depositos o for clássico\n{:?}", depositos);
	println!("!!!total com o for clássico: {}", total);

}




// s11_a02 - Método retain()
fn s11_a02() {
	println!("\ns11_a02 - Método retain()");

	let depositos = vec![ Conta{ cpf: "000.000.000-00".to_string(), saldo: 211.00},
						Conta{ cpf: "111.111.111-11".to_string(), saldo: 122.00},
						Conta{ cpf: "222.222.222-22".to_string(), saldo: 33.00},
						Conta{ cpf: "333.333.333-33".to_string(), saldo: 144.00},
						Conta{ cpf: "444.444.444-44".to_string(), saldo: 255.00} ];
	println!("\n!!!depositos original\n{:?}", depositos);

	
	let mut saldo_vip = 100.00;
	let mut clientes_vip: Vec<Conta> = depositos.into_iter().
										filter(|conta| conta.saldo >= saldo_vip).collect();
	println!("\n!!!clientes_vip com filter\n{:?}", clientes_vip);


	saldo_vip = 200.00;
	clientes_vip.retain(|conta| conta.saldo >= saldo_vip);
	println!("\n!!!clientes_vip com retain\n{:?}", clientes_vip);
	// retain() opera na própria memória da coleção


	saldo_vip = 250.00;
	clientes_vip.retain_mut(|conta| if conta.saldo >= saldo_vip {
														conta.saldo += 9000.00;
														true
													} else {
														false
													});
	println!("\n!!!clientes_vip com retain_mut\n{:?}", clientes_vip);

}




use std::io;
// s11_a03 - Método fold()
fn s11_a03() {
	println!("\ns11_a03 - Método fold()");

	let mut lista_palavras = Vec::new();

	loop {
		println!("[V2] Digite várias palavras ou somente enter para terminar");
		let mut linha = String::new();
		io::stdin().read_line(&mut linha).expect("Erro ao ler o teclado");
		linha = linha.trim().to_string();
		if linha.len() == 0 {
			break;
		} else {
			let palavras = linha.split_whitespace();	// Desconsidera vários espaços entre palavras
			for p in palavras {
				lista_palavras.push( p.trim().to_string() );	// cria String para colocar em 'lista_palavras'
			}
		}
	}
	println!("Foram digitadas {} palavras", lista_palavras.len());


	let mut minimo = 99999;
	let mut maximo = 0;
	let mut total = 0;
	for p in lista_palavras.iter() {
		let tam = p.chars().count();
		total += tam;
		if tam < minimo {
			minimo = tam;
		}
		if tam > maximo {
			maximo = tam;
		}
		println!("{}", p);
	}
	if lista_palavras.len() > 0 {
		println!("mínimo={}  máximo={}  médio={}", minimo, maximo, total/lista_palavras.len());
	}	


	let _tamanhos: Vec<usize> = lista_palavras.iter().map(|x| x.chars().count()).collect();

	let (minimo,maximo,total) = 
		lista_palavras.
		iter().
		map(|x| x.chars().count() ).
		fold( (99999,0,0), |acc,tam| (tam.min(acc.0),tam.max(acc.1),tam+acc.2) );

	if lista_palavras.len() > 0 {
		println!("mínimo={}  máximo={}  médio={}", minimo, maximo, total/lista_palavras.len());
	}

}




fn main() {
    println!("\nSeção 11 - Exemplos Adicionais");
	s11_a01();
	s11_a02();
	s11_a03();
}

