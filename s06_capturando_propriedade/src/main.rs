/*
	Seção 06 - Capturando a Propriedade

*/



/* 
	s06_a01 - Introdução ao Trait FnOnce

	Closures podem capturar valores do seu ambiente.
	Elas podem fazer isto de 3 maneiras:
	1) Referência imutável
	2) Referência mutável
	3) Assumindo a propriedade (move)

	Nesta seção vamos considerar a captura da propriedade através de 'move'
	Closures que fazem isto implementam o trait 'FnOnce'
*/



// s06_a02 - Closure Captura a Propriedade da Variável
fn s06_a02() {
	println!("\ns06_a02 - Closure Captura a Propriedade da Variável");

	let original = String::from("OOOriginal");

	let clos = |x:&str| { 
								let mut s = original;
								s.push_str(x);
								s };

	println!("Retorno de clos é {}", clos("***alterado") );
	//println!("Valor de s: {}", s);	// Escopo de 's' é limitado

	//println!("Retorno de clos é {}", clos("####") );
	// closure cannot be invoked more than once because 
	// it moves the variable `original` out of its environment

	//println!("Valor de original: {}", original);	// 'original' foi movido pela closure
}



// s06_a03 - Closure Também Captura a Propriedade
fn s06_a03() {
	println!("\ns06_a03 - Closure Também Captura a Propriedade");

	let mut original = String::from("original");

	let clos = |x:&str| {
								original.push_str(x);
								original };

	println!("Retorno de clos é {}", clos("*alterado") );

	//println!("Retorno de clos é {}", clos("####") );
	// closure cannot be invoked more than once because 
	// it moves the variable `original` out of its environment
}



// s06_a04 - Closure é Forçada a Capturar a Propriedade
fn s06_a04() {
	println!("\ns06_a04 - Closure é Forçada a Capturar a Propriedade");

	let mut original = String::from("original");

	//let mut clos = |x:&str| original.push_str(x);

	let mut clos = move|x:&str| { original.push_str(x);
													println!("mudei original: {}", original); };

	// Com 'move' temos que:
	// borrow of moved value: `original`

	clos("*alterado");
	clos("####");
	clos("@@@@");
	//println!("Valor de original ficou: {}", original);
}



// s06_a05 - Closure é Forçada a Capturar algo com Semântica COPY
fn s06_a05() {
	println!("\ns06_a05 - Closure é Forçada a Capturar algo com Semântica COPY");

	let mut original = 1000;

	let mut clos = move|x:i32| { original += x;
												println!("original capturado: {}",original) };
	//let mut clos = |x:i32| { original += x;
	//											println!("original capturado: {}",original) };
	
	// Com 'move' temos uma nova 'original' dentro da closure
	// Mas a 'original' original permanece, pela semântica COPY dela
	// A nova 'original' é destruída no fim da closure, por isto o warning se não for usada

	clos(900);
	clos(80);
	clos(7);
	println!("Valor de original ficou: {}", original);	//Mostra a 'original' original
}



// s06_a06 - Closure é Forçada a Capturar algo com Semântica MOVE
fn s06_a06() {
	println!("\ns06_a06 - Closure é Forçada a Capturar algo com Semântica MOVE");

	let original = vec![1, 2, 3, 4, 5];

	let clos = move|x:usize| original[x] == 999;
	//let clos = |x:usize| original[x] == 999;	// Sem o move 'original' continua disponível

	println!("Retorno de clos(1) é {}", clos(1));
	println!("Retorno de clos(2) é {}", clos(2));
	println!("Retorno de clos(3) é {}", clos(3));

	//println!("Valor de original ficou: {:?}", original);
	// borrow of moved value: `original`
}



// s06_a07 - FnOnce Inferido no Caso de 'Drop'
fn s06_a07() {
	println!("\ns06_a07 - FnOnce Inferido no Caso de 'Drop'");

	let original = vec![1, 2, 3, 4, 5];
	let clos = || { println!("{:?}",original); drop(original); };

	clos();
	//clos();
}



// s06_a08 - Exemplo do Livro no Listing 13.6
use std::thread;
fn s06_a08() {
	println!("\ns06_a08 - Exemplo do Livro no Listing 13.6");

	let list = vec![1, 2, 3];
	println!("Before defining closure: {:?}", list);

    thread::spawn( move|| println!("From thread: {:?}", list))
        .join()
        .unwrap();

	//println!("After defining closure: {:?}", list);
	println!("Fim da main()");
}



fn main() {
	println!("\nSeção 06 - Capturando a Propriedade");
	s06_a02();
	s06_a03();
	s06_a04();
	s06_a05();
	s06_a06();
	s06_a07();
	s06_a08();
}

