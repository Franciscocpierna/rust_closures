/*
	Seção 05 - Capturando Referência Mutável

*/



/* 
	s05_a01 - Introdução ao Trait FnMut

	Closures podem capturar valores do seu ambiente.
	Elas podem fazer isto de 3 maneiras:
	1) Referência imutável
	2) Referência mutável
	3) Assumindo a propriedade (move)

	Nesta seção vamos considerar a captura de referências mutáveis
	Closures que fazem isto implementam o trait 'FnMut'
*/



// s05_a02 - Captura Mutável Simples
fn s05_a02() {
	println!("\ns05_a02 - Captura Mutável Simples");

	let mut zzz = 1000;
	let mut clos = |x| zzz += x;
	clos(22);
	println!("Novo valor de zzz: {}", zzz);
}



// s05_a03 - Captura Mutável e Retorna Valor
fn s05_a03() {
	println!("\ns05_a03 - Captura Mutável e Retorna Valor");

	let mut zzz = 1000;
	let mut clos = |x| { zzz += x; x*2 };
	let y = clos(22);
	println!("Retorno da closure: {}", y);
	println!("Novo valor de zzz: {}", zzz);
}	



// s05_a04 - Captura Mutável de Campo de Struct
#[derive(Debug)]
struct Pessoa {
	nome: String,
	sobrenome: String,
	nota: i32,
}

fn s05_a04() {
	println!("\ns05_a04 - Captura Mutável de Campo de Struct");

	let mut eu_mesmo = Pessoa{ nome: "Romulo".to_string(),
								sobrenome: "Oliveira".to_string(),
								nota: 5 };
	println!("eu_mesmo: {:?}", eu_mesmo);

	let mut clos = |novo:&str| eu_mesmo.sobrenome = novo.to_string();

	eu_mesmo.nota = 10;
	println!("Antes de usar clos, nome: {}", eu_mesmo.nome);
	//println!("Antes de usar clos, sobrenome: {}", eu_mesmo.sobrenome);	// Foi emprestado como mutável
	clos("Silva");
	println!("Depois de usar clos, nome: {}", eu_mesmo.nome);
	println!("Depois de usar clos, sobrenome: {}", eu_mesmo.sobrenome);
}



// s05_a05 - Exemplo do Livro no Listing 13-5
fn s05_a05() {
	println!("\ns05_a05 - Exemplo do Livro no Listing 13-5");

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}



// s05_a06 - A Closure é COPY ou MOVE ?
fn s05_a06() {
	println!("\ns05_a06 - A Closure é COPY ou MOVE ?");

	let mut zzz = 1000;
	let mut clos1 = |x| zzz += x;
	clos1(22);

	let mut clos2 = clos1;
	clos2(33);

	//clos1(44);
	// closure cannot be moved more than once as 
	// it is not `Copy` due to moving the variable `zzz` out of its environment

	println!("Valor de zzz após clos2 é {}", zzz);
}



fn main() {
	println!("\nSeção 05 - Capturando Referência Mutável");
	s05_a02();
	s05_a03();
	s05_a04();
	s05_a05();
	s05_a06();
}



