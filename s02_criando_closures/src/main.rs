/*
	Seção 02 - Criando Closures

*/



/* 
	s02_a01 - Introdução às Closures

	Mostrar como closures podem ser criadas em Rust.

	Closures são funções anônimas, isto é, sem nome. Podem ser salvas em
	uma variável ou passadas como argumento para outras funções.
	
	É possível criar a closure em um escopo do programa e chamar a closure
	em outro para que ela seja avaliada em um contexto diferente.
	
	Ao contrário de funções, closures podem capturar valores que estão fora dela 
	mas estão no sscopo na qual ela foi criada.
*/



// s02_a02 - Closure Como Função Anônima
fn s02_a02() {
	println!("\ns02_a02 - Closure Como Função Anônima");

//	let clos                      = |x|      { x + 1 };
	let clos = |x| { x + 1 };
	let y = clos(22);
	println!("Retorno da closure: {}", y);
	println!("Retorno da closure: {}", clos(33));
}



// s02_a03 - Closure Com Vários Parâmetros
fn s02_a03() {
	println!("\ns02_a03 - Closure Com Vários Parâmetros");

	let clos = |x,y| { x + y };
	let z = clos(22,33);
	println!("Retorno da closure: {}", z);
	println!("Retorno da closure: {}", clos(100,99));
}



// s02_a04 - Closure Sem Parâmetro
fn s02_a04() {
	println!("\ns02_a04 - Closure Sem Parâmetro");

	let clos = || { println!("Sou uma closure") };
	for _i in 0..3 {
		clos();
	}
}



// s02_a05 - Sintaxe Sem Chaves
fn s02_a05() {
	println!("\ns02_a05 - Sintaxe Sem Chaves");

	let clos = |x| x + 1;
	let y = clos(100);
	println!("Retorno da closure: {}", y);
	println!("Retorno da closure: {}", clos(200));

	let clos2 = |x| { let z = x + 1; z * 100 };
	println!("Retorno da closure em clos2: {}", clos2(5));
}



// s02_a06 - Tipos De Dados São Inferidos
fn s02_a06() {
	println!("\ns02_a06 - Tipos De Dados São Inferidos");

	let clos_s = |x| println!("{}",x);
	clos_s("isto é um string literal");

	let clos_i = |x| println!("{}",x);
	clos_i(55);

	//clos_s(88);		// Tem que usar o tipo de dado inferido!!!
}



// s02_a07 - Tipo de Dado Inferido Não Pode Mudar
fn s02_a07() {
	println!("\ns02_a07 - Tipo de Dado Inferido Não Pode Mudar");

	let clos = |x| println!("{}",x);
	clos("isto é um string literal");
	//clos(5);	// Não pode mudar!!!
}



// s02_a08 - Tipo de Dado Inferido é Flexível
fn s02_a08() {
	println!("\ns02_a08 - Tipo de Dado Inferido é Flexível");

	let clos1 = |x,y| x + y;
	println!("Retorno da closure: {}", clos1(22,33));

	let clos2 = |x,y| x + y;
	println!("Retorno da closure: {}", clos2(String::from("qwerty"),"aaa") );
}



// s02_a09 - Nem Sempre Tipo Pode Ser Inferido
fn s02_a09() {
	println!("\ns02_09 - Nem Sempre Tipo Pode Ser Inferido ");

	//let clos = |x,y| x.to_string() + y;		// Não consegue inferir

	let clos = |x:i32,y| x.to_string() + y;

	println!("Retorno da closure: '{}'", clos( 33, " metros") );
}






fn main() {
    println!("\nSeção 02 - Criando Closures");
	s02_a02();
	s02_a03();
	s02_a04();
	s02_a05();
	s02_a06();
	s02_a07();
	s02_a08();
	s02_a09();
}

