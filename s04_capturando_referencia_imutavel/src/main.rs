/*
	Seção 04 - Capturando Referência Imutável

*/


/* 
	s04_a01 - Introdução ao Trait Fn

	Closures podem capturar valores do seu ambiente.
	Elas podem fazer isto de 3 maneiras:
	1) Referência imutável
	2) Referência mutável
	3) Assumindo a propriedade (move)

	Nesta seção vamos considerar a captura de referências imutáveis
	Closures que fazem isto implementam o trait 'Fn'
*/



// s04_a02 - Captura Imutável Simples
fn s04_a02() {
	println!("\ns04_a02 - Captura Imutável Simples");

	let zzz = 1000;
	let clos = |x| x + zzz;

	let y1 = clos(222);
	println!("Retorno da closure: {}", y1);
	println!("Retorno da closure: {}", clos(333));
	println!("Valor final de zzz é {}", zzz);
}



// s04_a03 - Captura Imutável é Empréstimo para a Closure
fn s04_a03() {
	println!("\ns04_a03 - Captura Imutável é Empréstimo para a Closure");

	let mut zzz = 1000;
	let clos = |x| x + zzz;

	let y1 = clos(222);
	println!("Retorno da closure: {}", y1);
	println!("Retorno da closure: {}", clos(333));
	println!("Valor final de zzz é {}", zzz);

	zzz = 8000;
//	println!("Retorno da closure: {}", clos(555));	// Empréstimo de zzz vai até aqui
	println!("Valor final de zzz é {}", zzz);
}



// s04_a04 - Variável Capturada é Anterior à Criação da Closure
fn s04_a04() {
	println!("\ns04_a04 - Variável Capturada é Anterior à Criação da Closure");

	let zzz = 1000;
	let clos = |x| x + zzz;	// Captura o primeiro zzz

	println!("Retorno da closure: {}", clos(333));
	println!("Valor de zzz é {}", zzz);

	let zzz = 8000;							// É uma nova variável zzz, outra variável
	println!("Valor de zzz é {}", zzz);

	println!("Retorno da closure: {}", clos(555));	// Empréstimo do primeiro zzz vai até aqui
	println!("Valor de zzz é {}", zzz);
}



// s04_a05 - Variável Capturada Precisa Existir Antes
fn s04_a05() {
	println!("\ns04_a05 - Variável Capturada Precisa Existir Antes");

	let zzz = 1000;
	let clos = |x| x + zzz;
	//let zzz = 1000;	// Não adianta criar a variável a ser capturada 'depois do closure'

	let y1 = clos(222);
	println!("Retorno da closure: {}", y1);
	println!("Retorno da closure: {}", clos(333));
	println!("Valor final de zzz é {}", zzz);
}



// s04_a06 - A Closure é COPY ou MOVE ?
fn s04_a06() {
	println!("\ns04_a06 - A Closure é COPY ou MOVE ?");

	let zzz = 1000;
	let clos1 = |x| x + zzz;
	println!("Retorno da closure em clos1: {}", clos1(11));

	let clos2 = clos1;
	println!("Retorno da closure em clos2: {}", clos2(22));
	println!("Valor de zzz após clos2 é {}", zzz);

	println!("Retorno novamente de clos1: {}", clos1(33));
}



// s04_a07 - Variável Capturada Pode Ser Campo de Struct
#[derive(Debug)]
struct Pessoa {
	nome: String,
	sobrenome: String,
	notas: [i32; 10],
}

fn s04_a07() {
	println!("\ns04_a07 - Variável Capturada Pode Ser Campo de Struct");

	let mut eu = Pessoa { nome:"Rômulo".to_string(),
							sobrenome: "Oliveira".to_string(),
							notas: [0,1,2,3,4,5,6,7,8,9] };

	let clos1 = |x| println!("{} {} recebeu {}",
										eu.nome, eu.sobrenome, x);

	clos1(eu.notas[1]);
	clos1(eu.notas[2]);
	eu.notas[3] = 100;
	clos1(eu.notas[3]);

	let clos2 = || println!("{:?}",eu);
	//eu.notas[3] = 100;	// Toda a estrutura foi emprestada para 'clos2'
	clos2();
}



// s04_a08 - Exemplo do Livro no Listing 13-4
fn s04_a08() {
	println!("\ns04_a08 - Exemplo do Livro no Listing 13-4");

	let list = vec![1, 2, 3];
	println!("Antes de definir a closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Antes de chamar a closure: {:?}", list);		// Empréstimo imutável pode
    only_borrows();
    println!("Após chamar a closure: {:?}", list);
}



fn main() {
	println!("\nSeção 04 - Capturando Referência Imutável");
	s04_a02();
	s04_a03();
	s04_a04();
	s04_a05();
	s04_a06();
	s04_a07();
	s04_a08();
}


