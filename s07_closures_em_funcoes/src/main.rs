/*
	Seção 07 - Closures na Chamada de Funções

*/



/* 
	s07_a01 - Introdução às Closures em Funções

	Mostrar como closures podem ser passadas como parâmetros para
	funções e como funções podem retornar closures.

*/



// s07_a02 - Closure Fn como Parâmetro de função
fn chama_com_cinco<F>(func: F)
	where F: Fn(i32) {
		func(5)
}

fn s07_a02() {
	println!("\ns07_a02 - Closure Fn como Parâmetro de função");

	let num1 = 100;

	let clos = |x| println!("{}", num1+x);
	chama_com_cinco( clos );
	chama_com_cinco( |x| println!("{}", num1+x) );

	let mut num2 = 200;
	chama_com_cinco( |x| println!("{}", num2+x) );
	num2 = 300;
	chama_com_cinco( |x| println!("{}", num2+x) );
}



// s07_a03 - Closure Fn como Retorno de Função
fn cria_incrementador() -> impl Fn(i32) -> i32 {
	move|x| {x + 1}			// Closure retornada
}

fn cria_somador(valor: i32) -> impl Fn(i32) -> i32 {
	// move aqui tem o papel de transferir 'valor' para a closure
	move|x| {valor + x} 				// Closure retornada
}

fn s07_a03() {
	println!("\ns07_a03 - Closure Fn como Retorno de Função");

	let clos_soma_1 = cria_incrementador();
	println!("clos_soma_1:  {}", clos_soma_1(200) );
	println!("clos_soma_1:  {}", clos_soma_1(300) );


	let clos_soma_1000 = cria_somador(1000);
	let clos_soma_8000 = cria_somador(8000);

	println!("clos_soma_1000:  {}", clos_soma_1000(33) );
	println!("clos_soma_1000:  {}", clos_soma_1000(55) );

	println!("clos_soma_1000:  {}", clos_soma_8000(33) );
	println!("clos_soma_1000:  {}", clos_soma_8000(55) );
}



// s07_a04 - Closure FnMut como Parâmetro de função
fn chama_com_oito<F>(mut func: F)
	where F: FnMut(i32) {
		func(8)
}

fn s07_a04() {
	println!("\ns07_a04 - Closure FnMut como Parâmetro de função");

	let mut num = 100;

	let clos = |x|{ num *=10; println!("{}", num+x); };

	chama_com_oito( clos );
	chama_com_oito( |x|{ num *=10; println!("{}", num+x); } );

	num = 200;
	//chama_com_oito( clos );		// 'clos' pegou 'num' emprestado lá atrás
	chama_com_oito( |x|{ num *=10; println!("{}", num+x); } );
}



// s07_a05 - Closure FnMut como Retorno de Função

fn cria_contador() -> impl FnMut() -> i32 {
	let mut i = 0;
	move|| { i += 1; i }
}

fn cria_somador_magico(valor: i32) -> impl FnMut(i32) -> i32 {
	let mut numero_magico = 10000;
	// move da propriedade de 'valor' e 'numero_magico' para a closure
	move|x| { numero_magico += 10000; x + valor + numero_magico } 		// Closure retornada
}

fn s07_a05() {
	println!("\ns07_a05 - Closure FnMut como Retorno de Função");

	let mut clos_contador = cria_contador(); // Retorna a closure
	clos_contador(); // i = 1
    clos_contador(); // i = 2
	clos_contador(); // i = 3
	clos_contador(); // i = 4
    println!("contador ficou em {}", clos_contador()); // i = 5


	let mut clos_sm_1000 = cria_somador_magico(1000);
	let mut clos_sm_8000 = cria_somador_magico(8000);

	println!("clos_sm_1000:  {}", clos_sm_1000(33) );	// Altera número mágico de clos_sm_1000
	println!("clos_sm_1000:  {}", clos_sm_1000(44) );
	println!("clos_sm_1000:  {}", clos_sm_1000(55) );

	println!("clos_sm_8000:  {}", clos_sm_8000(33) );	// Altera número mágico de clos_sm_8000
	println!("clos_sm_8000:  {}", clos_sm_8000(44) );
	println!("clos_sm_8000:  {}", clos_sm_8000(55) );

	println!("clos_sm_1000:  {}", clos_sm_1000(66) );
}



// s07_a06 - Closure FnOnce como Parâmetro de função
fn chama_com_qwerty<F>(func: F) -> String
	where F: FnOnce(&str) -> String {
		func("qwerty")
}

fn s07_a06() {
	println!("\ns07_a06 - Closure FnOnce como Parâmetro de função");
	let mut original = String::from("original");

	let clos = |x:&str| -> String { 
							original.push_str(x);
							original };

	let mut novo_string = chama_com_qwerty( clos );
	println!("Retorno de chama_com_qwerty é {}", novo_string );

	//println!("Retorno de chama_com_qwerty é {}", chama_com_qwerty( clos ) );
	//println!("Retorno de nova chama_com_qwerty é {}", chama_com_qwerty( clos ) );

	novo_string.push_str("###");
	println!("novo_string alterado é {}", novo_string );
}



// s07_a07 - Closure FnOnce como Retorno de Função
fn cria_banner(parte: &str) -> impl FnOnce(&str) -> String {
    let mut base = "MENSAGEM: ".to_string();
	base.push_str(parte);
	|txt: &str| -> String { base.push_str(txt); base }
}

fn s07_a07() {
	println!("\ns07_a07 - Closure FnOnce como Retorno de Função");

	let clos_banner_1 = cria_banner("111");
	println!("Retorno de clos_banner_1:\n   {}", clos_banner_1("cuidado com a closure") );
	//println!("Retorno de clos_banner_1:\n   {}", clos_banner_1("chama novamente") );

	let clos_banner_2 = cria_banner("222");
	println!("Retorno de clos_banner_2:\n   {}", clos_banner_2("cuidado!!!") );

	println!("Fim do s07_a07");
}



#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

// s07_a08 - Exemplo do Livro no Listing 13-7
fn s07_a08() {
	println!("\ns07_a08 - Exemplo do Livro no Listing 13-7");

	let mut list = [
		Rectangle { width: 10, height: 1 },
		Rectangle { width: 3, height: 5 },
		Rectangle { width: 7, height: 12 },
	];

	list.sort_by_key(|r| r.width);
	println!("!!!sort pelo width\n{:#?}", list);

	list.sort_by_key(|r| r.width+r.height);
	println!("!!!sort pelo width+height\n{:#?}", list);
}



// s07_a09 - Exemplo do Livro no Listing 13-9
fn s07_a09() {
	println!("\ns07_a09 - Exemplo do Livro no Listing 13-9");

	let mut list = [
		Rectangle { width: 10, height: 1 },
		Rectangle { width: 3, height: 5 },
		Rectangle { width: 8, height: 6 },
		Rectangle { width: 7, height: 12 },
	];

	let mut num_sort_operations = 0;

	list.sort_by_key(|r| {
		num_sort_operations += 1;
		r.width
	});
	println!("{:#?}, sorted in {num_sort_operations} operations", list);
}




fn main() {
    println!("\nSeção 07 - Closures na Chamada de Funções");
	s07_a02();
	s07_a03();
	s07_a04();
	s07_a05();
	s07_a06();
	s07_a07();
	s07_a08();
	s07_a09();
}


