/*
	Seção 01 - Introdução

*/







// s01_a05 - Usos mais Frequentes das Closures
#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

#[derive(Debug)]
struct Conta {
	cpf: String,
	saldo: f64,
}

fn s01_a05() {
	println!("\ns01_a05 - Usos mais Frequentes das Closures");


	// Ordenação
	println!("\nORDENAÇÃO");

	let mut list = [
		Rectangle { width: 10, height: 1 },
		Rectangle { width: 3, height: 5 },
		Rectangle { width: 7, height: 12 },
	];

	list.sort_by_key(|r| r.width+r.height);
	println!("!!!sort pelo width+height\n{:#?}", list);

	let mut num_sort_operations = 0;

	list.sort_by_key(|r| {
		num_sort_operations += 1;
		r.width
	});
	println!("{:#?}, sorted in {num_sort_operations} operations", list);


	// Iterações
	println!("\nITERAÇÕES");

	let mut depositos = vec![ Conta{ cpf: "000.000.000-00".to_string(), saldo: 11.00},
						Conta{ cpf: "111.111.111-11".to_string(), saldo: 22.00},
						Conta{ cpf: "222.222.222-22".to_string(), saldo: 33.00},
						Conta{ cpf: "333.333.333-33".to_string(), saldo: 144.00},
						Conta{ cpf: "444.444.444-44".to_string(), saldo: 255.00} ];

	let lista_cpfs:Vec<String> = depositos.iter().map(|conta| conta.cpf.clone() ).collect();
	println!("!!!lista_cpfs\n{:?}", lista_cpfs);

	let limite_especial = 1000.00;
	let mut total = 0.00;
	
	let saldo_especial: Vec<f64> = depositos.iter_mut().
										map(|conta| {
											conta.saldo += 100.00;
											total += conta.saldo;
											conta.saldo + limite_especial }).
										collect();

	println!("!!!total: {}", total);
	println!("!!!saldo_especial\n{:?}", saldo_especial);


	// Criação de Threads
	println!("\nCRIAÇÃO DE THREADS");

	use std::thread;
	use std::time::Duration;

	thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
	}

}



fn main() {
    println!("\nSeção 01 - Introdução");
	s01_a05();
}


