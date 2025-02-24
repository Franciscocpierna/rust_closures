/*
	Seção 03 - Definindo Tipos de Dados

*/



/* 
	s03_a01 - Introdução à Definição de Parâmetros em Closures

	Mostrar como os tipos de dados dos parâmetros e do retorno 
	podem ser definidos no momento da criação da closure.

*/



// s03_a02 - Especificando os Tipos de Dados
fn s03_a02() {
	println!("\ns03_a02 - Especificando os Tipos de Dados");

	let clos = |x:i8| x + 1;
	let y = clos(20);
	//let y = clos(2222);		// Valor não cabe em i8
	println!("Retorno da closure: {}", y);
	println!("Retorno da closure: {}", clos(30));
}



// s03_a03 - Closure com Vários Parâmetros
fn s03_a03() {
	println!("\ns03_a03 - Closure com Vários Parâmetros");

	let clos = |x:i32,y:i32| x + y;
	println!("Retorno da closure: {}", clos(1000,2000));
}



// s03_a04 - Closure com Tipos de Dados Diferentes
fn s03_a04() {
	println!("\ns03_a04 - Closure com Tipos de Dados Diferentes");

	let clos = |x:i32,y:f32| x as f32 + y;
	println!("Retorno da closure: {}", clos(100,99.99));
	let r = clos(100,99.99);
	println!("Retorno da closure: {}", r);
}



// s03_a05 - Definindo o Tipo de Retorno
fn s03_a05() {
	println!("\ns03_a05 - Definindo o Tipo de Retorno");

	let _clos1 = |x:i32,y:f32| x as f32 + y;

	let clos2 = |x:i32,y:f32| -> f64 {x as f64 + y as f64};

	println!("Retorno da closure: {}", clos2(100,99.99));
	let r = clos2(100,99.99);
	println!("Retorno da closure: {}", r);
}



// s03_a06 - Exemplo do Livro no Listing 13-2
fn s03_a06() {
	println!("\ns03_a06 - Exemplo do Livro no Listing 13-2");

	let expensive_closure = |num: u32| -> u32 {
		println!("calculating slowly...");
		std::thread::sleep(std::time::Duration::from_secs(num as u64));
		num
    };

	println!("dorme 1 segundo");
	expensive_closure(1);
	println!("dorme mais 1 segundo");
	expensive_closure(1);
	println!("fim");
}



// s03_a07 - A Closure é COPY ou MOVE ?
fn s03_a07() {
	println!("\ns03_a07 - A Closure é COPY ou MOVE ?");

	let clos1 = |x,y| x + y;
	println!("Retorno da closure em clos1: {}", clos1(10,20));

	let clos2 = clos1;
	println!("Retorno da closure em clos2: {}", clos2(20,40));

	println!("Retorno novamente de clos1: {}", clos1(40,120));
	println!("Atenção!!!: Nem sempre é assim, depende do que a Closure faz");
}



// s03_a08 - Closure ou Função ?
fn funcao(x: i32) -> i32 {
	x + 1
}

fn s03_a08() {
	println!("\ns03_a08 - Closure ou Função ?");
	let clos = |x| x + 1;

	println!("Retorno da função: {}", funcao(11) );
	println!("Retorno da closure: {}", clos(11));
	println!("Atenção!!!: Nem tudo que Closure faz Função pode fazer também");
}



fn main() {
	println!("\nSeção 03 - Definindo Tipos de Dados");
	s03_a02();
	s03_a03();
	s03_a04();
	s03_a05();
	s03_a06();
	s03_a07();
	s03_a08();
}


